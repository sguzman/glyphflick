# ADR 0001: Rust + egui for the native picker

- Status: Accepted for Rust + egui; **eframe portion superseded by ADR 0006**
- Date: 2026-10-02

## Context

Glyphflick needs to be a small native Linux/Wayland utility with low startup latency and a compact immediate interaction surface.

## Original decision

Implement Glyphflick in Rust and use egui/eframe for the native UI.

## Current interpretation

The Rust + egui decision remains accepted.

The eframe runtime choice was deliberately removed after source-level inspection showed that eframe's native dependency configuration enables egui-winit OS clipboard support even when Glyphflick does not use that clipboard path.

ADR 0006 replaces eframe with a direct Wayland/EGL + egui_glow runtime.

## Why Rust remains

Rust provides:

- one native binary;
- explicit dependency/process control;
- strong Unicode/string tooling;
- clear separation between runtime and domain logic.

## Why egui remains

egui provides:

- an immediate-mode model that maps well to a transient picker;
- a small interaction state surface;
- efficient virtualized rendering primitives;
- no need for a large retained widget hierarchy.

## Constraint

The UI toolkit is subordinate to latency.

Framework convenience is not a reason to keep a layer whose unavoidable initialization is unnecessary for Glyphflick.
