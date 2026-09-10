# Inter Font Embedding — Fact-Finding Notes

Research date: 2026-09-10. Follow-up to `docs/research/macos-style-ui.md` (§1.2/§3.3), which already
settled that SF Pro cannot legally be bundled and that Inter (OFL 1.1) is the embeddable alternative.
This doc just nails down the mechanics.

## 1. Authoritative distribution

Canonical source: Inter's own GitHub repo, [`rsms/inter`](https://github.com/rsms/inter). Latest
**stable** release tag is **`v4.1`** (published 2024-11-16) —
[`github.com/rsms/inter/releases/tag/v4.1`](https://github.com/rsms/inter/releases/tag/v4.1). Google
Fonts (fonts.google.com/specimen/Inter) mirrors the same OFL font but only distributes it as a
variable font, not as the discrete static+TTC+woff2 bundle the GitHub release ships — so for this
project, the GitHub release is the better primary source.

The release has exactly one asset: `Inter-4.1.zip` (33,707,794 bytes), confirmed via
`gh`/GitHub API (`api.github.com/repos/rsms/inter/releases/tags/v4.1`):
**direct download URL: `https://github.com/rsms/inter/releases/download/v4.1/Inter-4.1.zip`**.
Inside the zip, the relevant files live at `extras/ttf/Inter-<Weight>.ttf` (plain static TTFs —
confirmed present by unzipping and listing contents), plus a root-level `InterVariable.ttf` (variable
font, 879,708 bytes) and `Inter.ttc` (a TrueType Collection bundling *all* static weights+styles into
one 13.2 MB file — not useful here, egui wants a single font file per registered name, not a
collection). There is no way to fetch a single static TTF file directly without downloading the
whole 33.7 MB zip — the release doesn't expose per-file assets.

**Format choice — corrected from the original premise**: egui 0.36.2 does *not* actually require a
single-instance (static) TTF. `epaint::text::FontData` is parsed by **`skrifa`** (not `ab_glyph`/
`ttf-parser` — confirmed via `FontData::variation_axes()`'s `use skrifa::MetadataProvider` at
`epaint-0.36.2/src/text/fonts.rs:154`), and skrifa supports variable fonts natively: `FontData` has a
`variation_axes()` method and `FontTweak.coords: VariationCoords`
(`epaint-0.36.2/src/text/fonts.rs:145-173,249-250`) that lets you pick a point on a variable font's
`wght` axis per registered `FontData`. So embedding `InterVariable.ttf` once and registering it twice
under two names with different `coords` (e.g. `wght=400` and `wght=600`) is a real option, not just
static files. That said, static TTFs are still the simpler and marginally smaller choice for this
project (see §4) — no axis-coordinate tuning, no risk of picking a coordinate outside the font's valid
range, and `Inter-Regular.ttf` alone (402 KB) is far smaller than `InterVariable.ttf` (859 KB) if only
one weight is needed. WOFF2 files are **not** usable directly — `FontData` expects raw `.ttf`/`.otf`
bytes (doc comment at `fonts.rs:109,113`: "A `.ttf` or `.otf` file"); WOFF2 is a compressed wrapper
skrifa doesn't unwrap.

## 2. Which weights are actually needed

Checked `.strong()` directly in the local egui-0.36.2 source (since the "=" button currently calls
`.strong()` at `src/bin/gui.rs:252`):

- `RichText::strong()` — `egui-0.36.2/src/widget_text.rs:252-255`: `self.strong = true; self` — doc
  comment literally says **"Extra strong text (stronger color)."**
- That flag is consumed only in color resolution: `widget_text.rs:484-485` —
  `} else if self.strong { Some(visuals.strong_text_color()) }` — and line 414 confirms
  `strong: _, // already used by get_text_color` (i.e. it never touches font selection/weight).

**Conclusion: `.strong()` is purely a text-color modifier (picks `visuals.strong_text_color()`), not a
font-weight change.** It renders in the exact same font file as everything else. So no bold/second
font file is *required* for the existing `.strong()` call to keep working exactly as it does today.

For the broader UI: **Inter Regular is sufficient** for all button labels and body text. A Medium or
SemiBold weight is optional, only useful if you want the "=" button (or other emphasis) to become
*visually bolder in shape*, not just color — which `.strong()` alone doesn't give you. If that's
wanted, grab `Inter-SemiBold.ttf` (matches Apple's own "Headline" style being Bold/Semibold per the
prior research doc's §1.2 table) and register it as a second named font, applied via a custom
`TextStyle`/explicit font family on that one button — `.strong()` itself doesn't need it.

## 3. OFL 1.1 license — verbatim, from Inter's own release

`LICENSE.txt` inside `Inter-4.1.zip` (4,380 bytes) is the standard SIL OFL 1.1 text. The operative
bundling clause, quoted verbatim:

> "2) Original or Modified Versions of the Font Software may be bundled, redistributed and/or sold
> with any software, provided that each copy contains the above copyright notice and this license.
> These can be included either as stand-alone text files, human-readable headers or in the
> appropriate machine-readable metadata fields within text or binary files as long as those fields
> can be easily viewed by the user."

And from the preamble: "The fonts, including any derivative works, can be bundled, embedded,
redistributed and/or sold with any software provided that any reserved names are not used by
derivative works." Practical upshot for this project: embedding `Inter-Regular.ttf` via
`include_bytes!` inside the compiled binary is permitted — the only obligation is to ship the
copyright notice + license text alongside the distribution (e.g. a `LICENSE-Inter.txt` or a line in
the project's own license/notices file), which can be a "machine-readable metadata field" per the
clause — doesn't have to be a loose file, but a loose file is the simplest compliant approach.

## 4. File sizes (from unzipping `Inter-4.1.zip` and listing contents)

| File | Size |
|---|---|
| `extras/ttf/Inter-Regular.ttf` | 411,640 bytes ≈ **402 KB** |
| `extras/ttf/Inter-Medium.ttf` | 417,300 bytes ≈ **408 KB** |
| `extras/ttf/Inter-SemiBold.ttf` | 419,744 bytes ≈ **410 KB** |
| `InterVariable.ttf` (variable, all weights) | 879,708 bytes ≈ **859 KB** |

Regular alone: ~402 KB added to the binary. Regular + SemiBold (two static files, if emphasis needs a
real bold face rather than just `.strong()`'s color change): ~812 KB. That's roughly the same order as
just embedding the single variable font (~859 KB) and deriving both weights from it via
`FontTweak.coords` — so the "one variable file vs. two static files" choice is close to a wash on
size; static files win slightly and are simpler to reason about if only one or two fixed weights are
ever needed.

## 5. Embedding mechanics — egui 0.36.2 exact API

Confirmed in `~/.cargo/registry/src/index.crates.io-*/egui-0.36.2/src/context.rs:2106` (`set_fonts`,
replaces all fonts) and `:2129` (`add_font`, additive) — both on `egui::Context`. `FontDefinitions` is
at `epaint-0.36.2/src/text/fonts.rs:431` (`font_data: BTreeMap<String, Arc<FontData>>`,
`families: BTreeMap<FontFamily, Vec<String>>`); `FontData::from_static`/`from_owned` at
`fonts.rs:125,133`. Canonical pattern (matches egui's own `examples/custom_font/src/main.rs` in the
`emilk/egui` repo, adapted to this project):

```rust
fn install_inter_font(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "Inter-Regular".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/fonts/Inter-Regular.ttf"
        ))),
    );
    fonts.families.entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Inter-Regular".to_owned()); // put Inter first = default proportional font
    ctx.set_fonts(fonts);
}
```

Call site: inside the `run_native` startup closure, alongside the existing
`cc.egui_ctx.set_visuals(...)` call — `src/bin/gui.rs:35-38`:

```rust
Box::new(|cc| {
    install_inter_font(&cc.egui_ctx);
    cc.egui_ctx.set_visuals(egui::Visuals::dark());
    Ok(Box::new(CalculatorApp::default()))
}),
```

Using `insert(0, ...)` (not `push`) is required — egui tries fonts in list order per family and falls
back down the list for glyphs the first font lacks, so Inter must be first to actually take effect for
ASCII/Latin text instead of staying behind the default `Ubuntu-Light`.
