# Changelog

All maintained releases from v1.0.0 onward are recorded here.

## [1.0.0] - 2026-10-02

### MVP

- Graduated Glyphflick from ravenous development into the maintained v1 release line.
- Delivered the one-shot `invoke -> find -> select -> copied -> gone` Wayland picker workflow.
- Added the local 3,944-entry Unicode emoji corpus with deterministic ranked search and virtualized rendering.
- Added pointer selection, keyboard navigation, Enter commit, Escape cancel, hover names, and stable `glyphflick` window identity.
- Preserved exact Unicode clipboard output through the commit-time `wl-copy` backend.
- Promoted the Wayland + egui + CPU rasterizer + softbuffer runtime and removed the slower OpenGL/EGL architecture.
- Reached a 23.583 ms median process-to-first-populated-present on the accepted target host in the final seven-launch production probe.
- Added maintained Semantic Versioning, Conventional Commit history, zero-spend GitHub Release distribution, and versioned executable assets.

### Runtime requirements

- Linux + Wayland.
- `wl-copy` / `wl-clipboard`.
- Noto Color Emoji at a supported system-font path.

