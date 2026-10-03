# Roadmap

**MVP graduation: complete — v1.0.0 accepted 2026-10-02.**

Phases 0-5 describe the path that produced the accepted v1 baseline. Phases 6+ are optional post-MVP product development, not unfinished MVP debt.

This roadmap is ordered by product risk, not by feature excitement.

## Phase 0 — Formalization

**Status: complete**

Deliverables:

- product mission and non-goals;
- UX contract;
- architectural boundaries;
- clipboard lifetime decision;
- implementation queue;
- agent/ownership rules.

Exit condition:

The project can be implemented without rediscovering what it is supposed to be.

## Phase 1 — Minimal vertical slice

**Status: complete for v1.0.0**


Goal: prove the entire critical path with the smallest real application.

Deliver:

- Rust crate/binary;
- native egui window;
- tiny hard-coded sample glyph corpus;
- search input;
- clickable results;
- clipboard backend;
- success exit;
- Escape cancel.

This phase intentionally does not start with the full emoji corpus.

Exit condition:

A real Wayland session can invoke the program, select one of a few glyphs, observe the UI disappear, and paste the copied glyph successfully.

## Phase 2 — Real corpus and search

**Status: complete for v1.0.0**


Goal: turn the vertical slice into an actually useful picker.

Deliver:

- project-owned `Glyph` model;
- emoji corpus adapter;
- canonical names;
- aliases/shortcodes where available;
- deterministic ranking;
- virtualized grid;
- tooltip/accessibility names;
- search tests.

Exit condition:

The complete bundled emoji set is responsive and searchable without visible stalls.

## Phase 3 — Keyboard-complete interaction

**Status: complete for v1.0.0**


Goal: make pointer use optional.

Deliver:

- active selection;
- arrow-key grid navigation;
- Enter commit;
- robust focus behavior;
- query editing + grid navigation coexistence;
- scroll active result into view.

Exit condition:

The common workflow can be completed without touching the mouse.

## Phase 4 — Performance pass

**Status: complete for v1.0.0**


Goal: make the picker feel primitive-fast rather than merely acceptable.

Measure:

- process startup;
- first useful frame;
- corpus initialization;
- search update;
- click-to-copy;
- click-to-visible-exit;
- binary size;
- peak memory.

Possible work:

- trim eframe features;
- compare renderer/backend startup cost;
- reduce allocations;
- precompute normalized search metadata;
- adjust release profile;
- reconsider clipboard implementation if it dominates latency.

Exit condition:

Known bottlenecks are measured and the remaining latency is acceptable in repeated real use.

## Phase 5 — Reliability and host polish

**Status: MVP acceptance satisfied; further hardening is maintenance**


Deliver:

- robust clipboard error path;
- stable application identity for compositor matching;
- multi-invocation reliability;
- font/rendering QA;
- active-monitor behavior assessment;
- minimal host integration documentation without owning host config.

Exit condition:

Repeated daily use is boringly reliable.

## Phase 6 — Personal utility features

**Status: optional post-MVP**


Only after the base picker is excellent:

- recents;
- frequency ranking;
- favorites;
- categories;
- configurable dimensions;
- optional persistent local state.

These should remain cheap on startup.

## Phase 7 — Beyond emoji

**Status: optional post-MVP**


Potential corpora:

- Unicode symbols;
- arrows;
- mathematical symbols;
- punctuation;
- kaomoji;
- user-defined glyph/text entries.

At this stage the name Glyphflick pays off: the abstraction was never supposed to stop at emoji.

## Explicitly deferred

Do not pull these forward without a compelling use case:

- daemon architecture;
- cloud sync;
- account system;
- remote sticker/GIF providers;
- plugin framework;
- clipboard history;
- simulated typing/injection;
- compositor config management;
- large image-based emoji assets.
