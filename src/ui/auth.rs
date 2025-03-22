use eframe::egui::{self, RichText, TextEdit, Button};
use crate::ui::AppState;

pub struct AuthScreen {
    pub entered_password: String,
    pub master_password: String,
    pub failed_attempts: u32,
}

impl Default for AuthScreen {
    fn default() -> Self {
        Self {
            entered_password: String::new(),
            master_password: "1234".to_string(), // TODO: Remplacer par un vrai stockage sécurisé
            failed_attempts: 0,
        }
    }
}

impl AuthScreen {
    pub fn update(&mut self, ui: &mut egui::Ui, state: &mut AppState) {
        ui.vertical_centered(|ui| {
            ui.label(RichText::new("Entrez votre mot de passe").size(24.0));

            ui.add_sized([250.0, 40.0], TextEdit::singleline(&mut self.entered_password).password(true));

            if ui.add_sized([250.0, 40.0], Button::new("Se connecter")).clicked() {
                if self.entered_password == self.master_password {
                    *state = AppState::Manager;
                } else {
                    self.failed_attempts += 1;
                    self.entered_password.clear();
                }
            }

            if self.failed_attempts > 0 {
                ui.label(RichText::new("Mot de passe incorrect").color(egui::Color32::RED));
            }
        });
    }
}
