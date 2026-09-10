# `eframe` / `egui` Version Pinning Research

Research date: 2026-09-10 (machine `date` output: `Thu Sep 10 12:01:28 PM +08 2026`).

Context: `Cargo.toml` in this repo pins `eframe = "0.36.2"` and `egui = "0.36.2"`, added via `cargo add`.
Because both crates are pre-1.0 (major version `0`), Cargo's default caret requirement `"0.36.2"`
means `>=0.36.2, <0.37.0` (see [§5](#5-recommendation) for the primary-source citation on this rule).

---

## 1. Is `egui` 0.36.2 / `eframe` 0.36.2 a real, published, non-yanked release?

**Yes, for both crates.**

- crates.io API for `egui` ([`https://crates.io/api/v1/crates/egui`](https://crates.io/api/v1/crates/egui)) lists `0.36.2` with `yanked: false`, published 2026-09-08.
- crates.io API for `eframe` ([`https://crates.io/api/v1/crates/eframe`](https://crates.io/api/v1/crates/eframe)) likewise lists `0.36.2` with `yanked: false`, published 2026-09-08.
- docs.rs corroborates both as real, successfully-built publishes:
  - [`https://docs.rs/egui/0.36.2`](https://docs.rs/egui/0.36.2) renders full API docs headed "egui 0.36.2" (reported ~77% documented — a normal docs-coverage number, not a build failure).
  - [`https://docs.rs/eframe/0.36.2`](https://docs.rs/eframe/0.36.2) renders full API docs headed "eframe-0.36.2" (100% documented).

No yanked versions were found among the recent `0.34.x`–`0.36.x` range for either crate.

## 2. What is the current latest stable release, and is anything newer or yanked?

**`0.36.2` *is* the current latest stable release of both `egui` and `eframe` as of today (2026-09-10).** It is not behind — it's the newest published version.

From the crates.io API responses (same URLs as above):

| Crate  | Version | Yanked | Published   |
|--------|---------|--------|-------------|
| egui   | 0.36.2  | false  | 2026-09-08  |
| egui   | 0.36.1  | false  | 2026-08-07  |
| egui   | 0.36.0  | false  | 2026-08-05  |
| egui   | 0.35.0  | false  | 2026-06-25  |
| eframe | 0.36.2  | false  | 2026-09-08  |
| eframe | 0.36.1  | false  | 2026-08-07  |
| eframe | 0.36.0  | false  | 2026-08-05  |
| eframe | 0.35.0  | false  | 2026-06-25  |

No 0.36.x or later pre-release/yanked version exists for either crate — `0.36.2` is simply the tip of the release train, two patch releases past `0.36.0`. This was independently confirmed against the [`egui` top-level `CHANGELOG.md`](https://raw.githubusercontent.com/emilk/egui/master/CHANGELOG.md) and [`crates/eframe/CHANGELOG.md`](https://raw.githubusercontent.com/emilk/egui/master/crates/eframe/CHANGELOG.md), whose newest entries are both headed `## 0.36.2 - 2026-09-08` with nothing above them.

## 3. Official versioning/release policy

The pre-1.0 "minor bump = breaking change" convention is confirmed from three angles:

1. **The Cargo spec itself** (not egui-specific, but what makes the `"0.36.2"` requirement behave this way): the [Cargo Reference — SemVer Compatibility](https://doc.rust-lang.org/cargo/reference/semver.html) states that "Initial development releases starting with `0.y.z` can treat changes in `y` as a major release, and `z` as a minor release… This is because Cargo uses the convention that only changes in the left-most non-zero component are considered incompatible." The [Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) page's caret-requirement table gives the concrete mapping `0.2.3 := >=0.2.3, <0.3.0`, i.e. for `"0.36.2"` the resolved range is `>=0.36.2, <0.37.0`.
2. **egui's own README**, under the `## State` heading: "egui is in active development. It works well for what it does, but it lacks many features and the interfaces are still in flux. **New releases will have breaking changes.**" — [`README.md`, line 116](https://raw.githubusercontent.com/emilk/egui/master/README.md). This is a general statement (doesn't explicitly say "every minor release"), but combined with the observed changelog history (every `X.Y.0` release for years has contained breaking/deprecating changes; patch releases like `0.36.1`/`0.36.2` do not) it matches the Cargo 0.x convention exactly.
3. **egui's `CONTRIBUTING.md`**, under `## PR review`: "Note that each new egui release have some breaking changes, so we don't mind having a few of those in a PR. Of course, we still try to avoid them if we can, and if we can't we try to first deprecate old code using the `#[deprecated]` attribute." — [`CONTRIBUTING.md`, line 104](https://raw.githubusercontent.com/emilk/egui/master/CONTRIBUTING.md).

I could not find an explicit sentence in the repo stating "we do not follow SemVer" verbatim (no hit for "SemVer" itself in `README.md` or `CONTRIBUTING.md`), so that specific phrasing should be treated as an inference from the above rather than a direct quote — but the practical behavior (minor-version-per-breaking-release) is directly corroborated by CONTRIBUTING.md and by the changelog contents in §4 below.

**MSRV**: the workspace [`Cargo.toml`](https://raw.githubusercontent.com/emilk/egui/master/Cargo.toml) `[workspace.package]` section sets `rust-version = "1.95"` (bumped from 1.92 in the 0.36.0 release per the changelog — see §4), and `edition = "2024"`.

## 4. What changed 0.35.x → 0.36.x, and is this project's `App` trait signature current?

Per the [top-level `CHANGELOG.md`](https://raw.githubusercontent.com/emilk/egui/master/CHANGELOG.md) (`## 0.36.0 - 2026-08-05`, lines 37–91) and [`crates/eframe/CHANGELOG.md`](https://raw.githubusercontent.com/emilk/egui/master/crates/eframe/CHANGELOG.md) (`## 0.36.0 - 2026-08-05`, lines 21–33), the `0.35.0 → 0.36.0` bump's notable changes were:

- Mobile keyboard/IME improvements for eframe web ([#8045](https://github.com/emilk/egui/pull/8045)).
- Drag-to-reopen collapsed panels; window decoration theme now syncs with app theme.
- **Removed** `Modifiers` from `RawInput` (now an `egui::Event`) ([#8336](https://github.com/emilk/egui/pull/8336)) and removed `clip_rect_margin` ([#8366](https://github.com/emilk/egui/pull/8366)) — both are breaking API removals.
- MSRV raised from Rust 1.92 to 1.95 ([#8348](https://github.com/emilk/egui/pull/8348)).
- eframe: web text-input robustness, `webbrowser` dependency made optional, `DroppedFile` now stores a `web_sys::File`.

**No changes to the `eframe::App` trait's `update`/`ui` method signature occurred in 0.36.0.** (An earlier automated summarization pass mis-attributed this change to 0.36.0; I re-verified against the raw changelog text and that claim was incorrect — flagging it here since the task specifically asked to verify this rather than assume.)

**The actual `App::update` → `App::ui` breaking change happened two minor releases earlier, in `0.34.0` (2026-03-26)**, under the "More `Ui`, less `Context`" section of the top-level changelog:

> "In `eframe` we've deprecated `App::update` replaced it with `App::ui` (which provides a `&mut Ui` instead of a `&Context`)." — [`CHANGELOG.md`, line 258](https://raw.githubusercontent.com/emilk/egui/master/CHANGELOG.md), with the corresponding PR entry: "Replace `App::update` with `fn logic` and `fn ui`" ([#7775](https://github.com/emilk/egui/pull/7775), `CHANGELOG.md` line 266 / `crates/eframe/CHANGELOG.md` line 79).

**This project's code already uses the current signature.** `src/bin/gui.rs` (line 137–138 in this repo) has:
```rust
impl eframe::App for CalculatorApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
```
This is `App::ui(&mut self, ui: &mut egui::Ui, frame: &mut Frame)`, exactly the signature introduced in `0.34.0` and still current in `0.36.2` — **not** the older, deprecated `fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame)` form. Since the pin `"0.36.2"` (`>=0.36.2, <0.37.0`) is entirely within the post-`0.34.0` API era, this project's code is compatible with everything the current pin can resolve to, and would need no `App` trait changes to move within the `0.36.x` line.

## 5. Recommendation

**Cargo's own guidance**: the [Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) reference explains that a bare version string like `"0.36.2"` is a caret requirement and is the *recommended default* way to specify dependencies — for a 0.x crate this already resolves to `>=0.36.2, <0.37.0`, i.e. Cargo's own SemVer-compatibility convention (see [semver.html](https://doc.rust-lang.org/cargo/reference/semver.html), quoted in §3) treats the minor version as the "breaking" component for 0.x crates and already locks it. In other words, **for a 0.x dependency, the plain `"0.36.2"` string is not a loose/dangerous pin — it's Cargo's standard, idiomatic way of saying "this minor version's API, patch-updates only,"** which is exactly the granularity at which egui/eframe declare breaking changes (§3, §4).

Given that:

- `0.36.2` is a real, current, non-yanked, latest release for both crates (§1–§2).
- egui/eframe's own contribution guidelines confirm minor releases routinely carry breaking changes, so allowing automatic upgrades past `0.37.0` would be unsafe without review (§3).
- Patch releases (`0.36.1`, `0.36.2` per the changelogs) contain only fixes/small additions, not breaking changes, so allowing patch-level auto-updates (which the current pin does) is safe and desirable (bug fixes for free).
- The project's `App` implementation already matches the current (`0.34.0`+) trait shape, so there's no pending forced migration (§4).

**Recommendation: keep `"0.36.2"` as-is.** It is already the standard, ecosystem-idiomatic pinning strategy for a pre-1.0 crate — equivalent to `>=0.36.2, <0.37.0` — and does not need to be tightened to `=0.36.2`. An exact pin (`=0.36.2`) would only be warranted if the project needed hermetic, byte-for-byte reproducible builds independent of `Cargo.lock` (e.g. publishing a library where consumers might not use the lockfile) or if a specific patch release were known to be buggy; neither applies here, this is a binary application where `Cargo.lock` (already present and checked into the repo per the initial `ls`) is the actual reproducibility mechanism, and pinning exactly would forgo painless bug-fix patch updates for no benefit. Loosening the requirement (e.g. to a range spanning `0.35`–`0.37`) would be actively worse, since it would let Cargo resolve into a minor version egui itself documents as containing breaking changes.

There is also **no newer version to bump to** — `0.36.2` is already latest-stable for both crates as of today, so no version bump action is needed at all.
