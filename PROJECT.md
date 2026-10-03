# Project Contract

## Mission

Glyphflick is a low-latency, one-shot glyph picker for Wayland desktops.

Its job is not to become a desktop environment, clipboard manager, emoji platform, or configuration framework. Its job is to let a user summon a compact interface, choose a glyph, copy it, and return to the interrupted task with as little cognitive and temporal overhead as possible.

## Ownership model

This repository is treated as an actively owned software project rather than a scratchpad.

The project owner/maintainer is responsible for:

- preserving the product invariants;
- making architectural decisions explicit;
- keeping the implementation queue current;
- preferring direct repository changes over dumping implementation instructions into chat;
- preventing feature creep from degrading launch latency or interaction simplicity;
- recording material design changes in documentation or ADRs;
- keeping integration boundaries clear.

## User contract

The user's intended workflow is:

`invoke -> find -> select -> copied -> gone`

Anything that adds avoidable steps to that sequence is suspect.

## Non-goals

Glyphflick is not:

- a general-purpose clipboard history manager;
- a resident tray application;
- a web application;
- an Electron application;
- a compositor configuration manager;
- a Hyprland plugin;
- an input method editor;
- a font manager;
- a Unicode encyclopedia;
- a network service;
- a synchronization platform.

Those may be adjacent problems, but they are not this project's problem.

## Platform stance

### Primary

- Linux
- Wayland
- Hyprland as the first real deployment environment

### Architectural portability

The application should avoid binding its core model to Hyprland-specific APIs. The first integration target is Hyprland because that is where the product is needed, not because Glyphflick should become compositor-specific.

## Success criteria

The MVP is successful when all of the following are true:

1. Launching Glyphflick feels immediate in normal repeated use.
2. The picker receives focus predictably.
3. Search updates without perceptible lag.
4. Common glyphs can be selected with either mouse or keyboard.
5. Selection copies the intended Unicode sequence exactly.
6. The visible application disappears immediately after successful selection.
7. The copied glyph remains pasteable after the visible process exits.
8. Escape cancels cleanly without changing the clipboard.
9. A second invocation after exit works reliably.
10. No network access is required.

## Performance budget

Performance is a product feature, not a later optimization pass.

Targets are engineering goals rather than promises until measured:

- cold launch: minimize aggressively;
- warm/repeated launch: visually immediate;
- search response: effectively same-frame for the bundled corpus;
- click-to-dismiss: no perceptible UI delay;
- idle CPU after exit: zero, except any intentionally detached clipboard ownership helper if the selected backend requires one;
- resident memory after exit: zero for the UI process.

Every dependency should justify its startup and binary-size cost.

## Product hierarchy

When tradeoffs appear, prefer in this order:

1. correctness of copied output;
2. interaction latency;
3. predictable dismissal/focus behavior;
4. search quality;
5. keyboard and pointer ergonomics;
6. visual polish;
7. configurability;
8. extensibility.

This order is intentional. A beautiful picker that occasionally loses clipboard ownership is broken.

## Change policy

A feature belongs in the core product only if it improves the one-shot glyph-selection workflow without materially damaging latency or predictability.

Large adjacent capabilities should be isolated behind optional modules or deferred entirely.

## Current state

- Name selected: Glyphflick
- Repository initialized: yes
- Product formalized: yes
- Architecture formalized: yes
- UX formalized: yes
- Implementation: **MVP accepted; maintained v1 line**
- MVP graduation: **2026-10-02**
- Graduation version: **v1.0.0**
- Production runtime: **Wayland + egui + project CPU rasterizer + softbuffer**
- Target-host acceptance: **real release GUI exercised on EndeavourOS/Hyprland and accepted by the principal**
- Measured production startup: **23.583 ms median process-to-first-populated-present over seven target-host launches**
- Release distribution: **versioned GitHub Release assets**
- Host integration: explicitly not modified by this repository

The MVP success gate is closed. Future work is maintenance or post-MVP product development, not unfinished MVP construction.
