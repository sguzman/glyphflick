# ADR 0001: Rust + egui for the native picker

- Status: Accepted
- Date: 2026-10-02

## Context

Glyphflick needs to be a small native Linux/Wayland utility with low startup latency, straightforward distribution, and a compact immediate interaction surface.

The chosen stack should avoid browser runtimes and should make it easy to keep domain logic in a single executable.

## Decision

Implement Glyphflick in Rust and use egui/eframe for the native UI.

## Rationale

Rust provides:

- a small deployable native binary;
- strong Unicode/string tooling;
- explicit control over dependencies and process behavior;
- straightforward separation of platform/UI/domain modules.

egui provides:

- an immediate-mode model suited to a transient picker;
- fast iteration on grid/search interaction;
- native Linux support;
- no need to construct a large retained widget hierarchy.

## Constraints

Using egui does not excuse poor startup behavior.

The implementation must:

- minimize enabled features;
- measure renderer/backend startup cost;
- avoid heavyweight dependencies around egui;
- keep search/corpus/clipboard logic independent of egui.

## Consequences

Positive:

- rapid implementation;
- coherent single-language codebase;
- UI state maps naturally to one-shot interaction.

Negative/risks:

- eframe startup cost must be measured rather than assumed negligible;
- emoji rendering quality depends partly on the host font/rendering stack;
- native Wayland behavior may require targeted platform handling.

## Revisit when

Reconsider the UI framework only if measured startup/rendering behavior prevents the product from meeting its latency goals.
