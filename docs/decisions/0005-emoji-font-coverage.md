# ADR 0005: Emoji font coverage must be solved against launch latency

- Status: Proposed / measurement pending
- Date: 2026-10-02

## Context

Glyphflick's corpus targets modern Unicode emoji, but egui 0.36.2 documents that its default bundled fonts cover only about 1,216 emoji.

That creates two simultaneous requirements:

1. the picker must not advertise glyphs that render as missing boxes;
2. fixing coverage must not quietly add large startup costs through font discovery, filesystem scans, parsing, or oversized assets.

This is especially important because Glyphflick is process-per-invocation and latency is the primary product constraint.

## Current temporary position

Keep egui's bundled default fonts for the first executable baseline.

Do **not** add system-font enumeration or an additional font package blindly.

The baseline is valuable because it gives us the lowest-complexity font initialization path against which alternatives can be measured.

## Candidate strategies

### A. egui bundled defaults

Advantages:

- no extra runtime discovery;
- no project-maintained font asset;
- simple baseline.

Disadvantage:

- incomplete modern emoji coverage.

### B. Explicitly bundled modern monochrome emoji font

Advantages:

- deterministic coverage;
- no filesystem discovery;
- stable across hosts.

Costs to measure:

- binary size;
- font parse/registration time;
- first glyph atlas work.

### C. System font discovery/loading

Advantages:

- potentially uses the host's current emoji fonts;
- avoids bundling another font asset.

Risks:

- startup filesystem/fontconfig work;
- host-dependent availability and behavior;
- harder-to-reproduce latency.

### D. Restrict corpus to renderable bundled glyphs

Advantages:

- minimal runtime complexity.

Disadvantage:

- sacrifices corpus completeness and can hide useful modern emoji.

Treat this as fallback rather than preferred product behavior.

### E. Image/texture emoji atlas

Not preferred for the MVP.

It adds asset, texture, and rendering complexity to solve a text-selection utility problem and must show a compelling measured advantage before consideration.

## Decision rule

Choose the approach that provides acceptable modern emoji coverage with the lowest measured launch and first-frame cost.

Do not optimize binary size at the expense of launch latency merely because it is easier to measure.

Do not perform per-invocation font discovery if a deterministic bundled solution is faster.

## Consequences

Until this spike is measured:

- the corpus implementation can be complete while rendered coverage is not;
- Q013 host QA remains blocked;
- font work is considered part of the critical path, not visual polish.

## Revisit when

Revisit immediately after the first target-host build can produce:

- baseline first-UI timing;
- representative rendering coverage;
- font initialization comparisons.
