# Validation and QA

Glyphflick should push as much validation as possible into code and leave only genuinely host-dependent behavior for manual QA.

## Validation layers

### 1. Compile/static validation

Every implementation change should aim to pass:

- `cargo check`;
- `cargo test`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- formatting check.

CI enforces formatting, locked dependency resolution, check, tests, clippy, and a separate runtime-dependency/binary-size budget.

### 2. Unit tests

Primary candidates:

- query normalization;
- exact/prefix/substring ranking;
- alias matching;
- stable tie-breaking;
- Unicode sequence preservation;
- keyboard cursor movement;
- row/column boundary movement;
- empty result behavior;
- cancellation path;
- clipboard success/failure state transitions.

### 3. Fake-backend application tests

The clipboard backend should be injectable/fakeable enough to prove:

- selected text is exact;
- clipboard success triggers exit intent;
- clipboard failure does not trigger success exit;
- cancel never writes clipboard;
- repeated selection after a failure can recover.

### 4. Real Wayland QA

Only the host can validate some behavior.

Test matrix:

#### Launch/focus

- picker appears on invocation;
- search field is focused;
- first keystroke is not lost;
- repeated launch after prior exit works.

#### Pointer flow

- click correct glyph;
- UI disappears;
- paste into prior application succeeds.

#### Keyboard flow

- type query;
- move selection;
- Enter;
- UI disappears;
- paste succeeds.

#### Cancel

- establish known clipboard contents;
- launch Glyphflick;
- type/navigate;
- Escape;
- original clipboard contents remain available.

#### Clipboard persistence

- select a glyph;
- verify visible Glyphflick process is gone;
- paste more than once where compositor behavior allows;
- verify exact Unicode sequence.

#### Unicode edge cases

Test at least:

- simple single-codepoint emoji;
- variation-selector sequence;
- skin-tone modifier;
- ZWJ family/profession sequence;
- flag/regional-indicator sequence;
- keycap sequence.

#### Font/rendering

- glyphs do not render as empty boxes for common corpus entries;
- ZWJ sequences appear as expected where host fonts support them;
- tooltip/name remains usable when rendering is imperfect.

#### Repetition

Perform rapid repeated cycles:

`launch -> select -> paste -> launch -> select -> paste`

Look for:

- stale query;
- orphan windows;
- lost focus;
- clipboard races;
- accumulating helper processes.

## QA discipline

Manual QA is not a hazing ritual and should not be requested before repository-side checks are exhausted.

When user QA is required, provide:

- exactly what changed;
- the smallest concrete behavior to test;
- expected result;
- what diagnostic information is useful only if it fails.

Do not dump unrelated setup instructions into a QA request.

## Regression rule

Any bug found in host QA that can be represented deterministically in code should gain an automated regression test before being considered fully repaired.


## One-command target probe

Run from the repository root:

```bash
bash scripts/target-probe.sh
```

The current target probe is software-runtime-only. It:

1. runs the production/no-grid-text latency probe on the live Wayland session;
2. verifies target Noto Color Emoji coverage and representative complex sequence rasterization;
3. invokes Glyphflick's real `WlCopyClipboard` backend;
4. verifies clipboard persistence with `wl-paste`;
5. restores the prior text clipboard when possible.

The production latency sub-probe exits after the first successful **software present**, not an EGL/GL swap. OpenGL/EGL comparison machinery is historical and is not part of current QA.

The normal release build remains persistent until selection/cancel; benchmark auto-exit behavior exists only in builds compiled with the `timing` feature.

## MVP host acceptance

On 2026-10-02 the principal ran the real release GUI on the target EndeavourOS/Hyprland host and explicitly accepted the MVP and its release-build responsiveness.

That acceptance closes the MVP gate. Extended host-matrix checks remain useful maintenance evidence but are no longer prerequisites pretending the product has not shipped.
