#!/usr/bin/env bash
set -euo pipefail

cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."

runs="${GLYPHFLICK_PROBE_RUNS:-7}"
if ! [[ "$runs" =~ ^[1-9][0-9]*$ ]]; then
  echo "GLYPHFLICK_PROBE_RUNS must be a positive integer" >&2
  exit 2
fi

if [[ -z "${WAYLAND_DISPLAY:-}" || -z "${XDG_RUNTIME_DIR:-}" ]]; then
  echo "glyphflick target probe requires a live Wayland session" >&2
  exit 2
fi

for command in cargo wl-copy wl-paste; do
  if ! command -v "$command" >/dev/null 2>&1; then
    echo "required command not found: $command" >&2
    exit 2
  fi
done

emoji_font=""
for candidate in   /usr/share/fonts/noto/NotoColorEmoji.ttf   /usr/share/fonts/truetype/noto/NotoColorEmoji.ttf   /usr/share/fonts/TTF/NotoColorEmoji.ttf   /usr/local/share/fonts/NotoColorEmoji.ttf
do
  if [[ -r "$candidate" ]]; then
    emoji_font="$candidate"
    break
  fi
done

if [[ -z "$emoji_font" ]]; then
  echo "NotoColorEmoji.ttf was not found on Glyphflick's fixed lookup paths" >&2
  exit 2
fi

mkdir -p target
raw_log="$(mktemp)"
old_clip="$(mktemp)"
cleanup() {
  rm -f "$raw_log" "$old_clip"
}
trap cleanup EXIT

echo "glyphflick target probe"
echo "wayland_display=$WAYLAND_DISPLAY"
echo "emoji_font=$emoji_font"
echo "runs=$runs"
echo

echo "[1/4] building timing probe"
cargo build --release --locked --features timing

echo
echo "[2/4] measuring real Wayland startup"
for ((run = 1; run <= runs; run++)); do
  printf -- "--- run %d ---\n" "$run" >>"$raw_log"
  GLYPHFLICK_EXIT_AFTER_FIRST_SWAP=1 ./target/release/glyphflick 2>>"$raw_log"
done

if grep -q 'system_emoji_mapped=false' "$raw_log"; then
  echo "timed launch failed to map Noto Color Emoji" >&2
  cat "$raw_log" >&2
  exit 1
fi

if grep -q 'swap_interval_dont_wait=false' "$raw_log"; then
  echo "warning: EGL rejected SwapInterval::DontWait on this host" >&2
fi

summary_file="$(pwd)/target/glyphflick-target-probe.txt"
: >"$summary_file"

summarize_metric() {
  local key="$1"
  local values
  values="$(sed -nE "s/.*${key}=([0-9]+).*/\1/p" "$raw_log")"

  local count
  count="$(wc -l <<<"$values")"
  if [[ "$count" -ne "$runs" ]]; then
    echo "expected $runs values for $key, got $count" >&2
    cat "$raw_log" >&2
    exit 1
  fi

  local first min median p95 max avg
  first="$(head -n1 <<<"$values")"
  min="$(sort -n <<<"$values" | head -n1)"
  max="$(sort -n <<<"$values" | tail -n1)"
  median="$(sort -n <<<"$values" | awk '{ a[NR]=$1 } END { print a[int((NR + 1) / 2)] }')"
  p95="$(sort -n <<<"$values" | awk '{ a[NR]=$1 } END { i=int((95 * NR + 99) / 100); if (i < 1) i=1; if (i > NR) i=NR; print a[i] }')"
  avg="$(awk '{ sum += $1 } END { printf "%.0f", sum / NR }' <<<"$values")"

  printf '%-36s first=%8sus  min=%8sus  p50=%8sus  avg=%8sus  p95=%8sus  max=%8sus\n'     "$key" "$first" "$min" "$median" "$avg" "$p95" "$max" | tee -a "$summary_file"
}

{
  echo "Glyphflick target probe"
  echo "Wayland display: $WAYLAND_DISPLAY"
  echo "Emoji font: $emoji_font"
  echo "Measured launches: $runs"
  echo
} >>"$summary_file"

summarize_metric wayland_egl_gl_init_us
summarize_metric egui_runtime_init_us
summarize_metric font_init_us
summarize_metric corpus_init_us
summarize_metric startup_to_first_ui_us
summarize_metric startup_to_first_swap_complete_us

{
  echo
  echo "Raw timing log:"
  cat "$raw_log"
} >>"$summary_file"

echo
echo "[3/4] checking target emoji coverage"
coverage_output="$(cargo test --locked system_color_font_coverage_report -- --nocapture 2>&1)"
printf '%s\n' "$coverage_output"
{
  echo
  echo "Target emoji coverage:"
  printf '%s\n' "$coverage_output" | grep 'glyphflick system emoji' || true
} >>"$summary_file"

echo
echo "[4/4] checking real clipboard persistence"
had_old_clipboard=false
if wl-paste --no-newline >"$old_clip" 2>/dev/null; then
  had_old_clipboard=true
fi

restore_clipboard() {
  if [[ "$had_old_clipboard" == true ]]; then
    wl-copy --type 'text/plain;charset=utf-8' <"$old_clip" 2>/dev/null || true
  else
    wl-copy --clear 2>/dev/null || true
  fi
}

qa_text="glyphflick-wayland-clipboard-qa"
if ! cargo test --locked real_wayland_clipboard_establishes_selection -- --ignored --nocapture; then
  restore_clipboard
  exit 1
fi

actual="$(wl-paste --no-newline 2>/dev/null || true)"
if [[ "$actual" != "$qa_text" ]]; then
  restore_clipboard
  echo "clipboard persistence check failed: expected '$qa_text', got '$actual'" >&2
  exit 1
fi

restore_clipboard

echo "clipboard_persistence=ok" | tee -a "$summary_file"
echo
echo "probe_report=$summary_file"
