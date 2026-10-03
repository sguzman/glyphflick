# Architecture

## Architectural goal

Keep Glyphflick small enough that its architecture explains the whole application rather than hiding it. Latency from invocation to a useful, populated picker is the primary runtime constraint.

The application is one native Rust process with a Wayland-only window, egui interaction/state, CPU rasterization, and software presentation through softbuffer.

## Module boundaries

```text
main
 |
 +-- runtime      Wayland/winit + egui-winit + CPU rasterizer + softbuffer
 +-- app          egui state + interaction state machine
 +-- corpus       glyph records + bundled dataset adapter
 +-- search       normalization + ranking/filtering
 +-- navigation   allocation-free result cursor movement
 +-- clipboard    copy contract + Wayland implementation
 +-- perf         compile-time optional timing probes
```

UI/domain code does not own process/window lifecycle or presentation.

## Runtime architecture

Glyphflick does **not** use eframe, OpenGL, EGL, GLX, X11, or wgpu.

The runtime is assembled from:

- `winit` for a Wayland-only event loop and window;
- `egui-winit` for input/platform integration;
- `egui` for immediate-mode UI and tessellation;
- a project-owned CPU mesh rasterizer;
- `softbuffer` for presenting the CPU framebuffer to Wayland.

The event loop uses `ControlFlow::Wait`; there is no continuous polling or intentional display-refresh wait. The first useful frame contains the search field and visible result grid.

The software renderer consumes egui meshes, applies texture deltas including the color-emoji atlas, clips primitives, fast-paths canonical axis-aligned quads, falls back to textured-triangle rasterization, and presents the completed buffer through softbuffer.

OpenGL/EGL were measured during architecture selection and removed after repeated target-host tests showed the software path roughly halved process-to-first-useful-frame latency.

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

The UI does not expose third-party emoji crate types.

## Corpus strategy

The corpus is local, deterministic, preserves exact multi-codepoint sequences, and performs no network access or runtime discovery. The current adapter stores static emoji references and expands skin-tone variants into a compact index once per invocation.

## Search architecture

Search is deterministic:

```text
(query, corpus) -> ranked result indices
```

It uses reusable result/bucket vectors, no per-query sort, no fuzzy-search dependency, and ASCII case-insensitive matching for English canonical names and shortcodes. Search remains deliberately simple because measured startup/search cost is negligible compared with first-frame glyph rendering.

## UI architecture

egui owns UI interaction and tessellation, not process lifetime.

The application state holds the query, active result, ranked result indices, transient error state, commit/cancel intent, visible-grid bookkeeping, and the dedicated emoji font id.

The result grid is row-virtualized. Each visible glyph uses a project-owned fixed-size cell rather than egui's general-purpose Button atom-layout path. The custom cell retains click sensing, selection styling, hover names, accessibility metadata, and the same egui text rendering.

## Keyboard navigation

Navigation is an index state machine: constant-space row/column movement, no allocation, no animation, and direct scroll jumps when selection leaves the visible row range.

## Clipboard abstraction

Clipboard ownership is separate from egui platform clipboard support.

```text
Clipboard::copy(text) -> success/failure
```

The current backend invokes `wl-copy` only after commit. No clipboard helper or general clipboard library is initialized on launch.

## Process model

```text
external keybinding/launcher
    |
    v
Glyphflick process
    |
    +-- Wayland window + softbuffer context
    +-- egui input + first populated UI pass
    +-- CPU raster + present
    +-- search / navigate
    +-- commit
    +-- wl-copy establishes selection
    |
    v
event loop exits immediately
```

No resident Glyphflick daemon is required.

## Window/compositor boundary

Glyphflick exposes Wayland application ID `glyphflick`, title `Glyphflick`, and fixed transient-picker dimensions. It does not edit compositor configuration, install keybindings, assume a modifier, or embed Hyprland IPC.

## Dependency policy

Every runtime dependency must justify startup cost. Avoid general async runtimes, databases, configuration frameworks before needed, logging stacks, plugin systems, runtime font discovery, GPU context stacks, and convenience framework layers that initialize unused subsystems.

Runtime-budget CI rejects X11, generic OS clipboard stacks, wgpu, and OpenGL/EGL/glutin dependencies from the default graph.

## Performance strategy

Measure process start to first useful presentation, optimize the dominant buckets, and delete superseded architecture once the measurements settle a decision.

Current latency-oriented choices:

- Wayland-only winit;
- direct egui-winit integration;
- software presentation through softbuffer;
- project-owned CPU rasterization with a canonical-quad fast path;
- custom minimal glyph cells;
- no GPU context initialization;
- no OS clipboard initialization during startup;
- event-loop `Wait`, not `Poll`;
- virtualized result rows;
- allocation-reused search;
- index-only keyboard navigation;
- compile-time-optional timing instrumentation;
- no startup network/config work.

Target-host measurements have repeatedly put the production software path near 23-25 ms median first presentation, versus roughly 55 ms for the former OpenGL/EGL path.

## Font rendering

UI text uses bundled Ubuntu Light. Emoji cells use a dedicated `Glyphflick Emoji` family backed by a read-only mmap of a known Noto Color Emoji path. There is no fontconfig enumeration or directory walk. Color-font support comes from the pinned egui/epaint revision with `color_fonts` enabled.

## Testing strategy

Unit tests cover search/ranking, Unicode sequence preservation, keyboard selection movement, clipboard state transitions, and raster fast-path invariants. CI runs format, shell validation, check, tests, strict clippy, dependency guards, and a stripped release-size budget. Real Wayland validation remains the final layer for compositor-visible behavior and latency.
