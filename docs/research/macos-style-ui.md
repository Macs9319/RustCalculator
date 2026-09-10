# Adopting macOS Design Conventions in the egui/eframe Calculator UI

Research date: 2026-09-10.

Context: `src/bin/gui.rs` implements a hand-rolled dark theme (explicit `Color32` constants,
manual per-state `WidgetVisuals`, `CORNER_RADIUS = 10`, fixed `BUTTON_SIZE`, a painter-drawn
display readout) on top of `eframe`/`egui` `0.36.2`. This document investigates what it would
take to make that UI *read as macOS-native* — not a reimplementation of AppKit — against Apple's
own Human Interface Guidelines (HIG) and against what `egui`/`eframe` `0.36.2` can actually do,
verified from the crate source cached locally at
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/{egui,eframe,epaint,epaint_default_fonts}-0.36.2/`.

Apple's HIG site (`developer.apple.com/design/human-interface-guidelines/...`) is a client-rendered
app; plain `curl`/`WebFetch` only returns the `<title>` tag. The page content was instead pulled
from the same JSON data endpoint the site's own JS uses,
`https://developer.apple.com/tutorials/data/design/human-interface-guidelines/<page>.json`
(confirmed working via `curl`, HTTP 200, e.g. for `materials`, `color`, `typography`, `layout`,
`windows`, `buttons`, `dark-mode`). This is Apple's own served content for those exact page URLs,
not a third-party mirror; citations below link to the human-facing page URL that this JSON backs.

---

## 1. Apple's Human Interface Guidelines — concrete, citable specifics

### 1.1 Corner radius conventions

**Apple's current HIG text does not publish a literal point value for button or window corner
radius.** This was checked directly and is worth stating plainly rather than guessing:

- The [Buttons page](https://developer.apple.com/design/human-interface-guidelines/buttons),
  under "macOS → Push buttons," says only that a flexible-height push button "support[s] the same
  configurations as regular push buttons — and they use **the same corner radius** and content
  padding — so they look consistent with other buttons in your interface." No number is given.
- The [Windows page](https://developer.apple.com/design/human-interface-guidelines/windows),
  under "macOS window anatomy," describes the frame/body/title-bar structure but states no corner
  radius value for the window itself.
- Neither the [Layout page](https://developer.apple.com/design/human-interface-guidelines/layout)
  nor the [Materials page](https://developer.apple.com/design/human-interface-guidelines/materials)
  states a corner-radius number either.

This absence is itself informative: Apple's HIG has moved to describing controls in terms of the
dynamic "Liquid Glass" material (introduced with the redesign documented as of the
Materials page's changelog entries "June 9, 2025 — Added guidance for Liquid Glass" and
"September 9, 2025 — Updated guidance for Liquid Glass") rather than fixed pixel/point specs, so
there is no current Apple-stated single "macOS button corner radius" number to cite. Any specific
number (6pt, 8pt, 10pt, etc.) used in practice is an inference from observing shipped apps, not an
HIG citation — flagged here so this isn't presented as a fact it isn't.

For comparison (not a HIG citation, a codebase fact): `egui`'s own **default** dark/light `Visuals`
use `window_corner_radius: CornerRadius::same(6)` and `menu_corner_radius: CornerRadius::same(6)`,
with most individual widget states (`noninteractive`, `inactive`, `hovered`, `active`) defaulting to
`CornerRadius::same(2)` or `same(3)` — see
`egui-0.36.2/src/style.rs:1519`, `:1530`, and `:1688`–`:1765`. The project's current
`CORNER_RADIUS = 10` (`src/bin/gui.rs:19`) is visibly rounder than egui's own defaults, though
neither is an Apple-sourced number.

### 1.2 Typography — SF Pro, licensing, and alternatives

**System font**: "SF Pro is the system font in macOS" —
[Typography page](https://developer.apple.com/design/human-interface-guidelines/typography),
"Platform considerations → macOS." San Francisco (SF) is described as "a sans serif typeface
family that includes the SF Pro, SF Compact, SF Arabic, SF Armenian, SF Georgian, SF Hebrew, and
SF Mono variants" (same page, "Using system fonts"). New York (NY) is the companion serif family.

**Default/minimum text size on macOS**, from the same page's "Ensuring legibility" table:

| Platform | Default size | Minimum size |
|---|---|---|
| macOS | 13 pt | 10 pt |

And the macOS built-in text-style table (same page, "Platform considerations → macOS →
macOS built-in text styles") gives, among others: Body = 13pt/16pt line height (Regular, Semibold
emphasized), Headline = 13pt/16pt (Bold), Footnote/Caption = 10pt/13pt. These are Apple's own
stated point sizes for macOS, directly usable as a reference scale.

**Explicit instruction not to embed the system font**: "You can use the constants defined in
`SwiftUI/Font/Design` to access all system fonts — **don't embed system fonts in your app or
game**" (Typography page, "Using system fonts," note callout). This is Apple telling *its own*
developers not to bundle SF Pro as a font file even when targeting Apple platforms — you're
meant to reference it by system API, not ship the font.

**Licensing — verified directly against `developer.apple.com/fonts/`** (fetched as static HTML,
containing the actual "Apple SF Pro Font License Agreement" text, dated `EA1370`, `2/24/2016`):

> "2. Permitted License Uses and Restrictions. A. **Limited License.** Subject to the terms of this
> License, you may use the Apple Font solely for creating mock-ups of user interfaces to be used in
> software products running on Apple's iOS, OS X or tvOS operating systems..."
>
> "B. **Other Use Restrictions.** The grants set forth in this License do not permit you to, and
> you agree not to, install, use or run the Apple Font for the purpose of creating mock-ups of user
> interfaces to be used in software products running on **any non-Apple operating system** or to
> enable others to do so. **You may not embed the Apple Font in any software programs or other
> products.**"

— [`https://developer.apple.com/fonts/`](https://developer.apple.com/fonts/) (license text embedded
directly in that page's HTML).

**Conclusion: SF Pro cannot be legally bundled with this project.** The license (a) restricts even
*mock-up* use to Apple-OS-targeted software, (b) separately and explicitly forbids embedding the
font in any software product at all, and (c) this project's binary runs on Linux (per the
environment this repo is developed in) and is a general-purpose distributable app, not an Apple-
platform mock-up tool. There is no reading of this license under which shipping `SFPro.ttf` inside
the calculator's binary or asset folder is permitted.

**Royalty-free alternative — Inter**: Inter is released under the **SIL Open Font License 1.1**,
per its own site: "Inter is a free and open source font family. You are free to use this font in
almost any way imaginable." — [`https://rsms.me/inter/`](https://rsms.me/inter/), full license text
at [`https://github.com/rsms/inter/blob/v4.1/LICENSE.txt`](https://raw.githubusercontent.com/rsms/inter/v4.1/LICENSE.txt).
Inter is not an Apple recommendation — it is independently and commonly cited as the closest
freely-licensed metric/visual analogue to SF Pro (grotesque sans, tall x-height, designed for UI
text at small sizes), but that similarity claim is a design observation, not something Apple's HIG
states. It is a legitimate embeddable choice under OFL 1.1, unlike SF Pro.

**What `egui` ships by default** (not SF Pro, not Inter): the `epaint_default_fonts` crate embeds
`Ubuntu-Light.ttf` as the default proportional font and `Hack-Regular.ttf` as the default monospace
font, plus `NotoEmoji-Regular.ttf` / `emoji-icon-font.ttf` for emoji — see
`epaint_default_fonts-0.36.2/src/lib.rs:16,27,36,47` and the accompanying `fonts/` directory. So
out of the box, this project's UI is rendering in Ubuntu (proportional) — which is why it does not
currently read as macOS at all typographically, independent of color/shape choices.

### 1.3 Color system — macOS semantic colors

From the [Color page](https://developer.apple.com/design/human-interface-guidelines/color),
"System colors" section: "Avoid hard-coding system color values in your app... Each dynamic color
is semantically defined by its purpose, rather than its appearance or color values." The
"Platform considerations → macOS" section gives the full table of AppKit `NSColor` dynamic system
colors (name → purpose → API), including the exact ones the task asked about:

| Color | Use for… | AppKit API |
|---|---|---|
| Label color | The text of a label containing primary content. | `NSColor.labelColor` |
| Secondary label color | Text of lesser importance than a primary label. | `NSColor.secondaryLabelColor` |
| Tertiary label color | Text of lesser importance than a secondary label. | `NSColor.tertiaryLabelColor` |
| Window background color | The background of a window. | `NSColor.windowBackgroundColor` |
| Control background color | The background of a large interface element, such as a browser or table. | `NSColor.controlBackgroundColor` |
| Control color | The surface of a control. | `NSColor.controlColor` |
| Control accent | The accent color people select in System Settings. | `NSColor.controlAccentColor` |
| Separator color | A separator between different sections of content. | `NSColor.separatorColor` |
| Selected content background color | Background for selected content in a key window/view. | `NSColor.selectedContentBackgroundColor` |
| Text color / Text background color | Document text / background behind text. | `NSColor.textColor` / `NSColor.textBackgroundColor` |

(Full table: 27 named colors — see the page for the complete list, e.g. also
`disabledControlTextColor`, `findHighlightColor`, `gridColor`, `keyboardFocusIndicatorColor`.)

Apple does not publish literal RGB hex values for these on the documentation page itself ("Avoid
hard-coding system color values in your app. Documented color values are for your reference during
the app design process. The actual color values may fluctuate from release to release"), so there
is no primary-sourced hex table to copy — only the semantic names/roles above, which is itself the
point: these are meant to be looked up at runtime on macOS, not hard-coded.

**Light/dark adaptation** is confirmed by the same page: "iOS, iPadOS, macOS, and visionOS also
define sets of dynamic system colors that match the color schemes of standard UI components and
automatically adapt to both light and dark contexts," and by the
[Dark Mode page](https://developer.apple.com/design/human-interface-guidelines/dark-mode):
"Semantic colors (like `NSColor.labelColor` and `NSColor.controlColor` in macOS...) automatically
adapt to the current appearance... At a minimum, make sure the contrast ratio between colors is no
lower than 4.5:1. For custom foreground and background colors, strive for a contrast ratio of
7:1, especially in small text."

**"Desktop tinting"** — a macOS-only, non-obvious detail from the Dark Mode page's macOS section:
"When people choose the graphite accent color in General settings, macOS causes window backgrounds
to pick up color from the current desktop picture. The result — called desktop tinting — is a
subtle effect... Include some transparency in custom component backgrounds when appropriate."
This is explicitly Apple recommending a translucency behavior that is out of scope for a hard-coded
`Color32` palette (see §2.2 for whether egui can achieve it at all).

### 1.4 Materials/vibrancy — precisely what they are

From the [Materials page](https://developer.apple.com/design/human-interface-guidelines/materials):
"A material is a visual effect that creates a sense of depth, layering, and hierarchy between
foreground and background elements... By allowing color to pass through from background to
foreground, a material establishes visual hierarchy."

For macOS specifically ("Platform considerations → macOS"): "macOS provides several standard
materials with designated purposes, and vibrant versions of all system colors. For developer
guidance, see `AppKit/NSVisualEffectView/Material`... macOS defines two modes that blend background
content: **behind window** and **within window**. For developer guidance, see
`AppKit/NSVisualEffectView/BlendingMode`."

Also directly relevant to a small non-resizable utility window: the
[Windows page](https://developer.apple.com/design/human-interface-guidelines/windows), "macOS
window states," notes materials are tied to window *activation* state: "inactive windows don't use
materials (an effect that can pull color into a window from the content underneath it), which makes
them appear subdued and seem visually farther away than the main and key windows." So real AppKit
vibrancy isn't just a static blur — it changes behavior when the window loses focus, something a
hand-rolled static fill cannot replicate regardless of the rendering backend.

This is the material Apple's own `NSVisualEffectView`/`hudWindow`/`sidebar` styles implement at
the AppKit layer — see §2.2 for whether `egui`/`eframe` can drive the actual `NSVisualEffectView`
compositor effect (answer: not directly, but see the `window-vibrancy` crate finding).

### 1.5 Spacing/layout grid conventions

**Apple's HIG does not state a general-purpose "8pt grid" rule** for window/control layout in the
pages checked (Layout, Buttons). This is worth being explicit about since an "8pt grid" is a very
commonly repeated *secondary-source* claim about Apple design that this research did not find
stated as such in the primary source. What the HIG *does* state, concretely:

- General hit-region guidance (Buttons page, "Best practices," not macOS-specific but stated as a
  general rule): "a button needs a hit region of at least **44x44 pt** — in visionOS, 60x60 pt — to
  ensure that people can select it easily."
- tvOS-specific grid tables exist with exact pt values (e.g. "Two-column grid: Horizontal spacing
  40 pt, Minimum vertical spacing 100 pt") — Layout page, "Platform considerations → tvOS → Grids" —
  but these are stated as tvOS-only, not general macOS guidance, and should not be borrowed as a
  macOS spacing standard.
- macOS-specific layout guidance in the Layout page is qualitative, not numeric: "Avoid placing
  controls or critical information at the bottom of a window. People often move windows so that the
  bottom edge is below the bottom of the screen." No macOS spacing-unit number is given.

So: no primary-sourced macOS spacing grid value exists to cite. The only numeric, cross-platform
constant found is the 44×44pt minimum hit target.

### 1.6 Standard control heights/sizing for buttons

**No macOS push-button height in points is stated in the current Buttons page.** The page instead
describes macOS button *types* qualitatively (push buttons, flexible-height push buttons, square
buttons, help buttons, image buttons) without a height table. The only numeric control-sizing table
in the Buttons page at all is for **visionOS** button shapes (Mini 28pt / Small 32pt / Regular 44pt
/ Large 52pt / Extra large 64pt) — explicitly visionOS, not transferable to macOS as an HIG claim.

One numeric macOS detail that *is* stated: **image buttons** should "Include about **10 pixels** of
padding between the edges of the image and the button edges" (Buttons page, macOS → Image buttons).

The general 44×44pt minimum hit-region rule from §1.5 is the most defensible numeric floor to apply
to macOS buttons as well, since it's stated as a cross-platform "as a general rule," not gated to a
specific platform.

---

## 2. What `egui`/`eframe` 0.36.2 can actually do

Verified against the source cached at
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/` (`egui-0.36.2`, `eframe-0.36.2`,
`epaint-0.36.2`, `epaint_default_fonts-0.36.2`), which was present locally from this repo's prior
`cargo build` (no network fetch needed for this part).

### 2.1 Custom font loading — supported

`egui::Context` exposes `pub fn set_fonts(&self, font_definitions: FontDefinitions)` —
`egui-0.36.2/src/context.rs:2106`. Doc comment: "Tell `egui` which fonts to use... you can call
this to install additional fonts... This will overwrite the existing fonts." There's also an
additive `pub fn add_font(&self, new_font: FontInsert)` (`context.rs:~2127`) to add a font without
replacing everything.

`FontDefinitions` (`epaint-0.36.2/src/text/fonts.rs:431`) is a plain struct:
```rust
pub struct FontDefinitions {
    pub font_data: BTreeMap<String, Arc<FontData>>,
    pub families: BTreeMap<FontFamily, Vec<String>>,
}
```
`FontData` wraps raw TTF/OTF bytes. This confirms: yes, loading a custom `.ttf`/`.otf` (e.g. an
`Inter-Regular.ttf` bundled via `include_bytes!`) and calling
`cc.egui_ctx.set_fonts(font_definitions)` inside the `run_native` closure (where this project
already calls `cc.egui_ctx.set_visuals(...)`, `src/bin/gui.rs:36`) is the standard, supported path —
no crate-behavior surprises here.

### 2.2 Window vibrancy/translucency — plain alpha transparency only; true blur needs a third-party crate

`egui::viewport::ViewportBuilder` has `pub transparent: Option<bool>` and
`pub fn with_transparent(mut self, transparent: bool) -> Self`
(`egui-0.36.2/src/viewport.rs:306,424`). Its doc comment is explicit about what this actually is:

> "Sets whether the background of the window should be transparent... In `eframe` you control the
> transparency with `eframe::App::clear_color()`. If this is `true`, writing colors with alpha
> values different than `1.0` will produce a transparent window... **macOS:** When using this
> feature to create an overlay-like UI, you likely want to combine this with
> `Self::with_has_shadow` set to `false` in order to avoid ghosting artifacts."

This is **plain alpha-channel window transparency** (letting the desktop/whatever is behind the
window show through proportional to your draw calls' alpha) — it is not a blur, and not
`NSVisualEffectView` vibrancy. Searching the entire `egui-0.36.2` source tree for `blur`,
`vibrancy`, and `NSVisualEffect` returns **zero matches** — confirmed with
`grep -rn "blur\|vibrancy\|NSVisualEffect" egui-0.36.2/src/viewport.rs` (and the rest of the crate).
`eframe`'s own source has no macOS-specific compositor-blur code either, beyond the window-chrome
handling described below. So: out of the box, `eframe::NativeOptions`/`ViewportBuilder` cannot
produce the blurred, content-aware `NSVisualEffectView` look Apple's own apps use — only see-through
alpha.

**However, true vibrancy is reachable via a third-party crate, and the plumbing lines up
version-for-version with this project's dependency tree:**

- `eframe::Frame` implements `raw_window_handle::HasWindowHandle`
  (`eframe-0.36.2/src/epi.rs:17-19,92,102,688,702`), i.e. `Frame::window_handle()` returns a real
  `WindowHandle` wrapping the OS-native handle (on macOS, an `AppKitWindowHandle` — confirmed by
  `eframe-0.36.2/src/native/macos.rs:1-3`, which already imports
  `raw_window_handle::{AppKitWindowHandle, RawWindowHandle}` and `objc2_app_kit::{NSView, NSWindow, ...}`
  for its own window-chrome-metrics feature).
- `eframe-0.36.2` pins `raw-window-handle = "0.6.2"` (this project's own `Cargo.lock`).
- The `window-vibrancy` crate (`https://crates.io/crates/window-vibrancy`, latest stable `0.8.0`,
  repo `github.com/tauri-apps/tauri-plugin-vibrancy`) depends on `raw-window-handle ^0.6` and lists
  `winit ^0.30` / `tao ^0.30` as its supported windowing toolkits in its `Cargo.toml` dependency
  metadata — matching `eframe-0.36.2`'s own pinned `winit = "0.30.13"` exactly. Per its docs
  (docs.rs), on macOS it calls into real `NSVisualEffectView` APIs via `objc2-app-kit`, exposing
  `apply_vibrancy(&window, NSVisualEffectMaterial::..., state, radius)`.

So the honest answer is two-layered: **`eframe`/`egui` alone cannot do real vibrancy — only alpha
transparency** — but because `Frame` exposes a standard `HasWindowHandle`, a third-party crate
(`window-vibrancy`) that targets that same handle trait and the same `raw-window-handle`/`winit`
versions *can* be wired in at the app level to call real `NSVisualEffectView` materials on macOS.
This would require the app to grab the window handle (likely via `Frame` in `update`/`ui`, once
per window creation) and call `window-vibrancy`'s macOS function — extra integration work and an
extra dependency, not something `NativeOptions` exposes as a flag.

### 2.3 Corner-radius API — `CornerRadius`, confirmed

`epaint::CornerRadius` (re-exported as `egui::CornerRadius`, already used in this project at
`src/bin/gui.rs:19,116` etc.) is defined at `epaint-0.36.2/src/corner_radius.rs:13-25`:
```rust
pub struct CornerRadius {
    pub nw: u8,
    pub ne: u8,
    pub sw: u8,
    pub se: u8,
}
```
i.e. four independent per-corner `u8` radii (0–255), with `CornerRadius::same(n)` as a convenience
constructor (used throughout the project already). There's also `CornerRadiusF32` for sub-pixel
radii used internally by tessellation (`corner_radius_f32.rs:8-20`), but the public widget-styling
API is the integer `u8` version this project already uses — no gap here.

As noted in §1.1, egui's own default `Visuals::dark()`/`light()` use `CornerRadius::same(6)` for
windows/menus and `same(2)`–`same(3)` for individual widget states
(`egui-0.36.2/src/style.rs:1519,1530,1688-1765`) — meaningfully tighter than this project's
`CORNER_RADIUS = 10`.

### 2.4 `egui::Visuals` and OS light/dark detection — built in, but currently overridden by this project

`egui` ships `Theme` (`Dark`/`Light`) and `ThemePreference` (`Dark`/`Light`/`System`, with `System`
as `#[default]`) in `egui-0.36.2/src/memory/theme.rs:6-77`. `Context::system_theme()` — "Does the
OS use dark or light mode? This is used when the theme preference is set to
`ThemePreference::System`" (`context.rs:2150-2154`) — reflects the *actual* detected OS theme.

That value is populated by `eframe` from **winit's own OS theme detection**: `eframe`'s native glow
and wgpu backends call `event_loop.system_theme()` when constructing `egui_winit::State`
(`eframe-0.36.2/src/native/glow_integration.rs:1317`, `wgpu_integration.rs:303,1084`), and the web
backend has its own `fn system_theme() -> Option<egui::Theme>` reading the browser's preference
(`eframe-0.36.2/src/web/mod.rs:126`). So **`eframe` auto-detects and continuously tracks the OS
light/dark preference for you** via winit's native platform integration (on macOS, this reads
`NSApplication`'s effective appearance) — no extra dependency needed.

**This project currently opts out of that**: `src/bin/gui.rs:36` calls
`cc.egui_ctx.set_visuals(egui::Visuals::dark())` once at startup, which pins the app to dark mode
permanently regardless of OS setting or later OS-level changes. Since egui's default
`ThemePreference` is already `System` (`memory/theme.rs:75`, `#[default]`), simply **not** calling
`set_visuals` — or calling `ctx.set_theme(egui::ThemePreference::System)` explicitly — combined
with supplying both a light and a dark `Visuals`/palette, would make the app automatically track
macOS's Appearance setting. This is directly relevant to the HIG requirement from §1.3 that "make
sure all your app's colors work well in light [and] dark... contexts."

---

## 3. Synthesis — concrete recommendations for `src/bin/gui.rs`

Everything below is scoped to what §1 and §2 actually verified; anything not traceable to those
sections is flagged as a design choice rather than an Apple- or egui-sourced fact.

1. **Stop hard-coding dark-only.** Remove (or make conditional) the
   `cc.egui_ctx.set_visuals(egui::Visuals::dark())` call at `src/bin/gui.rs:36`. Leaving
   `ThemePreference` at its default (`System`, per §2.4) and providing **both** a light and a dark
   variant of the current `Color32` constants (§1.3's HIG quote: "Even if your app ships in a
   single appearance mode, provide both light and dark colors") is the direct, cheapest way to
   satisfy the HIG's light/dark requirement and to make the app track the OS setting the way
   `eframe` already supports for free (§2.4).

2. **Recolor using macOS's semantic role list as a checklist, not literal hex values** (since none
   are published, per §1.3): map `APP_BG`→role of `windowBackgroundColor`, `DISPLAY_BG`/`NUM_BG`/
   `CTRL_BG`→role of `controlBackgroundColor`/`controlColor`, `TEXT`→`labelColor`,
   `TEXT_MUTED`→`secondaryLabelColor`, `DISPLAY_BORDER`→`separatorColor`, and `EQUALS_BG` (the
   accent/"=" button)→role of `controlAccentColor` (the user's chosen macOS accent color — pick one
   reasonable fixed blue as a stand-in, since this app can't read the live macOS accent-color
   preference). Provide a light-mode set and a dark-mode set for each, and switch the active set
   based on `ctx.style().visuals.dark_mode` (or by keeping two `Visuals`/two constant tables and
   selecting by `ctx.theme()`).

3. **Font**: do not attempt to bundle SF Pro — it is legally blocked for this use (§1.2, direct
   quote from `developer.apple.com/fonts/`: "you may not... use... the Apple Font for the purpose of
   creating mock-ups of user interfaces to be used in software products running on any non-Apple
   operating system... You may not embed the Apple Font in any software programs or other
   products"). Bundle **Inter** (SIL OFL 1.1, freely embeddable — §1.2) via `include_bytes!` and
   `egui::FontDefinitions`/`ctx.set_fonts(...)` (§2.1, concretely supported), replacing the current
   default `Ubuntu-Light` proportional / `Hack-Regular` monospace fonts egui ships
   (`epaint_default_fonts-0.36.2/src/lib.rs`). This alone will likely move the UI's "read" toward
   macOS more than any color change, since Ubuntu is visually the most distinctly non-Apple element
   currently in play. Use Apple's own macOS point-size table as a sizing reference (§1.2): body
   text ≈13pt, the calculator's current display uses `FontId::monospace(28.0)` which is well above
   Apple's "Large Title" (26pt) — reasonable for a hero display readout, no change needed there
   beyond swapping which font file backs "monospace."

4. **Corner radius**: there is no Apple-stated number to match (§1.1), so treat this as a design
   choice, not a compliance item. If the goal is "closer to egui's own dark-mode-default macOS-ish
   feel" as a reference point, egui's own defaults are notably tighter than this project's current
   value: `same(6)` for window-level chrome, `same(2)`–`same(3)` for individual widgets
   (`egui-0.36.2/src/style.rs:1519,1530,1688-1765`), versus this project's flat `CORNER_RADIUS = 10`
   applied to both the display and every button. Consider dropping button corner radius toward
   6–8 and keeping the display readout's radius separate (slightly larger radii read as more
   "card-like," smaller as more "control-like" — again a design call, not an HIG citation).

5. **Button size**: bump `BUTTON_SIZE` (`src/bin/gui.rs:17`, currently `54.0 x 42.0`) so the height
   meets the cross-platform 44×44pt minimum hit-region guidance quoted in §1.5/§1.6 — e.g.
   `54.0 x 44.0` — since 42pt is 2pt under Apple's own stated floor.

6. **Materials/vibrancy — be honest about the ceiling.** `eframe`/`egui` alone cannot produce real
   `NSVisualEffectView` blur/vibrancy; `ViewportBuilder::with_transparent` is plain alpha
   transparency only, confirmed by zero matches for "blur"/"vibrancy"/"NSVisualEffect" anywhere in
   the `egui-0.36.2` source (§2.2). A real vibrant background **is** technically reachable by adding
   the third-party `window-vibrancy` crate (0.8.0), which is version-compatible with this project's
   pinned `eframe`/`winit`/`raw-window-handle` stack and can be driven off `eframe::Frame`'s
   `HasWindowHandle` implementation (§2.2) — but this is nontrivial extra integration (grabbing the
   raw AppKit window handle once the window exists, calling into `objc2-app-kit`-backed FFI, macOS-
   only code path with no effect on other platforms) and is not recommended as a first step. Given
   this is a small, always-on-top-of-its-content utility window (278×400, non-resizable) rather than
   a sidebar/content-browsing app, the HIG materials guidance's own primary use case (translucent
   panels that let scrolled content show through, per §1.4) doesn't strongly apply here anyway —
   recommend treating real vibrancy as a "nice-to-have, not needed for this app's shape" rather than
   a blocking gap.

7. **Window chrome — leave it alone.** The Windows page is explicit: "**Avoid creating custom
   window UI.** System-provided windows look and behave in a way that people understand and
   recognize... Doing so without perfectly matching the system's look and behavior can make your app
   feel broken" (§1, Windows page, "Best practices"). This project already uses default OS window
   decorations (no custom `with_decorations(false)` / painted title bar), which is already the HIG-
   correct choice for a utility app like this — nothing to change here, and no motivation from the
   HIG to add custom traffic-light-style chrome.

### What definitely cannot be achieved (repeated for clarity)

- **Bundling the literal SF Pro font file** — blocked by Apple's font license (§1.2), not an
  `egui` limitation.
- **Real compositor-level window blur/vibrancy via `eframe`/`egui` APIs alone** — not implemented
  anywhere in the `egui-0.36.2` source (§2.2); only reachable via the third-party `window-vibrancy`
  crate and manual AppKit FFI plumbing, which is out of scope for a first styling pass.
- **Live-reading the user's chosen macOS accent color** (System Settings → Accent color) — nothing
  in the `egui`/`eframe` source exposes this; only the light/dark OS theme is exposed
  (`Context::system_theme()`, §2.4). A fixed "systemBlue"-like stand-in is the practical substitute.
