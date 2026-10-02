# Glyphflick

**A summon-pick-copy-disappear glyph picker for Wayland desktops.**

Glyphflick exists because inserting an emoji or symbol should not interrupt the thing you were doing.

The intended interaction is deliberately tiny:

1. Invoke Glyphflick from a compositor or desktop keybinding.
2. Search, browse, or immediately select a glyph.
3. Click or confirm the glyph.
4. Glyphflick copies it to the clipboard and disappears.

No tray application. No persistent main window. No browser UI. No Electron. No workflow ceremony.

## Status

**Phase 0 — formalized. Implementation has not started.**

The repository currently defines the product, UX contract, architecture, design decisions, and implementation queue before code is allowed to accrete.

## Product invariants

- **Fast enough to feel like a system primitive.**
- **One-shot by default.** Selection ends the interaction.
- **Clipboard correctness beats fake instantness.** The UI may disappear immediately, but copied data must remain pasteable.
- **Keyboard and pointer are both first-class.**
- **Search is local and immediate.**
- **No daemon requirement for the MVP.**
- **No network dependency.**
- **Wayland-first.**
- **Hyprland-friendly, not Hyprland-coupled.**
- **Glyphs, not only emoji.** Emoji are the first corpus; the product model leaves room for symbols, kaomoji, and user-defined entries.
- **Configuration must never become mandatory ceremony.** Good defaults first.
- **The application owns its behavior; compositor configuration is outside this repository.**

## Scope

The MVP is a Rust + egui native application that can be launched on demand, presents a compact searchable glyph surface, copies a chosen glyph, and exits.

The repository does **not** own the user's Hyprland configuration, global keybindings, desktop package installation, or shell setup. Integration instructions can exist later as documentation, but integration changes are never silently treated as part of the application.

## Repository map

- [PROJECT.md](PROJECT.md) — ownership, boundaries, success criteria, and current state
- [AGENTS.md](AGENTS.md) — operating rules for AI/software agents working in this repository
- [docs/product.md](docs/product.md) — product specification
- [docs/ux.md](docs/ux.md) — interaction and visual behavior
- [docs/architecture.md](docs/architecture.md) — technical architecture and component boundaries
- [docs/roadmap.md](docs/roadmap.md) — phased delivery plan
- [docs/queue.md](docs/queue.md) — canonical implementation queue
- [docs/decisions/](docs/decisions/) — architectural decision records

## Name

**Glyphflick**: invoke it, flick a glyph into the clipboard, and return to the task you were already doing.

## License

Not selected yet. No license is implied by the public repository.
