# AGENTS.md

This file defines working rules for AI and software agents operating in this repository.

## Prime directive

**Do the work in the repository. Do not turn implementation into a wall of code or shell instructions in chat unless the user explicitly asks for that.**

When asked to continue work, inspect the repository, make coherent changes, validate them where tooling permits, and report the result succinctly.

## Project invariants

Never violate these casually:

- Glyphflick is one-shot by default.
- Successful selection means copy, dismiss, and return control to the prior task.
- Clipboard persistence must remain correct after visible UI dismissal.
- Search is local and fast.
- The MVP does not require a daemon.
- No network dependency is permitted for normal operation.
- Wayland is the primary platform.
- Hyprland is a deployment target, not an architectural dependency.
- The repository does not own the user's compositor configuration.
- Emoji are the first dataset, not the permanent conceptual boundary.
- Startup cost matters.
- Configuration is optional, never prerequisite ceremony.

## Repository workflow

- Prefer direct commits to the default branch for owner-directed work.
- Do not create PR ceremony unless specifically requested.
- Keep commits coherent and descriptive.
- Update documentation when implementation changes product behavior or architecture.
- Use ADRs for decisions that constrain future implementation.
- Keep the canonical work queue in `docs/queue.md`.
- Mark queue items complete when they actually land and are validated.

## Communication rules

Do not:

- paste large implementation files into chat as a substitute for committing them;
- instruct the user to manually edit unrelated system configuration when the task is repository work;
- claim host integration is complete when only application code exists;
- ask for human QA before automated/static validation that can be performed first;
- expand scope just because a nearby feature is technically interesting.

Do:

- report what changed;
- call out anything that requires real host QA;
- distinguish repository correctness from compositor/integration correctness;
- preserve unresolved questions in the repo rather than repeatedly asking them in chat when a reasonable default exists.

## Engineering standards

- Rust code should be small, idiomatic, and warning-free.
- Avoid abstraction before there is a concrete need.
- Avoid asynchronous runtimes unless a real feature requires one.
- Avoid background processes unless required for clipboard semantics or another explicit product invariant.
- Prefer compile-time/bundled data for the primary glyph corpus.
- Keep search deterministic and testable.
- Separate platform clipboard behavior from UI state.
- Separate corpus/search logic from egui rendering.
- Make dismissal semantics testable at the application boundary.
- Benchmark launch/search before adding heavyweight dependencies.

## Definition of done

A queue item is not done merely because code exists.

It is done when the relevant combination of the following is true:

- code compiles;
- tests pass;
- linting is clean;
- behavior is documented when user-visible;
- architecture docs still match reality;
- required host QA is identified;
- regressions against product invariants are considered.

## User environment boundary

The user may later wire Glyphflick into Hyprland with a keybinding. That integration is external.

Do not modify or prescribe edits to the user's Hyprland configuration unless the user explicitly asks for integration work.
