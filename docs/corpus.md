# Corpus and Search Model

Glyphflick is emoji-first, but the core model is deliberately glyph-oriented.

## Core record

The project should own a stable internal record similar to:

```text
GlyphRecord
- text
- canonical_name
- aliases[]
- keywords[]
- group
- subgroup
- source
```

Not every field must exist in the first implementation. The important rule is that third-party corpus types stop at the adapter boundary.

## Identity

For MVP, the copied Unicode sequence is the practical identity of an entry.

Future user-defined entries may require a separate stable ID if duplicate text with different metadata becomes useful.

## Unicode correctness

Never assume one visible emoji equals one Unicode scalar value.

Corpus/search/clipboard code must preserve sequences exactly, including:

- variation selectors;
- zero-width joiners;
- skin-tone modifiers;
- regional indicators;
- combining marks;
- keycap sequences.

The UI may display a sequence as one visual glyph while the data layer treats it as an opaque string.

## Sources

The initial source should be:

- local;
- versioned through normal Rust dependency/build mechanisms or repository data;
- sufficiently current;
- metadata-rich enough for useful names/search;
- deterministic offline.

Do not add runtime network fetching for emoji metadata.

## Normalization

Search normalization may include:

- Unicode-preserving text storage;
- lowercase canonical name;
- lowercase aliases;
- normalized whitespace;
- underscore/hyphen/space equivalence for shortcode-like queries.

Do not mutate the copied text to perform search normalization.

Search text and output text are separate concerns.

## Ranking v1

Given query `q`, rank matches broadly as:

1. exact alias/shortcode;
2. exact canonical name;
3. canonical-name prefix;
4. alias prefix;
5. word-prefix;
6. substring.

Exact numeric weights are implementation detail.

Tie-breaking must be deterministic.

## Fuzzy search

Deferred.

Reasons:

- corpus is small;
- emoji names are generally searchable through clear terms;
- fuzzy matching can produce surprising irrelevant results;
- fuzzy libraries add dependency/complexity cost.

Add only after observing real failed-search behavior.

## Default empty-query ordering

The initial implementation may use stable corpus ordering.

Later candidates:

- curated common glyphs;
- recents;
- frequency;
- favorites.

User-specific ordering should never destroy the ability to browse predictably.

## Multiple corpora

Future corpus types should plug into the same conceptual search surface.

Examples:

- emoji;
- arrows;
- math symbols;
- punctuation;
- kaomoji;
- user entries.

A future `CorpusId` or source field may support filtering/categories without changing the clipboard contract.

## Search tests

Maintain table-driven tests covering:

- exact names;
- aliases;
- punctuation differences;
- case;
- multiple-word queries;
- ambiguous prefixes;
- no-result queries;
- stable ordering.

When real user searches fail unexpectedly, turn them into corpus/search regression fixtures.
