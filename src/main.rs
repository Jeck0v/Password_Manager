mod ui;
mod db;

use eframe::egui;
use ui::{AppController, AppState};
use eframe::App;

impl App for AppController {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state {
                AppState::Calculator => self.calculator.update(ctx, ui, &mut self.state),
                AppState::Auth => self.auth_screen.update(ui, &mut self.state),
                AppState::Manager => self.password_manager.update(ui),
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Password Manager", options, Box::new(|_cc| Box::new(AppController::default())))
}
