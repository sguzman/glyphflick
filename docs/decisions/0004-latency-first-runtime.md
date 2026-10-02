# ADR 0004: Strip the runtime to the critical path

- Status: Accepted; runtime implementation refined by ADR 0006
- Date: 2026-10-02

## Context

Glyphflick exists specifically because the replacement workflow must be faster and less disruptive than the existing emoji-selection path.

A transient picker pays startup cost on every invocation. Dependencies and initialization that would be harmless in a resident desktop application can therefore be product defects here.

## Decision

The runtime is intentionally austere.

### Window/render stack

Use:

- egui 0.36.2;
- egui_glow 0.36.2;
- winit 0.30.13 with Wayland only;
- glutin/glutin-winit with EGL + Wayland only.

Do not use eframe, wgpu, X11, GLX, or egui-winit's OS clipboard feature.

Use an event-driven `ControlFlow::Wait` loop rather than continuous polling.

Request `SwapInterval::DontWait` so presentation is not intentionally delayed to a refresh boundary.

### Runtime omissions

The initial binary has no:

- async runtime;
- application logging setup;
- persistence framework;
- configuration parser;
- startup filesystem scan;
- network stack;
- fuzzy-search engine;
- database;
- resident Glyphflick daemon;
- startup clipboard provider;
- PNG application-icon path.

### Release profile

Use:

- `opt-level = 3`;
- fat LTO;
- one codegen unit;
- abort-on-panic;
- stripped symbols.

Compile time is allowed to be worse if it buys a leaner release artifact.

### Search

Use a linear corpus scan with allocation-reused fixed relevance buckets.

Do not build a fuzzy index or allocate normalized copies of every emoji name at startup without measurements showing query evaluation is a meaningful bottleneck.

### Keyboard navigation

Use index arithmetic over the existing ranked-result vector.

Do not create a separate widget model, focus tree, or navigation collection solely for keyboard movement.

### Clipboard

Do not probe or start `wl-copy` at application launch.

Invoke it only after commit so clipboard machinery contributes nothing to the launch path.

Do not enable egui-winit's OS clipboard feature merely to support rare search-field paste. If external paste becomes important, implement it on demand so its cost is paid only when invoked.

### Instrumentation

Production timing instrumentation is compiled out.

The Cargo feature `timing` opts a benchmark build into microsecond probes. The normal build performs no profiler environment lookup and no timing-clock reads in corpus/search/clipboard hot paths.

## Non-decision: custom allocator

Do not add mimalloc/talc/jemalloc based on generic benchmark folklore. Measure the actual picker first.

## Non-decision: daemon

A resident process could reduce repeated invocation latency but fundamentally changes resource/lifecycle behavior.

Do not introduce a daemon until measured cold/warm data proves process startup is the unacceptable dominant cost and lighter approaches have been exhausted.

## Consequences

Positive:

- no unused eframe application layer;
- no eframe-forced OS clipboard initialization;
- no X11/GLX/wgpu paths;
- no intentional vsync wait;
- event-driven idle behavior;
- production instrumentation overhead is compiled away.

Tradeoffs:

- initial binary is Wayland-only;
- host needs working EGL/OpenGL;
- host needs `wl-copy` for the first clipboard backend;
- external Ctrl+V paste into search is deferred until an on-demand path exists;
- custom runtime code is now ours to maintain.

## Revisit when

Revisit after target-machine measurements identify the actual dominant costs.
