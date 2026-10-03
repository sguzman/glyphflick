#!/usr/bin/env bash
set -euo pipefail

cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."

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
for candidate in \
  /usr/share/fonts/noto/NotoColorEmoji.ttf \
  /usr/share/fonts/truetype/noto/NotoColorEmoji.ttf \
  /usr/share/fonts/TTF/NotoColorEmoji.ttf \
  /usr/local/share/fonts/NotoColorEmoji.ttf
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
summary_file="$(pwd)/target/glyphflick-target-probe.txt"
old_clip="$(mktemp)"

cleanup() {
  rm -f "$old_clip"
}
trap cleanup EXIT

echo "glyphflick target probe"
echo "wayland_display=$WAYLAND_DISPLAY"
echo "emoji_font=$emoji_font"
echo "git_commit=$(git rev-parse --short=12 HEAD)"
echo

echo "[1/3] software-runtime latency"
bash scripts/latency-probe.sh
cp target/glyphflick-latency-probe.txt "$summary_file"

echo
echo "[2/3] target emoji coverage"
coverage_output="$(cargo test --locked system_color_font_coverage_report -- --nocapture 2>&1)"
printf '%s\n' "$coverage_output"
{
  echo
  echo "Target emoji coverage:"
  printf '%s\n' "$coverage_output" | grep 'glyphflick system emoji' || true
} >>"$summary_file"

echo
echo "[3/3] real clipboard persistence"
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
