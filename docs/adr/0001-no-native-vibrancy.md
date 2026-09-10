# macOS-style restyle stops at colors/fonts/sizing — no native vibrancy

The GUI is being restyled toward macOS conventions (see `docs/research/macos-style-ui.md`). Real `NSVisualEffectView` vibrancy/blur is achievable via the third-party `window-vibrancy` crate (confirmed version-compatible with our `eframe`/`winit` pins), but we decided against it: it's disproportionate effort for a 278×400 utility window, and — since the primary dev machine here is Linux, not macOS — it can be compiled but never actually visually verified locally. The restyle is scoped to a theme-aware color palette, an embedded Inter font, and HIG-derived button sizing instead.

If a contributor develops on macOS and wants true vibrancy later, `window-vibrancy` remains the documented path (see the research doc); this ADR just records why it isn't in scope now.
