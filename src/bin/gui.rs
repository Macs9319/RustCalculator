use eframe::egui;
use rust_calculator::{calculator, format_result};

const WINDOW_SIZE: [f32; 2] = [278.0, 400.0];

const APP_BG: egui::Color32 = egui::Color32::from_rgb(17, 18, 21);
const DISPLAY_BG: egui::Color32 = egui::Color32::from_rgb(26, 27, 32);
const DISPLAY_BORDER: egui::Color32 = egui::Color32::from_rgb(42, 43, 50);
const TEXT: egui::Color32 = egui::Color32::from_rgb(236, 237, 241);
const TEXT_MUTED: egui::Color32 = egui::Color32::from_rgb(112, 114, 124);
const ERROR: egui::Color32 = egui::Color32::from_rgb(224, 108, 108);
const NUM_BG: egui::Color32 = egui::Color32::from_rgb(38, 39, 46);
const CTRL_BG: egui::Color32 = egui::Color32::from_rgb(50, 51, 60);
const OP_BG: egui::Color32 = egui::Color32::from_rgb(51, 72, 102);
const EQUALS_BG: egui::Color32 = egui::Color32::from_rgb(47, 110, 227);

const BUTTON_SIZE: egui::Vec2 = egui::vec2(54.0, 44.0);
const SPACING: f32 = 5.0;
const CORNER_RADIUS: u8 = 10;
const DISPLAY_HEIGHT: f32 = 54.0;

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
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
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

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(APP_BG).inner_margin(16.0))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(SPACING, SPACING);

                let display_text = if self.display.is_empty() {
                    "0"
                } else {
                    self.display.as_str()
                };
                let text_color = if self.error {
                    ERROR
                } else if self.display.is_empty() {
                    TEXT_MUTED
                } else {
                    TEXT
                };

                let display_size = egui::vec2(ui.available_width(), DISPLAY_HEIGHT);
                let (rect, _response) =
                    ui.allocate_exact_size(display_size, egui::Sense::hover());
                let painter = ui.painter_at(rect);
                painter.rect(
                    rect,
                    CORNER_RADIUS,
                    DISPLAY_BG,
                    egui::Stroke::new(1.0, DISPLAY_BORDER),
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
                            let (bg, fg) = match categorize(label) {
                                KeyCategory::Control => (CTRL_BG, TEXT_MUTED),
                                KeyCategory::Operator => (OP_BG, TEXT),
                                KeyCategory::Number => (NUM_BG, TEXT),
                            };
                            if calc_button(ui, label, bg, fg) {
                                self.handle_label(label);
                            }
                        }
                    });
                }

                ui.horizontal(|ui| {
                    if calc_button(ui, "^", OP_BG, TEXT) {
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
