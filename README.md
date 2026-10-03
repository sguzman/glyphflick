# Glyphflick

**A summon-pick-copy-disappear glyph picker for Wayland desktops.**

Glyphflick exists because inserting an emoji or symbol should not interrupt the thing you were doing.

The interaction is deliberately tiny:

1. Invoke Glyphflick from a compositor or launcher.
2. Search, browse, or immediately select a glyph.
3. Click a glyph or confirm the keyboard selection.
4. Glyphflick copies it to the clipboard and disappears.

No tray application. No persistent main window. No browser UI. No Electron. No workflow ceremony.

## Status

**Glyphflick v1.0.0 — maintained MVP release.**

The accepted MVP has a real native GUI, a local Unicode emoji corpus, deterministic ranked search, a virtualized grid, keyboard and pointer selection, exact clipboard copy, Escape cancellation, and one-shot dismissal.

The release build has been exercised by the principal on the real target host and was accepted as extremely fast. The latest seven-launch production measurement reached a **23.583 ms median process-to-first-populated-present**. Debug builds are not performance-representative and must not be used to judge Glyphflick latency.

## Product invariants

- **Latency is the reason this project exists. Startup and selection latency outrank feature breadth.**
- **Fast enough to feel like a system primitive.**
- **One-shot by default.**
- **Clipboard correctness beats fake instantness.**
- **Keyboard and pointer are both first-class.**
- **Search is local and immediate.**
- **No daemon requirement for v1.**
- **No network dependency.**
- **Wayland-first.**
- **Hyprland-friendly, not Hyprland-coupled.**
- **Glyphs, not only emoji.**
- **Configuration must never become mandatory ceremony.**
- **The application owns its behavior; compositor configuration is outside this repository.**

## v1 runtime

Glyphflick uses egui **without eframe**.

The production runtime is a narrow Wayland software stack:

- `winit` for the Wayland-only event loop/window;
- `egui-winit` for input/platform integration;
- `egui` for UI state and tessellation;
- a project-owned CPU rasterizer;
- `softbuffer` for software presentation.

The normal launch path has no X11 backend, wgpu, OpenGL/EGL/glutin stack, generic OS clipboard initialization, async runtime, configuration/persistence read, network access, application logging setup, resident daemon, or polling loop.

Search reuses allocations and fixed relevance buckets instead of sorting every query. Keyboard navigation is index-only. The result grid is virtualized. `wl-copy` is launched only when a glyph is committed.

The normal release build contains no timing/profiling work. The Cargo feature `timing` compiles measurement probes into dedicated benchmark builds.

Emoji rendering maps the host's Noto Color Emoji file directly through fixed known paths; Glyphflick does not enumerate system fonts.

## Runtime requirements

Glyphflick v1 targets **x86_64 Linux + Wayland**.

Runtime requirements:

- a Wayland session;
- `wl-copy` from `wl-clipboard`;
- Noto Color Emoji installed at one of Glyphflick's documented lookup paths.

The GitHub Release asset is a versioned Linux x86_64 archive containing the stripped release executable. Source archives remain available automatically through GitHub Releases.

## Build from source

```bash
cargo build --release --locked
```

Use `target/release/glyphflick`. Performance claims refer to optimized release builds, not Cargo debug builds.

## Hyprland / tiling behavior

Glyphflick exposes the stable Wayland application identity `glyphflick` and uses a compact fixed-size undecorated window.

The application does **not** own compositor placement policy. Whether it floats, centers, or tiles is ultimately controlled by the compositor. On the accepted Hyprland target it appeared transiently over existing windows during testing, which is the desired behavior, but a compositor rule is the proper place to guarantee non-disruptive floating if that becomes necessary.

This repository does not mutate the user's Hyprland configuration.

## Versioning and releases

Glyphflick follows MVP-gated Semantic Versioning.

The accepted MVP establishes `v1.0.0`. After graduation:

- breaking established contracts -> major;
- meaningful backwards-compatible capability -> minor;
- backwards-compatible fixes/performance/reliability -> patch;
- internal-only changes may accumulate without a release.

Post-MVP commits use Conventional Commits. Release history and the compatibility contract are documented in [docs/versioning.md](docs/versioning.md).

GitHub Releases is the distribution surface for versioned executables. Release automation is deliberately zero-spend on the current public repository and does not use GitHub Actions artifact storage.

## Scope

The repository owns Glyphflick itself.

It does **not** own the user's Hyprland configuration, global keybindings, desktop package installation, or shell setup.

## Repository map

- [PROJECT.md](PROJECT.md) — ownership, boundaries, success criteria, and current state
- [AGENTS.md](AGENTS.md) — operating rules for agents working in this repository
- [CHANGELOG.md](CHANGELOG.md) — maintained release history
- [docs/versioning.md](docs/versioning.md) — post-MVP compatibility and release contract
- [docs/product.md](docs/product.md) — product specification
- [docs/ux.md](docs/ux.md) — interaction and visual behavior
- [docs/architecture.md](docs/architecture.md) — technical architecture
- [docs/corpus.md](docs/corpus.md) — glyph data and search model
- [docs/performance.md](docs/performance.md) — latency contract and measurements
- [docs/qa.md](docs/qa.md) — validation and host QA contract
- [docs/roadmap.md](docs/roadmap.md) — phased delivery plan
- [docs/queue.md](docs/queue.md) — canonical implementation queue
- [docs/decisions/](docs/decisions/) — architectural decision records

## License

Not selected yet. No license is implied by the public repository.
