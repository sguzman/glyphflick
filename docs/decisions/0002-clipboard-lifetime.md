# ADR 0002: Clipboard lifetime is independent from visible UI lifetime

- Status: Accepted
- Date: 2026-10-02

## Context

On Wayland, clipboard ownership semantics can require a provider to remain available after a selection is made.

Glyphflick's UX contract simultaneously requires the visible picker to disappear immediately after successful selection.

Naively terminating every process involved in clipboard ownership at selection time can make the copied glyph unavailable when the user attempts to paste.

## Decision

Treat **visible UI lifetime** and **clipboard ownership lifetime** as separate concerns.

The application may terminate its visible picker immediately after successful clipboard establishment, while the clipboard backend is allowed to use whatever narrowly scoped lifetime mechanism is necessary to keep the data pasteable.

## Consequences

- The core application remains one-shot.
- A clipboard helper/background ownership process is acceptable if required.
- Such a helper is not considered a general Glyphflick daemon.
- The UI reports success only once the backend has established valid clipboard ownership.
- Clipboard implementation remains behind an abstraction so the mechanism can change.

## First implementation direction

Prefer the lowest-risk correct Wayland mechanism for the first vertical slice.

An external helper such as `wl-copy` is acceptable for the first implementation if it provides the required ownership semantics cleanly.

A native Rust implementation may replace it later if:

- dependency-free distribution materially matters;
- startup measurements justify it;
- reliability is at least equivalent.

## Rejected idea

"Exit everything immediately and assume the compositor retains clipboard data."

Rejected because clipboard persistence is a correctness requirement, not an implementation detail we can wish away.

## Revisit when

Revisit after real Wayland measurements and QA of the first clipboard backend.
