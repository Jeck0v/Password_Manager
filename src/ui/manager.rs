use eframe::egui::{self, RichText, Button, TextEdit, Label, Ui, ScrollArea, Frame, Separator};

pub struct PasswordManager {
    pub passwords: Vec<(String, String, String)>, // tuples (site, username, password)
    pub new_site: String,
    pub new_username: String,
    pub new_password: String,
}

impl Default for PasswordManager {
    fn default() -> Self {
        Self {
            passwords: vec![
                ("example.com".to_string(), "user1".to_string(), "password123".to_string()),
                ("testsite.com".to_string(), "user2".to_string(), "password456".to_string()),
            ],
            new_site: String::new(),
            new_username: String::new(),
            new_password: String::new(),
        }
    }
}

impl PasswordManager {
    pub fn update(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.label(RichText::new("Gestionnaire de mots de passe").size(30.0));

            ui.group(|ui| {
                ui.label("Ajouter un nouveau mot de passe");

                ui.horizontal(|ui| {
                    ui.label("Site: ");
                    ui.add(TextEdit::singleline(&mut self.new_site).desired_width(300.0));
                });

                ui.horizontal(|ui| {
                    ui.label("Identifiant: ");
                    ui.add(TextEdit::singleline(&mut self.new_username).desired_width(300.0));
                });

                ui.horizontal(|ui| {
                    ui.label("Mot de passe: ");
                    ui.add(TextEdit::singleline(&mut self.new_password).desired_width(300.0));
                });

                if ui.button("Ajouter").clicked() {
                    if !self.new_site.is_empty() && !self.new_username.is_empty() && !self.new_password.is_empty() {
                        self.passwords.push((
                            self.new_site.clone(),
                            self.new_username.clone(),
                            self.new_password.clone(),
                        ));
                        self.new_site.clear();
                        self.new_username.clear();
                        self.new_password.clear();
                    }
                }
            });

            if self.passwords.is_empty() {
                ui.label("Aucun mot de passe enregistré.");
            } else {
                let mut indices_to_remove = Vec::new();

                ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical(|ui| {
                        for (index, (site, username, password)) in self.passwords.iter_mut().enumerate() {
                            Frame::none().fill(egui::Color32::from_gray(240)).show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.add(Label::new(RichText::new(format!("🌐 Site: {}", site)).size(18.0)).wrap(true));
                                        ui.add(Label::new(RichText::new(format!("👤 Utilisateur: {}", username)).size(18.0)).wrap(true));
                                        ui.add(Label::new(RichText::new(format!("🔑 Mot de passe: {}", password)).size(18.0)).wrap(true));
                                    });

                                    if ui.button("Modifier").clicked() {
                                        self.new_site = site.clone();
                                        self.new_username = username.clone();
                                        self.new_password = password.clone();
                                    }

                                    if ui.button("🗑 Supprimer").clicked() {
                                        indices_to_remove.push(index);
                                    }
                                });
                            });

                            ui.add(Separator::default());

                        }
                    });
                });

                for &index in indices_to_remove.iter().rev() {
                    self.passwords.remove(index);
                }
            }
        });
    }
}

// revoir