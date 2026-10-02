# Product Specification

## Problem

Selecting emoji on a Linux/Wayland desktop should be a sub-second interruption. Existing workflows often impose one or more of the following costs:

- opening a heavyweight application;
- browsing an awkward system picker;
- searching a web page;
- remembering shell commands;
- keeping a resident utility around;
- manually copying and then closing a window;
- dealing with clipboard data that vanishes when the picker exits.

Glyphflick exists to collapse the interaction into a tiny transient surface.

## Core use case

The user is typing somewhere else and wants a glyph.

1. They invoke Glyphflick through an external launcher/keybinding.
2. Glyphflick appears focused.
3. They either:
   - click a visible glyph;
   - type a query and click a result;
   - type a query and confirm a keyboard-selected result.
4. Glyphflick copies the selected Unicode sequence.
5. Glyphflick disappears immediately.
6. Focus returns naturally to the prior workflow.
7. The user pastes normally.

## Vocabulary

### Glyph

A selectable textual unit copied to the clipboard.

A glyph may be:

- an emoji sequence;
- a Unicode symbol;
- a kaomoji;
- a user-defined text snippet in a future extension.

The MVP corpus is emoji-first.

### Corpus

The complete set of selectable glyph records available to search and browse.

### Invocation

A fresh launch of the application. The MVP assumes process-per-invocation rather than a mandatory resident daemon.

### Commit

The act of choosing a glyph. A successful commit copies the glyph and terminates the visible interaction.

## MVP functional requirements

### Launch

- Opens as a compact transient picker.
- Search input receives immediate keyboard focus.
- No onboarding or splash screen.
- No network call.
- No configuration file is required.

### Browse

- Show a useful default set before typing.
- Provide a scrollable result surface.
- Render enough metadata through tooltips or secondary UI to disambiguate similar glyphs without making the grid noisy.

### Search

- Search begins on the first typed character.
- Matching is case-insensitive.
- Name matching is required.
- Alias/shortcode matching is desirable when the chosen corpus provides it.
- Search must not block the UI thread perceptibly.
- Empty query restores default ordering.

### Selection

- Pointer click commits immediately.
- Keyboard navigation is first-class.
- Enter commits the active result.
- Escape cancels without modifying clipboard contents.

### Clipboard

- Copy the exact intended UTF-8/Unicode sequence.
- Preserve multi-codepoint emoji sequences correctly.
- Clipboard content must remain available after the visible picker exits.
- Clipboard backend failure must not silently pretend success.

### Dismissal

- Success closes the visible picker immediately.
- Cancellation closes the picker immediately.
- No confirmation toast is required for the MVP; the disappearance is the acknowledgement.

## Default ordering

MVP ordering should optimize for usefulness without requiring stored history.

Preferred starting strategy:

1. small curated/common set or corpus-native stable order;
2. search relevance when a query exists.

Frequency-based recents belong after the basic picker is proven.

## Search model

A glyph record should expose searchable normalized fields such as:

- display sequence;
- canonical name;
- aliases;
- shortcodes;
- keywords/tags;
- group/category.

The search engine should be isolated from egui so relevance can evolve independently.

MVP relevance can remain intentionally simple:

1. exact alias/shortcode;
2. prefix match;
3. word-prefix match;
4. substring match.

Fuzzy matching is deferred until evidence shows it improves the workflow.

## Error behavior

Errors should be rare and compact.

### Clipboard failure

Do not close as if selection succeeded.

Show an inline error and allow retry/cancel.

### Corpus initialization failure

If using a compile-time/bundled corpus, this should be architecturally difficult or impossible.

### Rendering/font failure

The application should use the host's available emoji-capable font stack where possible. A fallback strategy may later be needed, but shipping a large embedded emoji image set is outside MVP scope unless native glyph rendering proves unacceptable.

## Persistence

MVP needs no persistent state.

Later persistence may include:

- recents;
- frequency;
- favorites;
- user-defined glyphs;
- UI preferences.

Any persistence layer must remain optional and local.

## Configuration

MVP should launch sensibly with zero configuration.

Potential future configuration:

- window dimensions;
- result cell size;
- default dataset/category;
- theme behavior;
- maximum recents;
- search weighting.

Configuration should never be required just to get a usable picker.

## Accessibility and ergonomics

- Search focus must be obvious.
- Keyboard-only completion must be possible.
- Pointer targets should be comfortably clickable.
- Tooltips/names should aid ambiguous visual glyphs.
- Text scaling should not destroy layout.
- Avoid animations that delay selection or dismissal.

## Out of scope for MVP

- clipboard history;
- synchronization;
- accounts;
- cloud search;
- sticker/GIF search;
- image emoji packs;
- inline insertion through simulated keypresses;
- compositor configuration editing;
- daemon/service architecture;
- plugin system;
- theme marketplace;
- remote content.

## Future expansion

After the core path is excellent, plausible extensions include:

- recents/frequency ranking;
- favorites;
- categories;
- non-emoji Unicode symbols;
- kaomoji;
- user-defined glyph packs;
- alternate clipboard backends;
- optional configuration file;
- command-line query/pick modes;
- packaging.

Every extension remains subordinate to the core latency and one-shot interaction contract.
