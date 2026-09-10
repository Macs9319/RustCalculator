use eframe::egui;
use rust_calculator::{calculator, format_result};

const WINDOW_SIZE: [f32; 2] = [278.0, 400.0];

// The equals-key accent is the one color that does NOT mirror between
// modes (grilling session decision Q6) — it's a plain constant, not part
// of `Palette`.
const EQUALS_BG: egui::Color32 = egui::Color32::from_rgb(47, 110, 227);

const BUTTON_SIZE: egui::Vec2 = egui::vec2(54.0, 44.0);
const SPACING: f32 = 5.0;
// No literal HIG value exists to anchor this to (see
// docs/research/macos-style-ui.md). Reviewed via screenshot in both dark
// and light mode while implementing the theme-aware palette (#3) and kept
// at its pre-existing value — it already reads well in both.
const CORNER_RADIUS: u8 = 10;
const DISPLAY_HEIGHT: f32 = 54.0;

/// Converts sRGB `[0,255]` to HSL: hue in `[0,360)`, saturation and
/// lightness in `[0,1]`.
fn rgb_to_hsl(c: egui::Color32) -> (f32, f32, f32) {
    let r = c.r() as f32 / 255.0;
    let g = c.g() as f32 / 255.0;
    let b = c.b() as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    let d = max - min;
    if d < f32::EPSILON {
        return (0.0, 0.0, l);
    }

    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };
    let mut h = if max == r {
        ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    h *= 60.0;
    if h < 0.0 {
        h += 360.0;
    }
    (h, s, l)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> egui::Color32 {
    if s < f32::EPSILON {
        let v = (l * 255.0).round().clamp(0.0, 255.0) as u8;
        return egui::Color32::from_rgb(v, v, v);
    }

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0).rem_euclid(2.0) - 1.0).abs());
    let m = l - c / 2.0;
    let (r1, g1, b1) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let to_u8 = |v: f32| ((v + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    egui::Color32::from_rgb(to_u8(r1), to_u8(g1), to_u8(b1))
}

/// Inverts a color's lightness around the midpoint while preserving hue
/// and saturation. This is the "systematic mirror" derivation agreed for
/// deriving the light-mode palette from the dark one (grilling session
/// decision Q5).
fn mirror_lightness(c: egui::Color32) -> egui::Color32 {
    let (h, s, l) = rgb_to_hsl(c);
    hsl_to_rgb(h, s, 1.0 - l)
}

/// WCAG relative luminance of an sRGB color.
// Only exercised by `palette_tests` today, which is exactly its job: it
// exists to make an under-contrast palette fail the build, not for any
// runtime UI behavior.
#[allow(dead_code)]
fn relative_luminance(c: egui::Color32) -> f32 {
    let channel = |v: u8| {
        let v = v as f32 / 255.0;
        if v <= 0.03928 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * channel(c.r()) + 0.7152 * channel(c.g()) + 0.0722 * channel(c.b())
}

/// WCAG contrast ratio between two colors (1.0 = no contrast, 21.0 = max).
#[allow(dead_code)]
fn contrast_ratio(a: egui::Color32, b: egui::Color32) -> f32 {
    let (la, lb) = (relative_luminance(a), relative_luminance(b));
    let (lighter, darker) = if la > lb { (la, lb) } else { (lb, la) };
    (lighter + 0.05) / (darker + 0.05)
}

/// The GUI's color palette. `dark()` holds the original hand-tuned colors;
/// `light()` derives from it mechanically via [`mirror_lightness`], with two
/// documented exceptions:
///
/// - `equals_bg` is fixed across both modes (grilling session decision Q6).
/// - `text_muted` is independently tuned per mode rather than mirrored.
///   WCAG contrast ratio is *not* preserved under simultaneous lightness
///   mirroring of both a background and its foreground text — mirroring
///   `ctrl_bg` and the original `text_muted` together turned a merely
///   under-threshold dark-mode pairing (~2.6:1) into a badly failing
///   light-mode one (~1.2:1). Caught by `palette_tests`, not assumed.
///
/// See `docs/adr` and `CONTEXT.md` for the categorical hierarchy these
/// fields map to.
#[derive(Clone, Copy)]
struct Palette {
    app_bg: egui::Color32,
    display_bg: egui::Color32,
    display_border: egui::Color32,
    text: egui::Color32,
    text_muted: egui::Color32,
    error: egui::Color32,
    num_bg: egui::Color32,
    ctrl_bg: egui::Color32,
    op_bg: egui::Color32,
    equals_bg: egui::Color32,
}

impl Palette {
    fn dark() -> Self {
        Self {
            app_bg: egui::Color32::from_rgb(17, 18, 21),
            display_bg: egui::Color32::from_rgb(26, 27, 32),
            display_border: egui::Color32::from_rgb(42, 43, 50),
            text: egui::Color32::from_rgb(236, 237, 241),
            text_muted: egui::Color32::from_rgb(140, 142, 152),
            error: egui::Color32::from_rgb(224, 108, 108),
            num_bg: egui::Color32::from_rgb(38, 39, 46),
            ctrl_bg: egui::Color32::from_rgb(50, 51, 60),
            op_bg: egui::Color32::from_rgb(51, 72, 102),
            equals_bg: EQUALS_BG,
        }
    }

    fn light() -> Self {
        let dark = Self::dark();
        Self {
            app_bg: mirror_lightness(dark.app_bg),
            display_bg: mirror_lightness(dark.display_bg),
            display_border: mirror_lightness(dark.display_border),
            text: mirror_lightness(dark.text),
            // Independently tuned, not mirrored — see the exception note
            // on this struct's doc comment.
            text_muted: egui::Color32::from_rgb(90, 92, 100),
            error: mirror_lightness(dark.error),
            num_bg: mirror_lightness(dark.num_bg),
            ctrl_bg: mirror_lightness(dark.ctrl_bg),
            op_bg: mirror_lightness(dark.op_bg),
            equals_bg: dark.equals_bg,
        }
    }

    /// Selects the palette matching the currently active egui theme.
    fn for_theme(dark_mode: bool) -> Self {
        if dark_mode { Self::dark() } else { Self::light() }
    }
}

#[cfg(test)]
mod palette_tests {
    use super::*;

    // All button/display text in this UI is >=17px, well within WCAG's
    // "large text" category, whose minimum contrast ratio is 3:1.
    const MIN_CONTRAST: f32 = 3.0;

    #[test]
    fn mirror_lightness_round_trips_approximately() {
        for c in [
            egui::Color32::from_rgb(17, 18, 21),
            egui::Color32::from_rgb(51, 72, 102),
            egui::Color32::from_rgb(236, 237, 241),
        ] {
            let back = mirror_lightness(mirror_lightness(c));
            for (a, b) in [(c.r(), back.r()), (c.g(), back.g()), (c.b(), back.b())] {
                assert!(
                    (a as i16 - b as i16).abs() <= 2,
                    "expected {c:?} to round-trip, got {back:?}"
                );
            }
        }
    }

    #[test]
    fn equals_accent_is_unchanged_between_modes() {
        assert_eq!(Palette::dark().equals_bg, Palette::light().equals_bg);
    }

    #[test]
    fn dark_palette_meets_minimum_contrast() {
        let p = Palette::dark();
        assert!(contrast_ratio(p.display_bg, p.text) >= MIN_CONTRAST);
        assert!(contrast_ratio(p.display_bg, p.error) >= MIN_CONTRAST);
        assert!(contrast_ratio(p.num_bg, p.text) >= MIN_CONTRAST);
        assert!(contrast_ratio(p.ctrl_bg, p.text_muted) >= MIN_CONTRAST);
        assert!(contrast_ratio(p.op_bg, p.text) >= MIN_CONTRAST);
        assert!(contrast_ratio(p.equals_bg, egui::Color32::WHITE) >= MIN_CONTRAST);
    }

    #[test]
    fn light_palette_meets_minimum_contrast() {
        let p = Palette::light();
        assert!(contrast_ratio(p.display_bg, p.text) >= MIN_CONTRAST);
        assert!(contrast_ratio(p.display_bg, p.error) >= MIN_CONTRAST);
        assert!(contrast_ratio(p.num_bg, p.text) >= MIN_CONTRAST);
        assert!(contrast_ratio(p.ctrl_bg, p.text_muted) >= MIN_CONTRAST);
        assert!(contrast_ratio(p.op_bg, p.text) >= MIN_CONTRAST);
        assert!(contrast_ratio(p.equals_bg, egui::Color32::WHITE) >= MIN_CONTRAST);
    }
}

const INTER_REGULAR: &str = "Inter-Regular";

/// Inserts the embedded Inter font at the front of the proportional family
/// (used for all button/label text), leaving the monospace family — and so
/// the numeric display, which explicitly requests monospace — untouched.
/// See ADR-0002 for why Inter rather than SF Pro or the system font.
fn build_fonts() -> egui::FontDefinitions {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        INTER_REGULAR.to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/Inter-Regular.ttf"
        ))),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, INTER_REGULAR.to_owned());
    fonts
}

#[cfg(test)]
mod build_fonts_tests {
    use super::*;

    #[test]
    fn embeds_inter_regular_font_data() {
        assert!(build_fonts().font_data.contains_key(INTER_REGULAR));
    }

    #[test]
    fn inter_leads_the_proportional_family() {
        let fonts = build_fonts();
        let proportional = &fonts.families[&egui::FontFamily::Proportional];
        assert_eq!(proportional.first(), Some(&INTER_REGULAR.to_owned()));
    }

    #[test]
    fn monospace_family_is_untouched_by_inter() {
        // The numeric display relies on the default monospace font for
        // stable digit widths; Inter must never appear in this family.
        let fonts = build_fonts();
        let monospace = &fonts.families[&egui::FontFamily::Monospace];
        assert!(!monospace.contains(&INTER_REGULAR.to_owned()));
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(WINDOW_SIZE)
            .with_min_inner_size(WINDOW_SIZE)
            .with_max_inner_size(WINDOW_SIZE)
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Calculator",
        options,
        Box::new(|cc| {
            // Follow the OS light/dark preference live (grilling session
            // decision Q2); this is egui's default, set explicitly to
            // document intent rather than rely on it silently.
            cc.egui_ctx.set_theme(egui::ThemePreference::System);
            cc.egui_ctx.set_fonts(build_fonts());
            Ok(Box::new(CalculatorApp::default()))
        }),
    )
}

#[derive(Default)]
struct CalculatorApp {
    display: String,
    error: bool,
    /// True right after '=' — the next digit starts a fresh expression
    /// instead of appending to the shown result.
    just_evaluated: bool,
}

impl CalculatorApp {
    fn push(&mut self, s: &str) {
        if self.error || self.just_evaluated {
            self.display.clear();
        }
        self.error = false;
        self.just_evaluated = false;
        self.display.push_str(s);
    }

    fn clear(&mut self) {
        self.display.clear();
        self.error = false;
        self.just_evaluated = false;
    }

    fn backspace(&mut self) {
        if self.error {
            self.clear();
            return;
        }
        self.display.pop();
        self.just_evaluated = false;
    }

    fn evaluate(&mut self) {
        if self.display.is_empty() {
            return;
        }
        match calculator::evaluate(&self.display) {
            Ok(result) => {
                self.display = format_result(result);
                self.error = false;
            }
            Err(e) => {
                self.display = e.to_string();
                self.error = true;
            }
        }
        self.just_evaluated = true;
    }

    fn handle_label(&mut self, label: &str) {
        match label {
            "C" => self.clear(),
            "Del" => self.backspace(),
            "÷" => self.push("/"),
            "×" => self.push("*"),
            "−" => self.push("-"),
            other => self.push(other),
        }
    }
}

/// Shifts each RGB channel of `c` by `delta` (clamped to a valid byte).
fn tint(c: egui::Color32, delta: i16) -> egui::Color32 {
    let shift = |v: u8| (v as i16 + delta).clamp(0, 255) as u8;
    egui::Color32::from_rgb(shift(c.r()), shift(c.g()), shift(c.b()))
}

fn set_state(v: &mut egui::style::WidgetVisuals, bg: egui::Color32, fg: egui::Color32) {
    v.weak_bg_fill = bg;
    v.bg_fill = bg;
    v.fg_stroke.color = fg;
    v.bg_stroke = egui::Stroke::NONE;
    v.corner_radius = egui::CornerRadius::same(CORNER_RADIUS);
}

/// A button styled with an explicit color for each interaction state,
/// so it keeps normal hover/press feedback instead of a flat fill.
fn calc_button(ui: &mut egui::Ui, label: &str, bg: egui::Color32, fg: egui::Color32) -> bool {
    ui.scope(|ui| {
        let widgets = &mut ui.style_mut().visuals.widgets;
        set_state(&mut widgets.inactive, bg, fg);
        set_state(&mut widgets.hovered, tint(bg, 18), fg);
        set_state(&mut widgets.active, tint(bg, -16), fg);

        ui.add_sized(
            BUTTON_SIZE,
            egui::Button::new(egui::RichText::new(label).size(17.0).color(fg)),
        )
        .clicked()
    })
    .inner
}

/// A calculator button's semantic role, per the taxonomy in `CONTEXT.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KeyCategory {
    Number,
    Operator,
    Control,
}

/// Classifies a key label per the `Operator key` / `Control key` / `Number key`
/// definitions in `CONTEXT.md`. The `=` key is handled separately (it's styled
/// uniquely, not part of this categorical hierarchy).
fn categorize(label: &str) -> KeyCategory {
    match label {
        "÷" | "×" | "−" | "+" | "%" | "^" => KeyCategory::Operator,
        "C" | "Del" | "(" | ")" => KeyCategory::Control,
        _ => KeyCategory::Number,
    }
}

/// The (background, foreground) color pair for a key's category. The single
/// place that maps `KeyCategory` to `Palette` fields, so every button —
/// including `^`, which isn't part of the row grid — goes through it rather
/// than re-deriving the mapping inline.
fn key_colors(palette: &Palette, category: KeyCategory) -> (egui::Color32, egui::Color32) {
    match category {
        KeyCategory::Control => (palette.ctrl_bg, palette.text_muted),
        KeyCategory::Operator => (palette.op_bg, palette.text),
        KeyCategory::Number => (palette.num_bg, palette.text),
    }
}

#[cfg(test)]
mod categorize_tests {
    use super::*;

    #[test]
    fn arithmetic_operators_are_operator_keys() {
        for label in ["÷", "×", "−", "+"] {
            assert_eq!(categorize(label), KeyCategory::Operator, "{label}");
        }
    }

    #[test]
    fn percent_and_power_are_operator_keys() {
        // % and ^ are binary arithmetic operators just like the other four,
        // so they must not fall back to Control styling (see CONTEXT.md).
        assert_eq!(categorize("%"), KeyCategory::Operator);
        assert_eq!(categorize("^"), KeyCategory::Operator);
    }

    #[test]
    fn non_arithmetic_actions_are_control_keys() {
        for label in ["C", "Del", "(", ")"] {
            assert_eq!(categorize(label), KeyCategory::Control, "{label}");
        }
    }

    #[test]
    fn digits_and_decimal_point_are_number_keys() {
        for label in ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "."] {
            assert_eq!(categorize(label), KeyCategory::Number, "{label}");
        }
    }
}

impl eframe::App for CalculatorApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // Keyboard support.
        ctx.input(|i| {
            for event in &i.events {
                if let egui::Event::Text(text) = event {
                    for ch in text.chars() {
                        if ch.is_ascii_digit() || "+-*/%^().".contains(ch) {
                            self.push(&ch.to_string());
                        }
                    }
                } else if let egui::Event::Key {
                    key, pressed: true, ..
                } = event
                {
                    match key {
                        egui::Key::Enter => self.evaluate(),
                        egui::Key::Backspace => self.backspace(),
                        egui::Key::Escape => self.clear(),
                        _ => {}
                    }
                }
            }
        });

        let palette = Palette::for_theme(ui.visuals().dark_mode);

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(palette.app_bg).inner_margin(16.0))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(SPACING, SPACING);

                let display_text = if self.display.is_empty() {
                    "0"
                } else {
                    self.display.as_str()
                };
                let text_color = if self.error {
                    palette.error
                } else if self.display.is_empty() {
                    palette.text_muted
                } else {
                    palette.text
                };

                let display_size = egui::vec2(ui.available_width(), DISPLAY_HEIGHT);
                let (rect, _response) =
                    ui.allocate_exact_size(display_size, egui::Sense::hover());
                let painter = ui.painter_at(rect);
                painter.rect(
                    rect,
                    CORNER_RADIUS,
                    palette.display_bg,
                    egui::Stroke::new(1.0, palette.display_border),
                    egui::StrokeKind::Inside,
                );
                painter.text(
                    egui::pos2(rect.right() - 14.0, rect.center().y),
                    egui::Align2::RIGHT_CENTER,
                    display_text,
                    egui::FontId::monospace(28.0),
                    text_color,
                );

                ui.add_space(4.0);

                let rows: [[&str; 4]; 5] = [
                    ["C", "Del", "(", ")"],
                    ["7", "8", "9", "÷"],
                    ["4", "5", "6", "×"],
                    ["1", "2", "3", "−"],
                    ["%", "0", ".", "+"],
                ];

                for row in rows {
                    ui.horizontal(|ui| {
                        for label in row {
                            let (bg, fg) = key_colors(&palette, categorize(label));
                            if calc_button(ui, label, bg, fg) {
                                self.handle_label(label);
                            }
                        }
                    });
                }

                ui.horizontal(|ui| {
                    let (bg, fg) = key_colors(&palette, categorize("^"));
                    if calc_button(ui, "^", bg, fg) {
                        self.push("^");
                    }

                    let equals_width = BUTTON_SIZE.x * 3.0 + SPACING * 2.0;
                    ui.scope(|ui| {
                        let widgets = &mut ui.style_mut().visuals.widgets;
                        set_state(&mut widgets.inactive, EQUALS_BG, egui::Color32::WHITE);
                        set_state(
                            &mut widgets.hovered,
                            tint(EQUALS_BG, 18),
                            egui::Color32::WHITE,
                        );
                        set_state(
                            &mut widgets.active,
                            tint(EQUALS_BG, -16),
                            egui::Color32::WHITE,
                        );
                        if ui
                            .add_sized(
                                egui::vec2(equals_width, BUTTON_SIZE.y),
                                egui::Button::new(
                                    egui::RichText::new("=")
                                        .size(18.0)
                                        .color(egui::Color32::WHITE)
                                        .strong(),
                                ),
                            )
                            .clicked()
                        {
                            self.evaluate();
                        }
                    });
                });
            });
    }
}
