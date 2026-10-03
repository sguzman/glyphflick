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

**Status: DONE — MVP HOST ACCEPTED**

The native Rust/egui binary uses a project-owned Wayland software runtime rather than eframe or a GPU context stack.

Acceptance:

- [x] repository compiles in Linux CI with Wayland software-presentation dependencies;
- [x] release build executes on target Wayland environment;
- [x] one binary named `glyphflick`;
- [x] window path requires no network/configuration setup;
- [x] app/corpus/search/clipboard/performance/runtime boundaries exist;
- [x] eframe removed from the runtime dependency graph;
- [x] Wayland-only winit + egui-winit + softbuffer stack is explicit.

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

**Status: DONE**

Current implementation uses `wl-copy` behind a project-owned `ClipboardBackend` trait.

Acceptance:

- [x] exact text sequence is passed unchanged;
- [x] backend reports process launch/non-zero-exit failure;
- [x] real Wayland clipboard persistence verified after UI exit;
- [x] clipboard lifetime architecture recorded in ADR 0002.

The helper is intentionally invoked only at commit time, so it contributes zero launch-path subprocess cost.

## Q005 — Commit and dismiss path

**Status: DONE — MVP HOST ACCEPTED**

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

**Status: DONE — MVP HOST ACCEPTED**

Acceptance:

- [x] egui `ScrollArea::show_rows` virtualizes rows;
- [x] cell geometry is fixed;
- [x] hover exposes canonical name;
- [ ] full-corpus scroll smoothness verified on target host.

## Q009 — Keyboard navigation

**Status: DONE — MVP HOST ACCEPTED**

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

**Status: DONE**

Current egui bundled fonts intentionally cover only a subset of the complete modern emoji corpus. Glyphflick must not silently present missing-glyph boxes, but it also must not solve coverage by adding expensive font discovery/parsing to every invocation without measurement.

Selected implementation:

- keeps Ubuntu Light as the isolated UI font;
- maps one known Noto Color Emoji file directly instead of enumerating system fonts;
- checks the Arch/EndeavourOS package path first, with a few fixed Linux fallback paths;
- memory-maps the font instead of reading/copying the ~10 MiB file into a startup buffer;
- renders picker cells through a dedicated emoji font family so emoji fallback cannot steal ordinary UI text;
- uses egui's post-0.36.2 color-font renderer pinned to an exact upstream commit;
- performs no fontconfig scan, directory walk, background enumeration, or font subprocess on launch.

Automated evidence:

- CI representative sequences for a basic emoji, skin tone, flag, and ZWJ family rasterize successfully;
- Ubuntu CI's older packaged Noto font covers 1,431 / 1,438 visible corpus scalars and 3,877 / 3,944 entries;
- the seven missing CI scalars are the seven newly added Unicode 17 standalone emoji characters;
- the target Arch package is Noto Color Emoji 2.051 / Unicode 17 and installs at Glyphflick's first lookup path;
- the stripped release binary fell from 6.158 MiB to 5.894 MiB while adding color-font rendering.

Still evaluate:

Target-host results:

- Noto Color Emoji mapped from `/usr/share/fonts/noto/NotoColorEmoji.ttf`;
- visible scalar coverage: 1,438 / 1,438;
- scalar-sufficient corpus entries: 3,944 / 3,944;
- representative sequence rasterization passed;
- font initialization median: 19 µs;
- first UI median: 38.362 ms;
- first successful swap median: 53.369 ms;
- missing-font fallback remains explicit rather than silently showing tofu.

Acceptance:

- [x] representative Unicode 17 coverage quantified in CI;
- [x] startup/font initialization cost measured on target host;
- [x] selected strategy performs no font enumeration/discovery scan;
- [x] rendering strategy documented in ADR 0005;
- [x] host rendering verified.

## Q010 — Stable window identity

**Status: DONE — MVP HOST ACCEPTED**

Acceptance:

- [x] Wayland app ID is `glyphflick`;
- [x] title is `Glyphflick`;
- [x] no Hyprland-specific logic exists in core app;
- [ ] compositor-visible identity verified on host.

## Q011 — Performance instrumentation

**Status: DONE FOR MVP — FURTHER PROFILING OPTIONAL**

Zero-dependency timing probes now exist behind the compile-time Cargo feature `timing`; normal release builds compile them out.

Currently instrumented:

- [x] process-main start to event-loop creation and `resumed`;
- [x] EGL display/config/window creation;
- [x] GL/GLES context creation;
- [x] EGL surface creation;
- [x] make-current and GL loader setup;
- [x] first egui run, first GL paint, and first swap call;
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
- [x] release binary size reported by runtime-budget CI: 6,180,672 bytes / 5.894 MiB on the selected color-font runtime (down from 6,457,568 bytes / 6.158 MiB);
- [ ] peak memory;
- [ ] cold vs warm launch series;
- [x] documented measurements from target machine (automated by `scripts/target-probe.sh`).

## Q012 — Startup optimization pass

**Status: DONE — SOFTWARE RUNTIME IS PRODUCTION**

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
- [x] opaque EGL config requests alpha size 0 after target A/B reduced median first-swap completion by ~1.1 ms;
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

Measured target work now establishes that EGL display/config/window creation (~30–31 ms median) and the first egui UI pass (~12 ms median) dominate startup. Alpha-zero is promoted to production.

The combined target architecture probe is complete:

- production OpenGL median first completed swap: 54.314 ms;
- deferred-grid median shell swap: 42.915 ms, but median fully populated second swap: 56.001 ms;
- flat Wayland softbuffer median first present: 3.074 ms.

The deferred-grid shell is therefore diagnostic only and is not promoted: it improves empty-shell visibility while slightly delaying the useful populated frame.

The flat softbuffer result is large enough to justify removing EGL as an active architecture candidate. A probe-only real-egui software renderer is now in-tree as `glyphflick-software-ui-probe`. It reuses the actual Glyphflick app, corpus, font atlas, and egui tessellation, then rasterizes the resulting meshes into softbuffer. The next target measurement decides whether CPU rendering preserves the flat-presenter startup advantage once the real UI is included.

Softbuffer and the CPU renderer are the normal production path. OpenGL/EGL/glutin/egui_glow code and direct dependencies are removed, and runtime-budget CI rejects their re-entry.

## Q013 — Real Wayland QA

**Status: MVP ACCEPTANCE RECORDED — EXTENDED QA OPTIONAL**

Host verification (automated where possible by `scripts/target-probe.sh`):

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


Measured architecture result: the final seven-launch comparison measured 24.268 ms median for the software baseline, 23.442 ms for the minimal-cell software candidate, and 54.881 ms for legacy OpenGL/EGL. The minimal cell is promoted and the OpenGL comparison implementation is deleted.

The final software-only diagnostic at commit `c31d652` measured the production path at 23.583 ms median process-to-first-present and the no-grid-text diagnostic at 11.608 ms. The remaining first-frame cost is dominated by emoji text generation; the product latency target is accepted and the optimization campaign is closed unless a real regression appears.

## Q017 — MVP graduation and maintained release protocol

**Status: DONE**

- [x] principal accepted the real release GUI on the target EndeavourOS/Hyprland host;
- [x] project state/documents graduated from pre-MVP language;
- [x] stable compatibility contract recorded;
- [x] post-MVP Conventional Commit policy adopted;
- [x] Semantic Versioning policy adopted;
- [x] initial v1.0.0 changelog established;
- [x] zero-spend GitHub Release distribution selected for the public repository;
- [x] versioned Linux x86_64 executable archive defined as the initial release asset;
- [x] old compiled assets are explicitly prunable under real provider constraints while durable history is preserved.
