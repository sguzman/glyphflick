# ADR 0003: Host integration stays outside the application

- Status: Accepted
- Date: 2026-10-02

## Context

Glyphflick will commonly be launched from a compositor keybinding. Hyprland is the first target environment.

It would be easy to conflate "works well with Hyprland" with "owns Hyprland configuration."

That would create unnecessary coupling and make repository work intrude into unrelated user configuration.

## Decision

Glyphflick will expose a stable executable and stable window identity.

External launchers/compositors decide:

- keybinding;
- monitor placement rules;
- floating rules;
- decorations;
- workspace behavior.

The repository may document examples, but does not modify or presume ownership of host configuration.

## Consequences

- Core application remains portable across Wayland compositors.
- Hyprland-specific configuration is documentation/integration material, not core logic.
- Project work can proceed without touching the user's desktop configuration.
- Host QA remains necessary for real focus/placement behavior.

## Revisit when

Only reconsider if a critical UX requirement cannot be achieved through standard window behavior or external compositor rules without unreasonable fragility.
