#!/usr/bin/env bash
set -euo pipefail

cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."

runs="${GLYPHFLICK_GRAPHICS_PROBE_RUNS:-7}"
if ! [[ "$runs" =~ ^[1-9][0-9]*$ ]]; then
  echo "GLYPHFLICK_GRAPHICS_PROBE_RUNS must be a positive integer" >&2
  exit 2
fi

if [[ -z "${WAYLAND_DISPLAY:-}" || -z "${XDG_RUNTIME_DIR:-}" ]]; then
  echo "glyphflick graphics probe requires a live Wayland session" >&2
  exit 2
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "required command not found: cargo" >&2
  exit 2
fi

mkdir -p target
report="target/glyphflick-graphics-probe.txt"
: >"$report"

echo "glyphflick graphics probe"
echo "wayland_display=$WAYLAND_DISPLAY"
echo "runs_per_mode=$runs"
echo

echo "[1/3] building timing binary"
cargo build --release --locked --features timing

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

  printf '%-7s %-36s first=%8sus  min=%8sus  p50=%8sus  avg=%8sus  p95=%8sus  max=%8sus\n' \
    "$label" "$key" "$first" "$min" "$median" "$avg" "$p95" "$max" | tee -a "$report"
}

run_mode() {
  local label="$1"
  local force_gles="$2"
  local alpha_zero="$3"
  local log
  log="$(mktemp)"

  echo
  echo "[$label] measuring $runs launches"

  for ((run = 1; run <= runs; run++)); do
    printf -- "--- run %d ---\n" "$run" >>"$log"
    if [[ "$force_gles" == true && "$alpha_zero" == true ]]; then
      GLYPHFLICK_EXIT_AFTER_FIRST_SWAP=1 GLYPHFLICK_FORCE_GLES=1 GLYPHFLICK_ALPHA_ZERO=1 \
        ./target/release/glyphflick 2>>"$log"
    elif [[ "$force_gles" == true ]]; then
      GLYPHFLICK_EXIT_AFTER_FIRST_SWAP=1 GLYPHFLICK_FORCE_GLES=1 \
        ./target/release/glyphflick 2>>"$log"
    elif [[ "$alpha_zero" == true ]]; then
      GLYPHFLICK_EXIT_AFTER_FIRST_SWAP=1 GLYPHFLICK_ALPHA_ZERO=1 \
        ./target/release/glyphflick 2>>"$log"
    else
      GLYPHFLICK_EXIT_AFTER_FIRST_SWAP=1 \
        ./target/release/glyphflick 2>>"$log"
    fi
  done

  local api config
  api="$(sed -nE 's/.*context_api=(.*)/\1/p' "$log" | sort -u | paste -sd ',' -)"
  config="$(sed -nE 's/.*(gl_config_alpha=.*)/\1/p' "$log" | sort -u | paste -sd ',' -)"
  echo "$label context_api=$api" | tee -a "$report"
  echo "$label $config" | tee -a "$report"

  if grep -q 'system_emoji_mapped=false' "$log"; then
    echo "$label failed to map Noto Color Emoji" >&2
    cat "$log" >&2
    rm -f "$log"
    exit 1
  fi

  for key in \
    event_loop_init_us \
    startup_to_resumed_us \
    egl_display_config_window_us \
    gl_context_create_us \
    egl_surface_create_us \
    gl_make_current_us \
    glow_loader_us \
    wayland_egl_gl_init_us \
    egui_runtime_init_us \
    startup_to_first_ui_us \
    first_egui_run_us \
    first_gl_paint_us \
    first_swap_call_us \
    startup_to_first_swap_complete_us
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
  echo "Glyphflick graphics probe"
  echo "Wayland display: $WAYLAND_DISPLAY"
  echo "Launches per mode: $runs"
  echo
} >>"$report"

echo "[2/4] default context"
run_mode default false false

echo
echo "[3/4] forced GLES context"
run_mode gles true false

echo
echo "[4/4] opaque alpha-zero config"
run_mode alpha0 false true

echo
echo "probe_report=$report"
