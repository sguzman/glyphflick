# UX Specification

## UX principle

Glyphflick should feel less like opening an application and more like temporarily revealing a system control.

The interface exists only long enough to make one selection.

## Interaction state machine

```text
LAUNCH
  |
  v
READY/SEARCHING
  |   |   Escape / focus-cancel policy
  |   v
  |  CANCEL -> EXIT
  |
  +-- select glyph
        |
        v
      COPY
      /  \
 success  failure
   |        |
   v        v
 EXIT     ERROR -> READY/SEARCHING
```

The common path must be short:

`LAUNCH -> READY -> COPY -> EXIT`

## Initial focus

On launch:

- the search field is active;
- typing immediately changes the query;
- no click is needed before searching;
- the result grid is already usable with the pointer.

## Layout

MVP surface:

- compact window;
- search field at top;
- glyph result grid below;
- minimal chrome;
- no permanent sidebar;
- no toolbar full of modes;
- no status bar unless an error requires one.

A category UI may be added later if it demonstrably improves browsing.

## Window behavior

Desired behavior:

- transient;
- compact;
- visually centered or compositor-positioned;
- not resizable for MVP unless resizing proves useful;
- no workflow-obstructing decorations;
- no lingering after commit/cancel.

Exact compositor placement is integration policy, not application ownership.

## Search field

Properties:

- focused on launch;
- clear placeholder;
- query retained only for the lifetime of the invocation in MVP;
- Ctrl+A and standard editing behavior should work naturally;
- clearing the query restores default results.

Potential enhancement:

- when keyboard navigation moves into the grid, typing ordinary text should continue to affect search rather than becoming an opaque navigation mode.

## Grid

Each visible item primarily displays the glyph.

Secondary information:

- accessible name available by tooltip or equivalent;
- selection/highlight state for keyboard navigation;
- no always-visible labels under every cell in MVP unless testing shows visual ambiguity is too high.

## Pointer behavior

- Hover: reveal name/metadata without blocking.
- Primary click: commit immediately.
- No double click.
- No separate Copy button.
- No context menu in MVP.

## Keyboard behavior

Required eventual MVP behavior:

- typing: search;
- Arrow keys: move active selection;
- Enter: commit active selection;
- Escape: cancel;
- Home/End or PageUp/PageDown: optional depending on grid implementation.

Keyboard movement should be spatially consistent with the rendered grid.

## Commit feedback

The default success feedback is disappearance.

Do not delay exit to show:

- "Copied!" toast;
- animation;
- checkmark;
- modal acknowledgement.

If later telemetry/QA proves users need feedback, it should not extend the critical path.

## Failure feedback

Clipboard failure is different because disappearing would lie.

In that case:

- remain visible;
- show a compact error;
- keep the selected/query context;
- allow retry through reselection;
- Escape still cancels.

## Theme

MVP should respect the native egui/system-dark/light direction where practical.

Visual goals:

- high contrast enough to scan;
- subtle selection state;
- no decorative branding that makes the picker visually heavy;
- emoji remain the visual focus.

## Latency perception

Perceived speed can be harmed even when measured CPU time is small.

Avoid:

- splash screens;
- fade-in delays;
- delayed focus;
- layout jumps;
- loading spinners for local data;
- animated exit;
- first-frame empty states.

The first useful frame should already contain the search field and results.

## Multi-monitor behavior

Glyphflick itself should not encode monitor assumptions.

The compositor/windowing environment determines placement unless portable APIs make sensible active-monitor placement reliable without compositor coupling.

This remains a host integration concern until proven otherwise.

## Repeated invocation

Launching, selecting, and launching again should be clean.

No stale query.
No stale error.
No hidden singleton window.
No orphan visible process.

## UX acceptance scenarios

### Mouse-first

1. Invoke.
2. Recognize desired glyph in defaults.
3. Click.
4. Window vanishes.
5. Paste succeeds.

### Search + mouse

1. Invoke.
2. Type `skull`.
3. Click skull.
4. Window vanishes.
5. Paste succeeds.

### Keyboard-only

1. Invoke.
2. Type query.
3. Navigate to result.
4. Press Enter.
5. Window vanishes.
6. Paste succeeds.

### Cancel

1. Invoke.
2. Type anything.
3. Press Escape.
4. Window vanishes.
5. Clipboard remains unchanged.

### Clipboard failure

1. Invoke.
2. Select glyph.
3. Clipboard backend fails.
4. Window remains.
5. Error is visible.
6. User may retry or cancel.
