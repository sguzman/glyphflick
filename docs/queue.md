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

**Status: AUTOMATED VALIDATION GREEN / TARGET QA PENDING**

The native Rust/egui binary uses a project-owned Wayland/EGL runtime rather than eframe.

Acceptance:

- [x] repository compiles in Linux CI with Wayland/EGL dependencies;
- [ ] release build executes on target Wayland environment;
- [x] one binary named `glyphflick`;
- [x] window path requires no network/configuration setup;
- [x] app/corpus/search/clipboard/performance/runtime boundaries exist;
- [x] eframe removed from the runtime dependency graph;
- [x] Wayland-only winit + EGL-only glutin stack is explicit.

Automated validation passes formatting, `cargo check`, unit tests, and strict Clippy. Real Wayland execution remains target-host QA.

## Q002 — Sample corpus vertical slice

**Status: SUPERSEDED BY Q006**

The implementation skipped the temporary sample dataset and went directly to the real local emoji corpus while preserving the intended project-owned adapter boundary.

## Q003 — Basic search

**Status: DONE**

Acceptance:

- [x] search field requests focus on launch;
- [x] typing filters records;
- [x] matching is ASCII case-insensitive for English names/shortcodes;
- [x] search logic has unit tests in-tree;
- [x] UI does not own ranking logic;
- [x] tests executed successfully in CI.

## Q004 — Clipboard backend spike

**Status: CODE COMPLETE / WAYLAND QA PENDING**

Current implementation uses `wl-copy` behind a project-owned `ClipboardBackend` trait.

Acceptance:

- [x] exact text sequence is passed unchanged;
- [x] backend reports process launch/non-zero-exit failure;
- [ ] real Wayland clipboard persistence verified after UI exit;
- [x] clipboard lifetime architecture recorded in ADR 0002.

The helper is intentionally invoked only at commit time, so it contributes zero launch-path subprocess cost.

## Q005 — Commit and dismiss path

**Status: CODE COMPLETE / QA PENDING**

Acceptance:

- [x] clicking a glyph invokes clipboard backend;
- [x] Enter commits the active result, or the first ranked result when navigation has not started;
- [x] successful backend return requests immediate viewport close;
- [x] failed copy leaves UI visible with inline error;
- [x] Escape requests close without clipboard work;
- [x] fake clipboard tests cover exact Unicode and failure behavior;
- [ ] host behavior validated.

## Q006 — Real emoji corpus adapter

**Status: DONE**

Acceptance:

- [x] Unicode 17 emoji corpus is local through `emojis` 0.9.0;
- [x] canonical names exposed;
- [x] all available shortcodes exposed;
- [x] skin-tone variants included;
- [x] multi-codepoint sequences remain opaque exact strings;
- [x] third-party emoji type remains private to corpus adapter internals;
- [x] corpus tests executed successfully in CI.

Implementation intentionally stores only static emoji references instead of allocating owned name/text copies at startup.

## Q007 — Search relevance v1

**Status: DONE**

Implemented deterministic bucket ranking without per-query sorting:

1. exact glyph text;
2. exact shortcode;
3. exact canonical name;
4. canonical-name prefix;
5. shortcode prefix;
6. canonical-name word-prefix;
7. name/shortcode substring.

Acceptance:

- [x] deterministic ranking;
- [x] tests cover exact shortcode, case, canonical name, substring;
- [x] no fuzzy search;
- [x] result vectors are allocation-reused;
- [x] tests executed successfully in CI.

## Q008 — Virtualized result grid

**Status: CODE COMPLETE / QA PENDING**

Acceptance:

- [x] egui `ScrollArea::show_rows` virtualizes rows;
- [x] cell geometry is fixed;
- [x] hover exposes canonical name;
- [ ] full-corpus scroll smoothness verified on target host.

## Q009 — Keyboard navigation

**Status: CODE COMPLETE / QA PENDING**

Acceptance:

- [x] typing remains focused on search;
- [x] Up/Down enters result navigation without a separate mode button;
- [x] active result state is index-only and allocation-free;
- [x] spatial arrow navigation follows rendered columns;
- [x] Left/Right remain normal text editing until result navigation is active;
- [x] off-screen active rows jump directly into view without animation;
- [x] query edits reset result selection and return to the top;
- [x] Enter commits the active result, or first ranked result before navigation;
- [x] Escape cancels;
- [x] navigation has unit tests in-tree and green in CI;
- [ ] keyboard behavior verified on target host.

## Q008A — Emoji font coverage and startup-cost spike

**Status: BLOCKING DESIGN / MEASUREMENT PENDING**

Current egui bundled fonts intentionally cover only a subset of the complete modern emoji corpus. Glyphflick must not silently present missing-glyph boxes, but it also must not solve coverage by adding expensive font discovery/parsing to every invocation without measurement.

Evaluate:

- bundled egui defaults as latency baseline;
- one explicitly bundled modern emoji font;
- system-font discovery/loading only if its startup cost is competitive;
- corpus filtering only as a fallback, because silently shrinking the useful corpus is undesirable.

Acceptance:

- [ ] representative Unicode 17 coverage quantified;
- [ ] startup/font initialization cost measured for viable approaches;
- [ ] selected strategy does not perform unnecessary per-launch discovery;
- [ ] rendering strategy documented in an ADR;
- [ ] host rendering verified.

## Q010 — Stable window identity

**Status: CODE COMPLETE / QA PENDING**

Acceptance:

- [x] Wayland app ID is `glyphflick`;
- [x] title is `Glyphflick`;
- [x] no Hyprland-specific logic exists in core app;
- [ ] compositor-visible identity verified on host.

## Q011 — Performance instrumentation

**Status: IN PROGRESS**

Zero-dependency timing probes now exist behind the compile-time Cargo feature `timing`; normal release builds compile them out.

Currently instrumented:

- [x] process-main start to first UI pass;
- [x] corpus initialization time and glyph count;
- [x] search update time and result count;
- [x] clipboard establishment call;
- [x] all timing probes compile out of normal release builds.

Still needed:

- [x] first successful EGL buffer-swap timing proxy;
- [x] Wayland/EGL/GL setup timing;
- [x] egui runtime initialization timing;
- [x] whether zero swap interval was actually accepted;
- [ ] compositor-observed first-visible-frame methodology;
- [ ] visible dismissal measurement;
- [ ] binary size;
- [ ] peak memory;
- [ ] cold vs warm launch series;
- [ ] documented measurements from target machine.

## Q012 — Startup optimization pass

**Status: IN PROGRESS / AUTOMATED BASELINE GREEN**

Already applied before measurement because they remove obviously unused machinery:

- [x] eframe removed entirely;
- [x] direct egui_glow runtime;
- [x] Wayland-only winit; X11 and Wayland CSD theme machinery omitted;
- [x] EGL-only glutin/glutin-winit; GLX omitted;
- [x] wgpu omitted;
- [x] egui-winit OS clipboard feature omitted, removing arboard/smithay clipboard startup initialization;
- [x] link-opening support omitted;
- [x] eframe PNG application-icon path omitted;
- [x] event loop uses Wait rather than Poll;
- [x] swap interval explicitly requests DontWait;
- [x] no async runtime;
- [x] no logging/profiling framework;
- [x] no config parsing/filesystem I/O on launch;
- [x] release `opt-level=3`;
- [x] fat LTO;
- [x] one codegen unit;
- [x] panic abort;
- [x] symbol stripping;
- [x] CI rejects X11, GLX, wgpu, and generic OS-clipboard dependencies if they re-enter the graph;
- [x] dependency/size budget workflow isolated from ordinary source commits.

Measurement-dependent work remains intentionally open.

## Q013 — Real Wayland QA

**Status: BLOCKED ON Q008A/Q011/Q012 / AUTOMATED BUILD GREEN**

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
