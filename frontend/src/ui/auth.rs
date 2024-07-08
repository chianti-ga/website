use eframe::egui;
use eframe::egui::{Button, Image};

use crate::app::AUTH_INFO;

pub struct AuthPanel {
    location_url: String,
}
impl AuthPanel {
    pub fn new(location_url: String) -> Self {
        AuthPanel {
            location_url
        }
    }
}

impl eframe::App for AuthPanel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("auth_panel").show(ctx, |ui_panel| {
            ui_panel.columns(3, |columns| {
                columns[1].vertical_centered(|ui| {
                    let discord_button = Button::image_and_text(Image::new(format!("{}discord_steam_link.svg", &self.location_url)).fit_to_original_size(0.75).maintain_aspect_ratio(true), "connexion via Discord et Steam");
                    if ui.add(discord_button).clicked() {
                        AUTH_INFO.lock().unwrap().authenticated = true;
                    };
                });
            });
        });
    }
}