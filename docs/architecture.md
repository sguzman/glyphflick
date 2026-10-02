# Architecture

## Architectural goal

Keep Glyphflick small enough that its architecture explains the whole application rather than hiding it.

The MVP is a single Rust binary with a deliberately narrow Wayland/EGL runtime around egui.

## Module boundaries

```text
main
 |
 +-- runtime      Wayland/winit + EGL/glutin + egui_glow event/render loop
 +-- app          egui state + interaction state machine
 +-- corpus       glyph records + bundled dataset adapter
 +-- search       normalization + ranking/filtering
 +-- navigation   allocation-free result cursor movement
 +-- clipboard    copy contract + Wayland implementation
 +-- perf         compile-time optional timing probes
```

The boundaries are intentional: UI/domain code should not depend on window-system or OpenGL setup.

## Runtime architecture

Glyphflick does **not** use eframe.

The runtime is intentionally assembled from the narrower layers underneath it:

- `winit` for a Wayland-only event loop/window;
- `glutin` + `glutin-winit` for EGL/OpenGL context creation;
- `egui_glow` for egui input translation and Glow rendering;
- `egui` for the immediate-mode UI.

Runtime features are trimmed aggressively:

- Wayland enabled;
- X11 disabled;
- EGL enabled;
- GLX disabled;
- wgpu absent;
- egui-winit OS clipboard feature absent;
- egui link opening absent;
- no eframe persistence/application framework.

The event loop uses `ControlFlow::Wait`, so Glyphflick does not continuously poll while idle.

Swap interval is requested as `DontWait`; the picker has no reason to wait for a display refresh boundary before presenting a ready frame.

On Wayland, the window is expected to become visible when its first buffer is presented, avoiding a separate splash/blank-frame path.

## Data model

A core glyph record conceptually contains:

```text
Glyph {
    text: Unicode sequence
    name: canonical human-readable name
    aliases: zero or more alternate names/shortcodes
    keywords: zero or more search terms
    group: optional category
}
```

The UI does not depend directly on a third-party emoji crate's public types. External corpus types terminate inside the corpus adapter.

## Corpus strategy

MVP corpus requirements:

- local;
- deterministic;
- compile-time/bundled metadata;
- no network;
- preserves multi-codepoint sequences exactly;
- canonical names and useful aliases;
- no startup filesystem discovery.

The current adapter stores static emoji references and expands skin-tone variants into a compact index once per invocation. That construction cost is instrumented because even small startup work must earn its place.

## Search architecture

Search is a deterministic component:

```text
(query, corpus) -> ranked result indices
```

Current behavior is deliberately simple:

- linear scan over a small corpus;
- reusable result/bucket vectors;
- no per-query sort;
- no fuzzy-search dependency;
- no startup-built search index;
- ASCII case-insensitive matching for the English canonical names/shortcodes.

This stays simple until measurements show search itself is material.

## UI architecture

egui owns rendering and immediate-mode interaction, not process/window lifecycle.

The application state holds:

- query;
- active result;
- ranked result indices;
- transient error state;
- commit/cancel intent;
- visible-grid bookkeeping.

The app requests exit with a boolean; the runtime owns actual event-loop termination. This keeps the app testable without eframe or a Wayland session.

## Keyboard navigation

Navigation is a small index state machine rather than an additional widget tree.

It performs:

- constant-space row/column movement;
- no allocations;
- no animation;
- direct scroll offset jumps when the selected row leaves the visible range.

## Clipboard abstraction

The clipboard boundary is intentionally separate from egui's OS clipboard integration.

Conceptual interface:

```text
Clipboard::copy(text) -> success/failure
```

The initial backend invokes `wl-copy` **only after a glyph is committed**.

This is important for two reasons:

1. clipboard ownership can survive the visible picker process;
2. no clipboard helper or clipboard library initialization is paid on Glyphflick's launch path.

The direct egui runtime intentionally leaves egui-winit's OS clipboard feature disabled. Standard search-field paste can later be implemented on demand rather than paying clipboard initialization on every invocation.

## Process model

```text
external keybinding/launcher
    |
    v
Glyphflick process
    |
    +-- Wayland/EGL init
    +-- first egui frame
    +-- search / navigate
    +-- commit
    +-- wl-copy establishes selection
    |
    v
event loop exits immediately
```

No resident Glyphflick daemon is required.

If `wl-copy` leaves a narrowly scoped clipboard-provider process alive, that is clipboard ownership, not a Glyphflick service.

## Window/compositor boundary

Glyphflick exposes:

- Wayland application ID: `glyphflick`;
- title: `Glyphflick`;
- fixed transient-picker dimensions.

It does not:

- edit Hyprland config;
- install keybindings;
- assume a particular modifier;
- embed Hyprland IPC in core behavior.

## Dependency policy

Every runtime dependency must justify startup cost.

Avoid:

- general async runtimes;
- databases;
- serialization/config frameworks before needed;
- logging stacks;
- plugin systems;
- runtime font discovery without measurement;
- convenience framework layers that initialize unused subsystems.

A useful abstraction can still be the wrong abstraction for a process-per-invocation utility.

## Performance strategy

Remove obvious unused work before profiling, then measure the rest.

Current latency-oriented choices:

- direct egui_glow runtime instead of eframe;
- Wayland-only winit;
- EGL-only glutin;
- no OS clipboard initialization during startup;
- no application icon decode path;
- no vsync wait request;
- event-loop `Wait`, not `Poll`;
- virtualized result rows;
- allocation-reused search;
- index-only keyboard navigation;
- compile-time-zero-cost production instrumentation;
- no startup files/network/config.

The remaining major unknowns are EGL/context creation, default-font initialization/coverage, corpus construction, and clipboard establishment.

## Testing strategy

### Unit tests

- matching/ranking;
- alias behavior;
- Unicode sequence preservation;
- keyboard selection movement;
- clipboard success/failure state transitions.

### Integration-ish tests

Fake the clipboard boundary to prove:

- selected text remains exact;
- successful copy requests exit;
- failed copy remains recoverable;
- cancellation never writes the clipboard.

### Host QA

Real Wayland validation is still required for:

- actual process startup/first presentation;
- focus;
- clipboard persistence;
- compositor-visible app identity;
- emoji font rendering;
- repeated invocation.

Host QA is the final layer, not a substitute for automated/static validation.

## Packaging

Deferred until the runtime is proven.

Likely eventual targets:

- release binary;
- Arch/AUR-friendly package;
- generic installation documentation.

## Security/privacy

Normal operation should:

- make no network requests;
- execute no remote content;
- store no telemetry;
- require no elevated privileges.
