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
