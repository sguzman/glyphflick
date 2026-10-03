# ADR 0007: Use software presentation and remove OpenGL/EGL

- Status: Accepted
- Date: 2026-10-03
- Supersedes: ADR 0006's EGL/OpenGL rendering choice

## Context

Glyphflick is a process-per-invocation picker. GPU throughput is not its limiting requirement; process start to first useful populated frame is.

Target-host probes separated window creation, egui work, rendering, and presentation. A flat Wayland softbuffer path presented in about 3 ms, while EGL display/config/window setup alone cost roughly 30 ms. A real egui + CPU raster + softbuffer implementation then repeatedly produced a useful first frame near 24-25 ms, versus roughly 55-57 ms for the OpenGL/EGL runtime.

After adding real egui-winit input integration and a conservative CPU quad fast path, the software path remained decisively faster. The final comparison before retiring OpenGL measured:

- production software baseline: 24.268 ms median first present;
- minimal-cell software candidate: 23.442 ms median first present;
- legacy OpenGL/EGL: 54.881 ms median first completed swap.

## Decision

Use a Wayland-only software presentation stack:

- winit;
- egui-winit;
- egui;
- project-owned CPU rasterizer;
- softbuffer.

Remove OpenGL/EGL/glutin/egui_glow code, features, probe binaries, and direct dependencies from the repository.

Promote the minimal fixed-size glyph cell used in the final A/B. It preserves interaction and accessibility semantics while avoiding the general-purpose egui Button atom-layout path.

Keep timing probes software-only. Future experiments must target measured software bottlenecks rather than reintroducing a GPU context stack without new evidence.

## Rationale

The software path is not merely simpler; it is more than twice as fast end-to-end on the target machine for Glyphflick's actual workload.

GPU paint itself was fast, but that advantage was overwhelmed by EGL/context initialization. Glyphflick exits after one selection and does not amortize that setup cost over a long-running session.

## Consequences

Positive:

- substantially lower launch latency;
- smaller and narrower runtime dependency graph;
- no GPU-driver/context startup dependency;
- simpler Wayland presentation model;
- software renderer can be optimized specifically for Glyphflick's small fixed UI.

Costs:

- Glyphflick owns CPU rasterizer correctness/performance;
- text/glyph shaping and CPU rasterization are now the dominant first-frame costs;
- renderer visual fidelity must remain covered by focused host QA.

## Guardrail

Runtime-budget CI rejects OpenGL/EGL/glutin/egui_glow dependencies from the default graph. The old comparison implementation is deleted rather than left as a dormant alternate runtime.
