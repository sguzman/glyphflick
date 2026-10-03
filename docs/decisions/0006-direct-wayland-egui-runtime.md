# ADR 0006: Replace eframe with a direct Wayland/EGL egui runtime

- Status: Superseded by ADR 0007
- Date: 2026-10-02

## Context

The first implementation used eframe with default features disabled, Glow selected, and Wayland-only intent.

Source-level inspection then found unavoidable native eframe work that Glyphflick does not need:

- eframe's native egui-winit dependency enables the OS `clipboard` feature;
- that feature pulls general clipboard implementations into startup;
- eframe also carries native application-framework behavior irrelevant to a one-window, one-shot picker;
- eframe's native package includes an image/PNG path for application icons.

Glyphflick already owns its real copy operation through a separate `ClipboardBackend`, currently `wl-copy`, and only needs that operation after selection.

Paying generic OS clipboard initialization at every launch violates the project's latency posture.

## Decision

Keep **egui**, remove **eframe**.

Build the native runtime directly from:

- `winit` for the Wayland event loop/window;
- `glutin` + `glutin-winit` for EGL/OpenGL;
- `egui_glow` for egui/winit translation and rendering.

Feature policy:

- Wayland only;
- EGL only;
- no X11;
- no GLX;
- no wgpu;
- no egui-winit OS clipboard feature;
- no link-opening feature.

Runtime policy:

- `ControlFlow::Wait`;
- no continuous repaint loop;
- `SwapInterval::DontWait`;
- request redraw only for initial paint or actual input/window events;
- exit the event loop immediately after a successful commit;
- destroy egui's GL resources before dropping the current context.

## Rationale

This removes work from the exact path that matters: process start to first usable frame.

It also makes the boundary explicit:

```text
Wayland/EGL runtime -> egui UI -> Glyphflick domain logic
```

Clipboard ownership is no longer entangled with GUI framework initialization.

## Clipboard consequence

Because the egui-winit OS clipboard feature is intentionally disabled, external OS paste into the search field is not part of this baseline.

If it becomes useful, implement paste on demand, likely through a narrowly scoped Wayland helper/backend, so a user who never pastes into search never pays clipboard initialization during launch.

## Risk

A custom runtime is more code than `eframe::run_native`.

That maintenance cost is accepted because latency is not incidental to Glyphflick; it is the primary product requirement.

## Validation

This decision remains subject to target-host measurement.

If the direct runtime does not materially improve startup, we still retain a narrower and more controllable dependency graph, but future architecture changes must be based on measured first-frame data.


## Supersession

Target-host measurement later showed that EGL/OpenGL context setup dominated launch latency. The production runtime was replaced by the software-presentation architecture documented in ADR 0007. This ADR is retained as historical reasoning for removing eframe, not as the current rendering-stack decision.
