# ADR 0005: Map one color emoji font; do not enumerate system fonts

- Status: Accepted for target QA; target latency measurement pending
- Date: 2026-10-02

## Context

Glyphflick exposes a Unicode 17 emoji corpus. The picker must render modern emoji correctly without turning every transient invocation into a font-discovery job.

The original egui 0.36.2 bundled monochrome path failed this requirement badly. Measured against Glyphflick's corpus, it exposed only 89 of 1,438 visible emoji scalars and only 93 of 3,944 corpus entries were scalar-sufficient.

A pinned modern Google Fonts outline probe was not a solution either: stable egui's outline path reported only 15 of 1,438 visible scalars. The issue was renderer capability, not merely font age.

General system-font enumeration was also rejected. The newer egui system-font provider documents that enumeration can take hundreds of milliseconds and may block fallback lookup until enumeration completes. That is incompatible with Glyphflick's process-per-invocation latency posture unless measurements prove otherwise.

## Decision

Glyphflick uses a narrow Linux color-font path:

1. keep Ubuntu Light as the UI font;
2. locate Noto Color Emoji through a short, fixed path list rather than font enumeration;
3. check the Arch/EndeavourOS package path first:
   `/usr/share/fonts/noto/NotoColorEmoji.ttf`;
4. memory-map the font read-only rather than copying the file into a heap buffer;
5. register it as a dedicated `Glyphflick Emoji` family;
6. render picker cells explicitly through that family;
7. enable only egui/epaint's color-font rendering machinery needed for bitmap/COLR glyphs;
8. pin the required post-0.36.2 egui revision exactly.

The application does not invoke fontconfig, scan font directories, spawn a font helper, start a background font-enumeration thread, or load arbitrary system fallback fonts.

## Why a dedicated emoji family

Noto Color Emoji also maps characters that can appear in ordinary text. Putting it in the UI fallback chain risks the emoji font stealing digits, symbols, or presentation choices in the search field.

The UI family therefore remains isolated to Ubuntu Light. Emoji cells opt into `Glyphflick Emoji` explicitly.

## Automated evidence

On the Ubuntu GitHub runner with `fonts-noto-color-emoji` installed:

- 1,431 / 1,438 visible corpus scalars are available;
- 3,877 / 3,944 corpus entries are scalar-sufficient;
- representative basic emoji, skin-tone, flag, and ZWJ-family sequences rasterize to non-empty geometry.

The seven missing CI scalars are:

- U+1F6D8 LANDSLIDE;
- U+1FA8A TROMBONE;
- U+1FA8E TREASURE CHEST;
- U+1FAC8 HAIRY CREATURE;
- U+1FACD ORCA;
- U+1FAEA DISTORTED FACE;
- U+1FAEF FIGHT CLOUD.

Those are the seven standalone emoji characters newly added in Unicode Emoji 17.0. The target Arch package is Noto Color Emoji 2.051, the Unicode 17 release, and installs at Glyphflick's first lookup path.

## Binary-size result

The previous stripped release baseline was:

- 6,457,568 bytes;
- 6.158 MiB.

The selected color-font implementation is:

- 6,180,192 bytes;
- 5.894 MiB.

The new implementation is 277,376 bytes smaller, approximately 4.3%, while providing color-font support.

Binary size is not treated as proof of lower launch latency. Target first-frame measurements remain required.

## Rejected alternatives

### Keep egui's bundled monochrome emoji faces

Rejected because measured coverage is far too small for the Unicode 17 corpus.

### Replace only the monochrome font file

Rejected after the modern Noto outline probe demonstrated that the stable renderer path, not only font age, was the limiting factor.

### General system-font provider

Rejected for the launch path because it enumerates host fonts and introduces background/blocking discovery machinery that Glyphflick does not need.

### Pre-rasterized full emoji atlas

Deferred. Existing egui experiments use multi-megabyte atlas assets and eagerly decode them. A Glyphflick-specific atlas would also need full UTF-8 sequence keys for flags, skin tones, and ZWJ sequences rather than single-codepoint registration. It remains an option only if target measurements show the mmap color-font path is too slow.

### Bundle a ~10 MiB color font

Not selected. Mapping the distro-provided font avoids inflating the binary and avoids copying the entire file into process memory on startup.

## Remaining work

Before Q008A is fully done:

- measure font initialization and first visible frame on the target host;
- verify the target Arch Unicode 17 font renders the newest corpus entries;
- decide the user-visible behavior when Noto Color Emoji is not installed.

## Consequences

The selected design intentionally accepts a Linux package dependency for full color emoji rendering in exchange for:

- no font discovery scan;
- no background enumeration;
- no bundled multi-megabyte color font;
- modern sequence rendering;
- a smaller executable;
- a narrow, measurable startup path.
