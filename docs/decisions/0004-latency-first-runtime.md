# ADR 0004: Strip the runtime to the critical path

- Status: Accepted
- Date: 2026-10-02

## Context

Glyphflick exists specifically because the replacement workflow must be faster and less disruptive than the existing emoji-selection path.

A transient picker pays startup cost on every invocation. Dependencies and initialization that would be harmless in a resident desktop application can therefore be product defects here.

## Decision

The initial runtime is intentionally austere.

### eframe features

Use eframe 0.36.2 with default features disabled and enable only:

- `default_fonts`;
- `glow`;
- `wayland`.

Do not enable the default wgpu renderer or X11 backend in the initial Wayland-targeted binary.

### Runtime omissions

The initial binary has no:

- async runtime;
- application logging framework;
- persistence framework;
- configuration parser;
- startup filesystem scan;
- network stack;
- fuzzy-search engine;
- database;
- resident Glyphflick daemon.

### Release profile

Use:

- `opt-level = 3`;
- fat LTO;
- one codegen unit;
- abort-on-panic;
- stripped symbols.

Compile time is allowed to be worse if it buys a smaller/faster release artifact.

### Search

Use a linear corpus scan with allocation-reused fixed relevance buckets.

Do not build a fuzzy index or allocate normalized copies of every emoji name at startup without measurements showing that query evaluation is a meaningful bottleneck.

### Keyboard navigation

Use index arithmetic over the existing ranked-result vector.

Do not create a separate widget model, focus tree, or navigation collection solely for keyboard movement.

### Clipboard

Do not probe or start `wl-copy` at application launch.

Invoke it only after commit so clipboard machinery contributes nothing to the launch path.

### Instrumentation

Production timing instrumentation is compiled out.

The Cargo feature `timing` opts a benchmark build into microsecond probes. The normal build therefore performs no profiler environment lookup and no timing-clock reads in corpus/search/clipboard hot paths.

## Rationale

These choices remove clearly unused work without pretending we already know the remaining bottleneck.

Current eframe documentation states that the default native renderer is wgpu and that Glow can materially reduce binary size. A smaller dependency/rendering path is the better first hypothesis for a process-per-invocation utility, but renderer choice remains benchmark-revisitable.

## Non-decision: custom allocator

egui documentation notes that alternative allocators can improve some application workloads.

Glyphflick will not add mimalloc/talc/jemalloc yet because allocator initialization and binary impact can matter to cold startup. This must be measured on the actual picker before adoption.

## Non-decision: daemon

A resident process could eventually reduce repeated invocation latency, but it would fundamentally alter the product/process model and consume resources continuously.

Do not introduce a daemon until measured cold/warm launch data proves process startup is the dominant unacceptable cost and lighter approaches have been exhausted.

## Consequences

Positive:

- narrower Linux runtime;
- less renderer/dependency machinery;
- no avoidable launch I/O;
- normal release builds pay no instrumentation overhead;
- benchmark builds can measure the remaining cost cleanly.

Tradeoffs:

- initial binary is Wayland-only;
- host needs a working `wl-copy` for the first clipboard backend;
- bundled-font cost remains because glyph rendering requires it;
- assumptions about Glow must be validated against real launch measurements.

## Revisit when

Revisit after Q011 produces target-machine cold/warm launch data and renderer/clipboard timing.
