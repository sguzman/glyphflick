# Canonical Work Queue

This is the project's authoritative implementation queue.

Items are ordered unless a dependency or newly discovered defect requires reprioritization.

## Q000 — Project formalization

**Status: DONE**

- [x] Establish mission.
- [x] Establish scope/non-goals.
- [x] Establish UX contract.
- [x] Establish architecture.
- [x] Establish clipboard lifetime invariant.
- [x] Establish roadmap.
- [x] Establish agent working rules.

## Q001 — Bootstrap Rust application

**Status: READY**

Create the minimum Rust project required to open a native egui surface.

Acceptance:

- repository builds on target Linux environment;
- one binary named `glyphflick`;
- window launches without network/filesystem setup;
- code structure already respects app/corpus/search/clipboard boundaries even if modules are tiny.

## Q002 — Sample corpus vertical slice

**Depends on:** Q001

Add a deliberately tiny built-in sample corpus.

Acceptance:

- several emoji/glyphs render;
- records use a project-owned `Glyph` model;
- no dependency-specific emoji type leaks into the UI.

## Q003 — Basic search

**Depends on:** Q002

Acceptance:

- search field focused on launch;
- typing filters sample records;
- matching is case-insensitive;
- search logic has unit tests;
- UI does not own ranking logic.

## Q004 — Clipboard backend spike

**Depends on:** Q001

Resolve the concrete Wayland clipboard mechanism.

Candidates may include an external `wl-copy` helper or a native Rust implementation.

Acceptance:

- exact Unicode sequence copies;
- backend reports failure honestly;
- content remains pasteable after visible UI process exits;
- backend choice and rationale recorded in ADR if implementation changes the current decision.

## Q005 — Commit and dismiss path

**Depends on:** Q002, Q004

Acceptance:

- clicking a glyph invokes clipboard backend;
- successful copy closes UI immediately;
- failed copy leaves UI visible with compact error;
- Escape exits without changing clipboard;
- fake clipboard tests cover success/failure behavior.

## Q006 — Real emoji corpus adapter

**Depends on:** Q003

Acceptance:

- complete chosen emoji corpus available locally;
- canonical name preserved;
- aliases/shortcodes included when source supports them;
- multi-codepoint sequences preserved exactly;
- external crate types terminate at corpus boundary.

## Q007 — Search relevance v1

**Depends on:** Q006

Implement deterministic ranking:

1. exact alias/shortcode;
2. prefix;
3. word-prefix;
4. substring.

Acceptance:

- tests cover representative ambiguous queries;
- stable ordering;
- no fuzzy search yet.

## Q008 — Virtualized result grid

**Depends on:** Q006

Acceptance:

- full corpus scroll remains smooth;
- only visible/near-visible rows are rendered when practical;
- cell layout is stable;
- hover exposes human-readable name.

## Q009 — Keyboard navigation

**Depends on:** Q008

Acceptance:

- arrows move active result spatially;
- Enter commits;
- selection scrolls into view;
- typing/search remains natural;
- Escape cancels.

## Q010 — Stable window identity

**Depends on:** Q001

Acceptance:

- predictable app ID/title/class properties for external compositor rules;
- no Hyprland-specific logic required in core app.

## Q011 — Performance instrumentation

**Depends on:** Q005, Q006

Measure:

- startup to first useful frame;
- corpus setup;
- search update;
- copy call;
- visible dismissal;
- memory;
- binary size.

Acceptance:

- measurements documented;
- no optimization claimed without evidence.

## Q012 — Startup optimization pass

**Depends on:** Q011

Acceptance:

- dependency/features reviewed;
- renderer/backend choice measured;
- unnecessary launch I/O removed;
- release profile tuned only where measurements justify it.

## Q013 — Real Wayland QA

**Depends on:** Q005, Q009, Q012

Manual host verification:

- launch/focus;
- copy persistence;
- repeated invocation;
- cancellation;
- keyboard-only flow;
- pointer flow;
- font rendering;
- compositor behavior.

This is intentionally late enough that QA is validating a coherent application, not compensating for missing automated work.

## Q014 — Minimal integration documentation

**Depends on:** Q013

Document examples for wiring Glyphflick into a compositor/launcher.

Acceptance:

- documentation only;
- no assumption that repository owns user's config;
- Hyprland may be the first example;
- keybinding choice remains user-controlled.

## Q015 — Recents/frequency design

**Depends on:** daily-use feedback after Q013

Do not implement until there is evidence it improves real selection speed.

Potential design constraints:

- local only;
- tiny persistence format;
- bounded history;
- no startup regression.

## Q016 — Additional glyph corpora

**Depends on:** mature emoji picker

Investigate:

- Unicode symbols;
- arrows;
- math;
- punctuation;
- kaomoji;
- user-defined entries.

Keep corpora separable and search-compatible.

---

## Queue discipline

When an item begins, update its status to `IN PROGRESS`.

When implementation lands but host QA is required, use `CODE COMPLETE / QA PENDING`.

Only use `DONE` when its acceptance criteria are actually satisfied.
