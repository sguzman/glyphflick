# Performance Contract

Glyphflick is latency-sensitive by definition. Performance is not a cleanup phase after feature work; it is part of the product contract.

## What we care about

The critical path is:

`process start -> first useful frame -> user input -> result -> clipboard established -> visible exit`

The picker can be functionally correct and still fail the product if this path feels heavy.

## Metrics

Track at least:

### Launch

- process start to application initialization;
- application initialization to first useful frame;
- total process start to first useful frame.

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

## Benchmark philosophy

Do not optimize imagined bottlenecks.

Before a performance-specific change:

1. record baseline;
2. identify the actual expensive segment;
3. change one relevant mechanism;
4. record the new measurement;
5. keep the change only if the tradeoff is worthwhile.

## Target character

Until target hardware measurements exist, numerical thresholds are provisional.

The qualitative standard is stronger:

- first frame should feel immediate;
- typing should never visibly trail the keyboard;
- the full emoji corpus should not create scroll/search hitching;
- dismissal after selection should feel instantaneous.

Once Q011 lands, replace vague language with observed measurements and explicit budgets.

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

Potential experiments after the vertical slice exists:

- eframe renderer/backend comparison;
- enabled-feature minimization;
- precomputed normalized search fields;
- stable indexed corpus vs per-query normalization;
- virtualized grid tuning;
- release LTO/codegen settings;
- native clipboard backend vs helper subprocess;
- font fallback behavior.

## Anti-optimizations

Do not:

- introduce unsafe code merely to shave unmeasured microseconds;
- build custom data structures before profiling;
- replace clear linear scans over a small corpus with complex indexing without evidence;
- keep a daemon solely to make benchmarks look good unless real repeated-use latency demands it and the product model is explicitly revisited.

## Reporting

Performance claims in commits/docs should include the environment and measurement method when possible.

"Faster" without a baseline is not a result.
