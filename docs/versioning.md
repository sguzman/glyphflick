# Versioning and Release Contract

## Lifecycle

- regime: **maintained**
- MVP accepted: **2026-10-02**
- graduation release: **v1.0.0**
- tag scheme: **vX.Y.Z**
- canonical release host: **GitHub Releases**
- initial binary target: **x86_64-unknown-linux-gnu**

Glyphflick used its pre-MVP package version only as provisional toolchain metadata. The accepted usable product begins its maintained history at v1.0.0.

## Established v1 contract

The stable product contract is intentionally narrow.

### User workflow

`invoke -> find -> select -> copied -> gone`

Existing v1 users may reasonably depend on:

- launch produces one compact picker window;
- search is focused and local;
- visible results are immediately selectable by pointer;
- keyboard search/navigation + Enter can commit a result;
- Escape cancels without a successful clipboard write;
- successful selection copies the exact Unicode sequence and requests exit;
- no network access is required for normal operation;
- no resident Glyphflick daemon is required.

### Platform/runtime

v1 targets Linux Wayland.

Runtime requirements are part of the packaging contract:

- `wl-copy` is available for commit-time clipboard ownership;
- Noto Color Emoji is available at a documented fixed lookup path;
- the released prebuilt executable target is x86_64 GNU/Linux.

The stable Wayland app identity is `glyphflick`.

Compositor placement policy is **not** an application compatibility promise. Floating/centering rules remain external integration policy, although non-disruption of the tiling workflow is the desired deployment behavior.

### Configuration and persistence

v1 has no required application configuration file and no persistent user database.

Adding optional configuration or persistence is normally minor-version work if existing behavior remains valid. Making it mandatory or invalidating an established format can be a breaking change.

### Public API

Glyphflick v1 is an application, not a stable Rust library API. Internal Rust modules are not a public compatibility surface.

## Release classification

- **major**: breaks an established contract above or another documented compatibility surface;
- **minor**: adds meaningful backwards-compatible capability;
- **patch**: fixes or improves existing behavior without materially expanding/breaking the contract;
- **no release required**: internal/docs/test/CI/refactor work with no release-worthy product change.

The highest-impact change in a release determines the bump.

## Commit protocol

Post-MVP commits use Conventional Commits.

Default release signals:

- `feat` -> minor;
- `fix`, `perf`, release-relevant `deps` -> patch;
- `type!` or `BREAKING CHANGE:` -> major;
- `docs`, `test`, `ci`, `refactor`, `style`, `build`, `chore` -> no bump by themselves.

Actual compatibility semantics outrank the prefix.

## Release tooling

- `cliff.toml`: changelog generation from maintained Conventional Commit history;
- `release.toml`: cargo-release orchestration;
- `.github/workflows/release.yml`: verified build and GitHub Release publication;
- Cargo manifest: package-version source of truth.

The release workflow is triggered only by a main-branch commit whose subject is exactly:

`chore(release): prepare X.Y.Z`

It validates that the subject matches the Cargo package version, runs the release validation suite, builds the stripped release binary, packages it, generates a SHA-256 file, and publishes both directly to GitHub Releases.

The workflow does not use GitHub Actions artifact storage.

## Zero-spend boundary

The repository is public. Standard GitHub-hosted Actions runners are currently free and unlimited for public repositories, and GitHub Release assets currently have no total release-size or bandwidth limit subject to per-asset/per-release constraints.

Do not switch to paid/larger runners, private metered build/storage paths, GitHub Packages, or another billable distribution mechanism without explicit principal authorization.

If repository visibility or GitHub billing policy changes, re-verify this boundary before the next automated release.

## Release asset retention

Keep old compiled assets while free and practical.

If a real provider/storage/quota constraint blocks a newer release, older compiled executable/archive assets may be pruned oldest/non-current first. Preserve the release record, tag, changelog, source commit, and version identity whenever the provider allows asset-only removal.

Do not rewrite/reuse historical version tags. Do not delete immutable historical releases merely to recover space without explicit principal authorization.

## v1.0.0 graduation evidence

The runtime code promoted to v1.0.0 is the already-validated software-rendered runtime at commit `c31d652`; graduation changes are documentation, release metadata/tooling, and stale-probe cleanup rather than a new runtime architecture.

The principal exercised the real optimized release GUI on the target EndeavourOS/Hyprland host and accepted the MVP and its responsiveness.

The final seven-launch production latency probe at `c31d652` measured **23.583 ms median process-to-first-populated-present**.
