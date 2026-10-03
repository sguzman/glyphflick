# Performance Contract

Glyphflick is latency-sensitive by definition. Performance is not a cleanup phase after feature work; it is the central product constraint.

## What we care about

The critical path is:

`process start -> first useful frame -> user input -> result -> clipboard established -> visible exit`

The picker can be functionally correct and still fail the product if this path feels heavy.

## Hard engineering posture

There is no acceptable feature-driven excuse for casually making the critical path slower.

Before adding work to launch or selection:

1. establish why it belongs on the critical path;
2. prefer a design that moves it off the path;
3. measure the cost when measurement infrastructure exists;
4. reject or redesign it if the benefit does not justify the regression.

"Still fast enough" is not the optimization goal. The goal is the fastest correct implementation we can reasonably maintain.

## Metrics

Track at least:

### Launch

- process-main entry to application construction;
- corpus construction;
- application construction to first UI pass;
- total process-main entry to first useful UI pass;
- external wall-clock process/window timing once a reliable host method exists.

"Useful" means the search field and an initial result set are visible and interactive.

### Search

- query edit to ranked result availability;
- query edit to rendered updated result set.

Measure representative queries:

- one character;
- common word;
- rare word;
- no results.

### Commit

- click/Enter to clipboard backend invocation;
- clipboard invocation to confirmed success;
- confirmed success to visible exit.

### Resource cost

- release binary size;
- peak RSS during a normal invocation;
- filesystem reads during launch;
- subprocess creation if clipboard backend uses one.

## Instrumentation policy

Production profiling must not make the thing being measured slower.

The normal build compiles timing instrumentation out.

A dedicated Cargo feature named `timing` enables microsecond probes for:

- Wayland/EGL/OpenGL initialization;
- egui runtime initialization;
- whether zero swap interval was accepted;
- corpus construction;
- main-entry to first UI pass;
- first successful EGL buffer swap;
- search updates and result counts;
- clipboard establishment.

Additional probes should follow the same compile-time pattern unless there is a compelling reason not to.

## Current critical-path design

The implementation currently avoids:

- startup subprocesses;
- startup network access;
- startup config/persistence reads;
- async runtime initialization;
- logging framework initialization;
- eframe entirely;
- wgpu initialization;
- X11 and GLX backend support;
- egui-winit OS clipboard initialization;
- application-icon PNG loading;
- continuous event-loop polling;
- intentional vsync waiting;
- fuzzy-search indexing;
- pre-normalized owned copies of the corpus;
- resident-service coordination.

The corpus stores static references. Search reuses result allocations and uses fixed relevance buckets instead of sorting every query. Result rendering is row-virtualized. Keyboard navigation is index arithmetic only. `wl-copy` is launched only after selection.

## Benchmark philosophy

Do not optimize imagined bottlenecks, but do remove obviously unnecessary work before measuring.

For measurement-dependent changes:

1. record baseline;
2. identify the actual expensive segment;
3. change one relevant mechanism;
4. record the new measurement;
5. keep the change only if the tradeoff is worthwhile.

Cold and repeated/warm launches must be reported separately.

## Current automated baseline

The original slim monochrome-font baseline at commit `b8ff9f1` produced a stripped release binary of:

- 6,457,568 bytes;
- 6.158 MiB.

The selected color-font implementation initially measured:

- 6,180,672 bytes;
- 5.894 MiB.

After adding compile-time-only graphics instrumentation, the normal stripped release measured:

- 6,181,024 bytes;
- 5.895 MiB.

That is 276,896 bytes smaller, about a 4.3% reduction, despite adding color-font rendering support. The timing/QA hooks remain compiled out of the normal release; the post-probe release measurement stayed at 5.894 MiB. The reduction comes from dropping the embedded monochrome emoji faces and mapping the host's Noto Color Emoji file instead.

These are CI-host size measurements, not target-machine latency measurements. The runtime-budget workflow rejects X11, GLX, wgpu, arboard, and smithay-clipboard if they re-enter the runtime dependency graph and now runs on source changes as well as dependency changes.

## Target-host baseline — EndeavourOS / Wayland

The first target run used seven real Wayland launches through `scripts/target-probe.sh`.

Median values:

- Wayland/EGL/OpenGL initialization: 32.618 ms;
- egui runtime initialization: 3.224 ms;
- emoji font initialization: 19 µs;
- corpus initialization: 25 µs;
- process start to first UI pass: 38.362 ms;
- process start to first successful EGL buffer swap: 53.369 ms.

Observed ranges were tight: first-swap completion ranged from 52.288 ms to 54.769 ms. EGL/GL initialization is therefore the dominant measured launch component, not font or corpus work.

The target host also reported `SwapInterval::DontWait` accepted on all measured launches, complete 1,438 / 1,438 visible scalar coverage, 3,944 / 3,944 corpus entry coverage, representative complex emoji rasterization success, and clipboard persistence success.

The first measured launch is reported separately in the probe output but is not treated as a controlled cold-cache measurement.

## Target character

Until target hardware measurements exist, numerical thresholds are provisional.

The qualitative standard is uncompromising:

- first frame should feel immediate;
- typing should never visibly trail the keyboard;
- the full emoji corpus should not create scroll/search hitching;
- keyboard movement should be effectively instantaneous;
- dismissal after selection should feel instantaneous.

Once Q011 produces real target-machine data, establish explicit regression budgets from the measured baseline. Those budgets are regression alarms, not permission to stop optimizing.

## Dependency review

For each material dependency, consider:

- startup initialization work;
- dynamic library/system requirements;
- transitive dependency count;
- binary-size contribution;
- filesystem access;
- background threads;
- renderer initialization.

A dependency can be convenient and still be wrong for a transient utility.

## Likely experiments

Potential experiments after the first build is validated:

- Wayland/EGL/OpenGL startup breakdown;
- first-swap proxy versus compositor-observed visibility;
- OpenGL versus OpenGL ES context creation if context setup dominates;
- enabled-feature minimization audit;
- eager corpus construction versus alternate static indexing;
- precomputed normalized search fields only if query time warrants them;
- virtualized grid tuning;
- release LTO/codegen settings;
- native clipboard backend versus helper subprocess;
- direct Noto Color Emoji mmap + color-font initialization cost;
- allocator comparison only if allocation profiling identifies it as material.

## Anti-optimizations

Do not:

- introduce unsafe code merely to shave unmeasured microseconds;
- build custom data structures before profiling;
- replace clear linear scans over a small corpus with complex indexing without evidence;
- add a custom allocator based on generic benchmark folklore;
- keep a daemon solely to make benchmarks look good unless real repeated-use latency demands it and the product model is explicitly revisited.

## Reporting

Performance claims in commits/docs should include the environment and measurement method when possible.

"Faster" without a baseline is not a result.


### Graphics A/B — target host

A seven-launch target A/B isolated the graphics startup and first-frame costs.

Median results:

| Mode | EGL display/config/window | Graphics init | First egui run | GL paint | Swap call | First swap complete |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| OpenGL, alpha 8 | 31.139 ms | 34.467 ms | 12.413 ms | 2.534 ms | 0.844 ms | 55.719 ms |
| GLES, alpha 8 | 30.633 ms | 33.827 ms | 12.083 ms | 2.383 ms | 0.793 ms | 54.387 ms |
| OpenGL, alpha 0 | 30.017 ms | 33.306 ms | 12.394 ms | 2.419 ms | 0.793 ms | 54.625 ms |

Interpretation:

- EGL display/config/window creation is the dominant startup cost at roughly 30–31 ms.
- The first egui UI pass is the second-largest cost at roughly 12 ms.
- GL paint is roughly 2.4–2.5 ms and the first swap call itself is under 1 ms.
- GLES improved the repeated median but showed a one-off 12.321 ms egui runtime initialization spike on its first measured launch, so it is not promoted from one series.
- Alpha-zero is appropriate for Glyphflick's intentionally opaque window and improved the median without changing the context API. It is promoted to the production EGL template.


### Next architecture probe

`scripts/latency-probe.sh` is the next target-host experiment. It measures three paths with the same invocation series:

1. production OpenGL with the promoted alpha-zero EGL config;
2. timing-only deferred-grid rendering, reporting both the first shell swap and second fully populated swap;
3. a flat Wayland softbuffer presenter, with setup, buffer acquisition/fill, present-call, and process-to-first-present timings.

The softbuffer crate is gated behind the `softbuffer-probe` feature and is absent from the normal production dependency tree. CI compiles it under all-features/all-targets, while the runtime-budget workflow confirms the default release remains 6,181,056 bytes / 5.895 MiB.


### Flat softbuffer and deferred-grid target result

The seven-launch combined architecture probe resolved both outstanding questions.

Median values:

| Mode | First useful/present milestone | Fully populated milestone |
| --- | ---: | ---: |
| Production OpenGL | 54.314 ms first completed swap | 54.314 ms |
| Deferred grid | 42.915 ms shell swap | 56.001 ms second populated swap |
| Flat softbuffer | 3.074 ms first present | n/a — flat presenter only |

The deferred-grid experiment is not promoted. It proves that the first egui grid pass accounts for roughly 11 ms, but merely moving that work to a second frame makes the actual populated picker slightly slower than production.

The softbuffer result is architecturally significant. Its median process-start-to-present time is 3.074 ms, with only about 0.715 ms in window creation, 0.022 ms in buffer acquisition, 0.466 ms in filling the entire 560×440 buffer, and 0.008 ms in the present call. This demonstrates that the existing ~30 ms EGL display/config/window path is avoidable presentation-stack cost rather than an unavoidable Wayland window cost.

This does **not** by itself prove that a software-rendered Glyphflick is faster, because the flat presenter contains no egui tessellation, font-atlas upload, texture sampling, or UI rasterization.

### Real-egui software renderer probe

The next measurement closes that gap. `glyphflick-software-ui-probe` is gated behind `softbuffer-probe` and:

1. creates the same fixed-size Wayland window through softbuffer;
2. constructs the real Glyphflick egui context, mmap-backed Noto Color Emoji setup, corpus, search state, and UI;
3. runs the actual first egui frame and tessellates it;
4. applies egui texture deltas, including the real color-emoji/font atlas, to CPU texture storage;
5. rasterizes egui meshes into the softbuffer back buffer using clipped textured triangles and premultiplied-alpha blending;
6. presents once and exits.

The probe reports egui app initialization, egui run, tessellation, texture update, software rasterization, present-call, and process-to-first-present separately.

This renderer is intentionally experimental. It is not production code and cannot displace OpenGL until target measurements show a material end-to-end win with the real UI. CI compiles/tests/clippy-checks the probe, while the normal runtime budget remains 6,181,056 bytes / 5.895 MiB and rejects forbidden production dependencies.


### Real UI software result and production-shaped candidate

The first real-egui software probe measured seven launches on the target EndeavourOS/Hyprland host:

| Metric | Median |
| --- | ---: |
| Production OpenGL first completed swap | 54.677 ms |
| Real egui + CPU + softbuffer first present | 24.517 ms |
| Software egui run | 11.294 ms |
| Software tessellation | 0.012 ms |
| Software texture update | 0.437 ms |
| Software rasterization | 9.407 ms |
| Software present call | 0.016 ms |

This cuts measured process-to-visible latency by 30.160 ms, about 55%, while rendering the actual Glyphflick UI and color-emoji/font atlas.

The presentation architecture decision is therefore settled in favor of pursuing software presentation. The remaining work is no longer to prove that EGL is expensive; it is to turn the software path into a production-equivalent runtime and then optimize its two dominant buckets: first egui UI generation and CPU rasterization.

The software probe has now been upgraded into that production-shaped candidate. It uses the pinned `egui-winit` integration already shipped through `egui_glow` for real Wayland keyboard/text/mouse input and platform output, keeps the picker open during normal execution, and preserves an environment-controlled one-frame exit only for automated latency measurement. The next target probe measures the cost of this real input/runtime bridge before production promotion.


### Production software promotion and first raster fast path

The production-shaped software candidate was promoted after a second seven-launch target-host run with real egui-winit input handling.

At commit `c350d7d`, median values were:

| Metric | Production software | Legacy OpenGL |
| --- | ---: | ---: |
| First useful presentation | 24.488 ms | 56.162 ms |
| egui run | 11.864 ms | 12.510 ms |
| Software raster / GL paint | 8.936 ms | 2.494 ms |
| Present / swap call | 0.027 ms | 0.892 ms |

The software runtime is 31.674 ms faster end-to-end at the median, a 56.4% reduction versus the retained legacy OpenGL comparison path. The default runtime dependency graph no longer contains EGL/glutin/egui_glow, and the stripped release is approximately 5.77 MiB.

The first CPU raster optimization recognizes egui's canonical axis-aligned four-vertex/six-index rectangle topology and renders it with a rectangle loop instead of two generic barycentric triangles. On the target first frame it matched 93 quads while 70 triangles remained on the generic fallback. The raster median reached 8.936 ms. The fast path stays because it is conservative, covered by unit tests, and improves the measured hot path without changing UI semantics.

The dominant remaining startup bucket is now the egui first-frame run at roughly 11-12 ms. The next timing pass separates:

- time spent inside the Glyphflick app closure;
- search-field/widget construction;
- visible-grid/widget construction;
- egui work performed after the app closure returns.

That split determines whether the next optimization should simplify the widget tree or target egui/font-frame processing.


### Fast-grid A/B and OpenGL retirement

At commit `4919248`, the seven-launch target-host A/B measured:

| Mode | Grid p50 | egui p50 | Raster p50 | First present/swap p50 |
| --- | ---: | ---: | ---: | ---: |
| Production software baseline | 10.888 ms | 11.658 ms | 8.945 ms | 24.268 ms |
| Minimal-cell software candidate | 10.290 ms | 11.074 ms | 8.786 ms | 23.442 ms |
| Legacy OpenGL/EGL | n/a | 12.075 ms | 2.369 ms GL paint | 54.881 ms |

The minimal cell saves 0.598 ms in the grid and 0.826 ms end-to-end at the median while keeping the same 90 visible glyphs, click targets, hover names, selection state, scrolling, and accessibility metadata. It is promoted to production.

The OpenGL comparison path has served its purpose. Repeated measurements established that EGL/context startup overwhelms the faster GPU paint for this one-shot utility. The legacy OpenGL/EGL code and direct dependencies are removed rather than retained as a dormant alternate architecture.

The next software-only diagnostic suppresses glyph text painting while retaining cell allocation/interaction. Its delta isolates how much of the remaining ~10 ms grid cost comes from emoji text shaping/font work and the resulting raster workload.


### Final MVP latency gate and optimization closure

At commit `c31d652`, the final seven-launch software-only diagnostic measured:

| Metric | Production | No-grid-text diagnostic |
| --- | ---: | ---: |
| App UI p50 | 10.539 ms | 0.285 ms |
| Grid p50 | 10.280 ms | 0.031 ms |
| egui run p50 | 11.071 ms | 0.488 ms |
| Texture update p50 | 0.410 ms | 0.110 ms |
| Software raster p50 | 8.950 ms | 8.002 ms |
| First populated present p50 | **23.583 ms** | 11.608 ms |

Both modes rendered the same 90-cell interaction workload; only glyph text painting was suppressed in the diagnostic. The grid delta is therefore almost entirely emoji text shaping/font work, while roughly 8 ms of software raster cost remains even without glyph text.

This measurement closes the MVP optimization campaign. The production release was then exercised directly on the target EndeavourOS/Hyprland host and accepted by the principal as extremely fast.

The release build is the product. Cargo debug builds are intentionally not latency targets and may be dramatically slower because the CPU renderer and first-frame text work depend heavily on optimization.

Future performance work requires one of:

- a measured regression against the accepted v1 baseline;
- a concrete user-visible latency problem;
- a new feature whose critical-path cost must be evaluated.

Do not continue optimization archaeology merely because a lower number is theoretically possible.
