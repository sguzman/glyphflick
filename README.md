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

**Implementation active — the latency-first native vertical slice is in-tree; target-host build and Wayland QA are pending.**

The current code includes a full local emoji corpus, deterministic ranked search, virtualized result rendering, spatial keyboard navigation, one-shot copy/dismiss behavior, and compile-time optional latency instrumentation.

## Product invariants

- **Latency is the reason this project exists. Startup and selection latency outrank feature breadth.**
- **Fast enough to feel like a system primitive.**
- **One-shot by default.**
- **Clipboard correctness beats fake instantness.**
- **Keyboard and pointer are both first-class.**
- **Search is local and immediate.**
- **No daemon requirement for the MVP.**
- **No network dependency.**
- **Wayland-first.**
- **Hyprland-friendly, not Hyprland-coupled.**
- **Glyphs, not only emoji.**
- **Configuration must never become mandatory ceremony.**
- **The application owns its behavior; compositor configuration is outside this repository.**

## Latency posture

Glyphflick now uses egui **without eframe**.

The runtime is a narrow Wayland/EGL stack built directly from winit, glutin, and egui_glow. This lets the project omit general-purpose framework work that is irrelevant to the picker.

The normal launch path has:

- no X11 backend;
- no wgpu;
- no eframe;
- no egui OS-clipboard initialization;
- no async runtime;
- no config/persistence read;
- no network;
- no application logging setup;
- no fuzzy-search index;
- no icon image decode path;
- no resident Glyphflick daemon;
- no continuous polling loop;
- no requested vsync wait.

Search reuses allocations and fixed relevance buckets instead of sorting every query. Keyboard navigation is index-only. The result grid is virtualized. `wl-copy` is not launched until a glyph is committed.

The normal release build contains no timing/profiling work. The Cargo feature `timing` compiles measurement probes into a dedicated benchmark build.

The remaining large questions are EGL/context startup, font initialization/coverage, corpus construction, and clipboard establishment. Those are measurement targets, not excuses for adding more machinery.

## Scope

The repository owns Glyphflick itself.

It does **not** own the user's Hyprland configuration, global keybindings, desktop package installation, or shell setup.

## Repository map

- [PROJECT.md](PROJECT.md) — ownership, boundaries, success criteria, and current state
- [AGENTS.md](AGENTS.md) — operating rules for agents working in this repository
- [docs/product.md](docs/product.md) — product specification
- [docs/ux.md](docs/ux.md) — interaction and visual behavior
- [docs/architecture.md](docs/architecture.md) — technical architecture
- [docs/corpus.md](docs/corpus.md) — glyph data and search model
- [docs/performance.md](docs/performance.md) — latency contract
- [docs/qa.md](docs/qa.md) — validation and host QA contract
- [docs/roadmap.md](docs/roadmap.md) — phased delivery plan
- [docs/queue.md](docs/queue.md) — canonical implementation queue
- [docs/decisions/](docs/decisions/) — architectural decision records

## License

Not selected yet. No license is implied by the public repository.
