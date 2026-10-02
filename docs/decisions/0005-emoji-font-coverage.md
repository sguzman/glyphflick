# ADR 0005: Emoji font coverage must be solved against launch latency

- Status: Proposed / measurement pending
- Date: 2026-10-02

## Context

Glyphflick targets a Unicode 17 emoji corpus, but rendering coverage and launch latency pull in opposite directions.

The exact egui 0.36.2 release documents that its default bundled fonts support roughly 1,216 emoji. The bundled set is deterministic and avoids runtime host-font discovery, but it cannot represent the complete Unicode 17 corpus.

That creates two simultaneous requirements:

1. the picker must not silently advertise large numbers of missing-glyph boxes;
2. fixing coverage must not quietly add expensive per-launch font discovery, filesystem scans, decoding, or oversized assets.

This is a process-per-invocation utility. Font work is therefore part of the startup critical path.

## Verified baseline

The current `egui/default_fonts` feature embeds the 0.36.2 default font package, including:

- Ubuntu Light for proportional UI text;
- Hack for monospace text;
- Noto Emoji;
- emoji-icon-font.

No eframe/system-font provider is installed by Glyphflick's direct runtime.

Therefore the baseline is:

- deterministic;
- offline;
- no runtime system-font search;
- approximately 1,216 documented emoji supported;
- incomplete relative to the Unicode 17 corpus.

## Immediate optimization opportunity

Glyphflick does not need a dedicated code font.

After the runtime reaches compile-clean validation, measure replacing `egui/default_fonts` with an explicit font definition containing only the faces the picker actually uses:

- Ubuntu Light;
- Noto Emoji;
- emoji-icon-font if its additional coverage is useful.

This may remove Hack and an unused fallback path from first-frame font initialization.

Do not land that change before a clean baseline build exists; otherwise font work muddies runtime validation.

## Candidate strategies

### A. Current deterministic egui bundle

Advantages:

- no runtime discovery;
- simple baseline;
- known egui compatibility.

Disadvantage:

- incomplete modern emoji coverage;
- includes at least one font Glyphflick likely does not need.

### B. Slimmed deterministic egui font subset

Advantages:

- preserves offline deterministic startup;
- may remove unused font parsing/fallback work;
- no new font asset source.

Disadvantage:

- same fundamental emoji coverage ceiling unless the source font changes.

This is the preferred first latency experiment after compile validation.

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

1. record the current default-font first-swap baseline;
2. measure a slimmed deterministic font subset;
3. quantify renderable representative emoji coverage;
4. only then evaluate a newer bundled emoji source if coverage remains unacceptable.

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
