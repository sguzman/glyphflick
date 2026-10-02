# ADR 0005: Emoji font coverage must be solved against launch latency

- Status: In progress / shipped-font coverage measurement pending
- Date: 2026-10-02

## Context

Glyphflick targets a Unicode 17 emoji corpus, but rendering coverage and launch latency pull in opposite directions.

The exact egui 0.36.2 release documents that its default bundled fonts support roughly 1,216 emoji. The bundled set is deterministic and avoids runtime host-font discovery, but it cannot represent the complete Unicode 17 corpus.

That creates two simultaneous requirements:

1. the picker must not silently advertise large numbers of missing-glyph boxes;
2. fixing coverage must not quietly add expensive per-launch font discovery, filesystem scans, decoding, or oversized assets.

This is a process-per-invocation utility. Font work is therefore part of the startup critical path.

## Verified current baseline

Glyphflick no longer enables the generic `egui/default_fonts` feature. It installs an explicit deterministic font definition containing only:

- Ubuntu Light for UI text;
- Noto Emoji;
- emoji-icon-font.

The unused Hack code face has been removed.

No eframe/system-font provider is installed by Glyphflick's direct runtime. The current baseline is therefore:

- deterministic;
- offline;
- no runtime system-font search;
- no unused Hack face;
- still potentially incomplete relative to the Unicode 17 corpus.

The exact shipped-font scalar coverage is now measured from the font bytes through egui's own `has_glyph` API in CI instead of being inferred from package descriptions.

## Candidate strategies

### A. Current deterministic egui bundle

Advantages:

- no runtime discovery;
- simple baseline;
- known egui compatibility.

Disadvantage:

- incomplete modern emoji coverage;
- includes at least one font Glyphflick likely does not need.

### B. Slimmed deterministic egui font subset — implemented baseline

Advantages:

- preserves offline deterministic startup;
- removes the unused Hack face;
- no new font asset source.

Disadvantage:

- same fundamental emoji coverage ceiling unless the source font changes.

This is now the active baseline rather than a future experiment.

### C. Explicitly bundled newer monochrome emoji dependency

Advantages:

- potentially much broader modern emoji coverage;
- deterministic;
- no filesystem discovery.

Costs to establish before adoption:

- actual Unicode version/coverage;
- binary size;
- parse/registration time;
- first visible-frame cost;
- licensing/distribution implications.

Do not add raw font files to this repository merely for convenience.

### D. System font discovery/loading

Advantages:

- may use the host's current emoji fonts.

Risks:

- per-launch filesystem/fontconfig work;
- host-dependent results;
- reproducibility problems;
- color-font decoding may add more runtime machinery.

This is not the preferred path for a latency-first process-per-invocation utility.

### E. Restrict corpus to verified-renderable bundled glyphs

Advantages:

- minimal runtime complexity;
- no missing-glyph entries.

Disadvantage:

- intentionally sacrifices corpus completeness.

Treat as a fallback if broader rendering imposes unacceptable launch cost.

### F. Image/texture emoji atlas

Not preferred for MVP.

It adds asset, texture, and rendering complexity and must demonstrate a compelling measured latency/correctness advantage before consideration.

## Decision rule

Choose the approach that gives the best useful emoji coverage at the lowest measured first-visible-frame cost.

Priority order for this decision:

1. visible entries must actually render;
2. no runtime host-font scan unless measurements prove it competitive;
3. minimize first-frame font initialization;
4. minimize binary/asset size where it does not conflict with launch speed;
5. expand coverage toward Unicode 17.

Do not optimize binary size at the expense of launch latency merely because size is easier to measure.

## Measurement plan

Once the build is validated:

1. quantify the active three-face bundle's scalar coverage in CI;
2. record first-swap/font-initialization timing on the target host;
3. compare only against a newer deterministic emoji source if coverage remains unacceptable;
4. retain the smaller solution unless broader coverage justifies its measured launch cost.

Representative coverage must include:

- basic emoji;
- variation-selector sequences;
- skin tones;
- flags;
- ZWJ sequences;
- recently added Unicode emoji.

## Consequences

Until this spike is resolved:

- corpus completeness and rendered coverage are intentionally distinct;
- Q013 host QA remains blocked;
- system-font discovery is explicitly not part of the baseline;
- font optimization is treated as performance engineering, not visual polish.
