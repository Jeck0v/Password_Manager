use eframe::egui::{self, Button, RichText, TextEdit};
use crate::ui::AppState;

pub struct Calculator {
    input: String,
    pub secret_triggered: bool,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            input: String::new(),
            secret_triggered: false,
        }
    }
}

impl Calculator {
    pub fn update(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, state: &mut AppState) {
        ui.label(RichText::new("Calculatrice").size(30.0));
        ui.add_sized([250.0, 50.0], TextEdit::singleline(&mut self.input).hint_text("Entrez un calcul..."));

        let buttons = [
            ["7", "8", "9", "/"],
            ["4", "5", "6", "*"],
            ["1", "2", "3", "-"],
            ["C", "0", "=", "+"],
        ];

        for row in buttons {
            ui.horizontal(|ui| {
                for &b in &row {
                    if ui.add_sized([60.0, 60.0], Button::new(RichText::new(b).size(24.0))).clicked() {
                        match b {
                            "=" => self.evaluate_expression(),
                            "C" => self.clear(),
                            _ => self.add_char(b.chars().next().unwrap()),
                        }
                    }
                }
            });
        }

        let shift_pressed = ctx.input(|i| i.modifiers.shift);
        if self.input.trim() == "1337" && shift_pressed { //changer paterne secret
            *state = AppState::Auth;
        }
    }

    fn evaluate_expression(&mut self) {
        let expr = self.input.clone();
        let result = meval::eval_str(&expr);

        match result {
            Ok(value) => self.input = format!("{}", value),
            Err(_) => self.input = "Erreur".to_string(),
        }
    }

    fn clear(&mut self) {
        self.input.clear();
    }

    fn add_char(&mut self, c: char) {
        self.input.push(c);
    }
}
