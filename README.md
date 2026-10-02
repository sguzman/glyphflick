# Glyphflick

**A summon-pick-copy-disappear glyph picker for Wayland desktops.**

Glyphflick exists because inserting an emoji or symbol should not interrupt the thing you were doing.

The intended interaction is deliberately tiny:

1. Invoke Glyphflick from a compositor or desktop keybinding.
2. Search, browse, or immediately select a glyph.
3. Click or confirm the glyph.
4. Glyphflick copies it to the clipboard and disappears.

No tray application. No persistent main window. No browser UI. No Electron. No workflow ceremony.

## Status

**Implementation active — the first keyboard-complete native vertical slice is in-tree; target-host build and Wayland QA are pending.**

The current code includes a full local emoji corpus, deterministic ranked search, virtualized result rendering, spatial keyboard navigation, one-shot copy/dismiss behavior, and compile-time optional latency instrumentation. See [docs/queue.md](docs/queue.md) for the exact validation state rather than treating unrun code as finished.

## Product invariants

- **Latency is the reason this project exists. Startup and selection latency outrank feature breadth.**
- **Fast enough to feel like a system primitive.**
- **One-shot by default.** Selection ends the interaction.
- **Clipboard correctness beats fake instantness.** The UI may disappear immediately, but copied data must remain pasteable.
- **Keyboard and pointer are both first-class.**
- **Search is local and immediate.**
- **No daemon requirement for the MVP.**
- **No network dependency.**
- **Wayland-first.**
- **Hyprland-friendly, not Hyprland-coupled.**
- **Glyphs, not only emoji.** Emoji are the first corpus; the product model leaves room for symbols, kaomoji, and user-defined entries.
- **Configuration must never become mandatory ceremony.** Good defaults first.
- **The application owns its behavior; compositor configuration is outside this repository.**

## Latency posture

The first implementation deliberately excludes machinery that is not needed for the critical path:

- eframe default features are disabled;
- only Wayland, Glow, and bundled fonts are enabled;
- wgpu and X11 are omitted;
- no async runtime;
- no persistence/config read on launch;
- no network;
- no logging framework;
- no fuzzy-search index;
- no resident Glyphflick daemon.

Search reuses allocations and uses fixed relevance buckets instead of sorting every query. Keyboard navigation is an index-only state machine. The result grid is virtualized. The clipboard helper is not started until a glyph is committed, so it contributes nothing to launch latency.

The normal release build contains no timing/profiling work at all. Instrumentation exists behind the Cargo feature `timing`, allowing a dedicated measurement build without making every normal invocation pay for clocks, environment checks, or a profiling framework.

Optimization beyond the obvious removals is measurement-driven. The remaining large questions are native window/renderer startup, font initialization, corpus construction, and clipboard establishment.

## Scope

The MVP is a Rust + egui native application that can be launched on demand, presents a compact searchable glyph surface, copies a chosen glyph, and exits.

The repository does **not** own the user's Hyprland configuration, global keybindings, desktop package installation, or shell setup. Integration instructions can exist later as documentation, but integration changes are never silently treated as part of the application.

## Repository map

- [PROJECT.md](PROJECT.md) — ownership, boundaries, success criteria, and current state
- [AGENTS.md](AGENTS.md) — operating rules for AI/software agents working in this repository
- [docs/product.md](docs/product.md) — product specification
- [docs/ux.md](docs/ux.md) — interaction and visual behavior
- [docs/architecture.md](docs/architecture.md) — technical architecture and component boundaries
- [docs/corpus.md](docs/corpus.md) — glyph data and search model
- [docs/performance.md](docs/performance.md) — latency metrics and optimization discipline
- [docs/qa.md](docs/qa.md) — automated validation and host QA contract
- [docs/roadmap.md](docs/roadmap.md) — phased delivery plan
- [docs/queue.md](docs/queue.md) — canonical implementation queue
- [docs/decisions/](docs/decisions/) — architectural decision records

## Name

**Glyphflick**: invoke it, flick a glyph into the clipboard, and return to the task you were already doing.

## License

Not selected yet. No license is implied by the public repository.
