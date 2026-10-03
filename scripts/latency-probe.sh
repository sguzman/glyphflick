#!/usr/bin/env bash
set -euo pipefail

cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."

runs="${GLYPHFLICK_LATENCY_PROBE_RUNS:-7}"
if ! [[ "$runs" =~ ^[1-9][0-9]*$ ]]; then
  echo "GLYPHFLICK_LATENCY_PROBE_RUNS must be a positive integer" >&2
  exit 2
fi

if [[ -z "${WAYLAND_DISPLAY:-}" || -z "${XDG_RUNTIME_DIR:-}" ]]; then
  echo "glyphflick latency probe requires a live Wayland session" >&2
  exit 2
fi

for command in cargo rustc git; do
  if ! command -v "$command" >/dev/null 2>&1; then
    echo "required command not found: $command" >&2
    exit 2
  fi
done

mkdir -p target
report="$(pwd)/target/glyphflick-latency-probe.txt"
: >"$report"

echo "glyphflick latency probe"
echo "wayland_display=$WAYLAND_DISPLAY"
echo "runs_per_mode=$runs"
echo "git_commit=$(git rev-parse --short=12 HEAD)"
echo "rustc=$(rustc --version)"
echo

echo "[1/3] building measured binary"
cargo build --release --locked --features timing --bin glyphflick

summarize_metric() {
  local log="$1"
  local label="$2"
  local key="$3"
  local values
  values="$(sed -nE "s/.*${key}=([0-9]+).*/\1/p" "$log")"

  local count
  count="$(wc -l <<<"$values")"
  if [[ "$count" -ne "$runs" ]]; then
    echo "expected $runs values for $label/$key, got $count" >&2
    cat "$log" >&2
    exit 1
  fi

  local first min median p95 max avg
  first="$(head -n1 <<<"$values")"
  min="$(sort -n <<<"$values" | head -n1)"
  max="$(sort -n <<<"$values" | tail -n1)"
  median="$(sort -n <<<"$values" | awk '{ a[NR]=$1 } END { print a[int((NR + 1) / 2)] }')"
  p95="$(sort -n <<<"$values" | awk '{ a[NR]=$1 } END { i=int((95 * NR + 99) / 100); if (i < 1) i=1; if (i > NR) i=NR; print a[i] }')"
  avg="$(awk '{ sum += $1 } END { printf "%.0f", sum / NR }' <<<"$values")"

  printf '%-13s %-38s first=%8sus  min=%8sus  p50=%8sus  avg=%8sus  p95=%8sus  max=%8sus\n' \
    "$label" "$key" "$first" "$min" "$median" "$avg" "$p95" "$max" | tee -a "$report"
}

run_mode() {
  local label="$1"
  local suppress_text="$2"
  local log
  log="$(mktemp)"

  echo
  echo "[$label] measuring $runs launches"

  for ((run = 1; run <= runs; run++)); do
    printf -- "--- run %d ---\n" "$run" >>"$log"
    if [[ "$suppress_text" == "1" ]]; then
      GLYPHFLICK_EXIT_AFTER_FIRST_PRESENT=1 \
        GLYPHFLICK_SUPPRESS_GRID_TEXT=1 \
        ./target/release/glyphflick 2>>"$log"
    else
      GLYPHFLICK_EXIT_AFTER_FIRST_PRESENT=1 \
        ./target/release/glyphflick 2>>"$log"
    fi
  done

  for key in \
    event_loop_init_us \
    context_init_us \
    startup_to_resumed_us \
    window_surface_init_us \
    egui_winit_init_us \
    egui_app_init_us \
    app_ui_us \
    ui_search_us \
    ui_grid_us \
    egui_run_us \
    tessellate_us \
    texture_update_us \
    software_raster_us \
    raster_fast_quads \
    raster_fallback_triangles \
    present_call_us \
    startup_to_first_present_us
  do
    summarize_metric "$log" "$label" "$key"
  done

  {
    echo
    echo "Raw $label log:"
    cat "$log"
    echo
  } >>"$report"

  rm -f "$log"
}

{
  echo "Glyphflick latency probe"
  echo "Wayland display: $WAYLAND_DISPLAY"
  echo "Launches per mode: $runs"
  echo "Git commit: $(git rev-parse HEAD)"
  echo "Rust: $(rustc --version)"
  echo
} >>"$report"

echo "[2/3] production software runtime"
run_mode production 0

echo
echo "[3/3] no-grid-text diagnostic"
run_mode no-grid-text 1

echo
echo "probe_report=$report"
