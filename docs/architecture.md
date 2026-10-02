# Architecture

## Architectural goal

Keep Glyphflick small enough that its architecture explains the whole application rather than hiding it.

The expected MVP is a single Rust binary with a few sharply separated modules.

## Proposed module boundaries

```text
main
 |
 +-- app          egui state + interaction state machine
 +-- corpus       glyph records + bundled dataset adapter
 +-- search       normalization + ranking/filtering
 +-- clipboard    copy contract + Wayland implementation
 +-- platform     narrowly scoped host/window behavior if needed
 +-- config       deferred; optional when it becomes real
```

The module names are provisional; the boundaries are the important part.

## Data model

A core glyph record should conceptually contain:

```text
Glyph {
    text: Unicode sequence
    name: canonical human-readable name
    aliases: zero or more alternate names/shortcodes
    keywords: zero or more search terms
    group: optional category
}
```

The UI should not depend directly on a third-party emoji crate's public types. Adapt external corpus data into a project-owned model so the corpus implementation can change without infecting the application.

## Corpus strategy

MVP preference:

- local;
- deterministic;
- bundled or compile-time accessible;
- no network;
- fast to enumerate;
- preserves multi-codepoint sequences;
- exposes names and useful aliases.

The corpus layer owns data adaptation, not ranking.

## Search architecture

Search should be a pure-ish component:

```text
(query, corpus) -> ranked result indices
```

Desired properties:

- deterministic;
- allocation-conscious where useful;
- testable without a GUI;
- no async runtime;
- no I/O in the search hot path.

Normalization may precompute lowercase/searchable fields at startup if measurement supports it.

Because the corpus is small by general search-engine standards, simplicity should win until benchmarks prove otherwise.

## UI architecture

egui owns rendering and immediate-mode interaction, but not domain logic.

The application state should hold:

- query;
- active result/keyboard cursor;
- visible result ordering;
- transient error state;
- commit/cancel intent.

It should call into search and clipboard through narrow APIs.

## Clipboard abstraction

The clipboard boundary is critical because Wayland selection ownership can outlive or depend on process lifetime.

Conceptual interface:

```text
Clipboard::copy(text) -> success/failure
```

The UI must not care whether the implementation uses:

- a direct Wayland library;
- a clipboard crate;
- a small external helper such as `wl-copy`;
- a detached ownership process.

What matters is the behavioral contract:

1. return success only after clipboard ownership is valid;
2. allow the visible UI to terminate;
3. copied content remains pasteable afterward.

The first implementation should optimize for correctness and low engineering risk. A later native backend can replace it if measurements justify removing an external helper.

## Process model

MVP:

```text
keybinding/launcher
    |
    v
glyphflick process
    |
    +-- render/search/select
    |
    +-- establish clipboard ownership
    |
    v
exit visible application
```

No daemon is required.

If the chosen clipboard mechanism needs a background ownership process after the UI exits, that process is an implementation detail of the clipboard backend, not a reason to convert the entire app into a resident daemon.

## Window/compositor boundary

Glyphflick creates a native application window.

It may expose stable properties such as application ID/title that a compositor can match.

It does not:

- edit Hyprland config;
- install keybindings;
- assume a particular modifier key;
- embed Hyprland IPC into core behavior without a compelling reason.

## Dependency policy

Every runtime dependency should answer at least one of:

- Does it materially simplify correctness?
- Does it reduce startup/interaction latency risk?
- Does it provide data we would otherwise have to maintain?
- Is replacing it ourselves clearly worse?

Avoid:

- general async runtimes for a synchronous picker;
- database engines for tiny local state;
- serialization frameworks before configuration/persistence exists;
- plugin frameworks;
- logging stacks disproportionate to the program.

## Performance strategy

Measure before exotic optimization, but design away obvious startup costs.

Likely levers:

- minimal eframe feature set;
- lightweight renderer choice based on measured startup behavior;
- pre-normalized corpus search fields;
- virtualized result rendering;
- no image assets in the hot path;
- no dynamic network/data loading;
- release LTO/strip settings after profiling;
- avoid unnecessary filesystem access on launch.

## Testing strategy

### Unit tests

- query normalization;
- matching/ranking;
- alias behavior;
- Unicode sequence preservation;
- keyboard selection index movement;
- cancel does not invoke clipboard commit;
- clipboard errors keep application in recoverable state.

### Integration-ish tests

Abstract clipboard backend to allow a fake implementation:

- selecting result sends exact text;
- successful copy requests exit;
- failed copy does not request success exit.

### Host QA

Some behavior cannot be proven by unit tests alone:

- actual Wayland clipboard persistence;
- focus behavior;
- compositor placement;
- emoji font rendering;
- perceived startup latency.

Host QA should be a narrow final layer, not a substitute for automated validation.

## Packaging

Deferred until MVP works.

Likely eventual targets:

- release binary;
- Arch package/AUR-friendly packaging;
- generic install instructions.

Packaging must not become a prerequisite to local development.

## Security/privacy

Normal operation should:

- make no network requests;
- execute no remote content;
- store no telemetry;
- require no elevated privileges.

User-defined future glyph packs/configuration should be treated as data, not executable code.
