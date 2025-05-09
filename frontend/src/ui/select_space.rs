use crate::app::{image_resolver, Space, SELECTED_ROLE, SELECTED_SPACE};
use eframe::egui;
use eframe::egui::{Align, Image, Layout};
use egui::Button;
use shared::permissions::DiscordRole;
use std::sync::RwLockReadGuard;

pub struct SpacePanel {}
impl SpacePanel {
    pub fn new() -> Self {
        SpacePanel {}
    }
}

impl eframe::App for SpacePanel {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(5, |mut columns| {
                let role_binding = SELECTED_ROLE.clone();
                let user_role: RwLockReadGuard<DiscordRole> = role_binding.read().unwrap();

                let is_staff: bool = *user_role == DiscordRole::PlatformAdmin || *user_role == DiscordRole::Admin || *user_role == DiscordRole::LeadScenarist || *user_role == DiscordRole::Scenarist;

                if is_staff {
                    columns[0].with_layout(Layout::top_down(Align::Center), |ui| {
                        let image: Image = Image::new(image_resolver("admin_expo.svg"))
                            .fit_to_original_size(1.0)
                            .max_width(ui.available_width() - 20.0)
                            .maintain_aspect_ratio(true)
                            .show_loading_spinner(true);

                        ui.centered_and_justified(|ui| {
                            let admin_space_btn = Button::image_and_text(image, "");

                            if ui.add(admin_space_btn).clicked() {
                                SELECTED_SPACE.write().unwrap().selected_space = Space::EadminSpace;
                            }
                        });
                    });
                }

                /*columns[1].with_layout(Layout::top_down(Align::Center), |ui| {
                    let image: Image = Image::new(image_resolver("rapport_exp.svg"))
                        .fit_to_original_size(1.0)
                        .max_width(ui.available_width() - 20.0)
                        .maintain_aspect_ratio(true)
                        .show_loading_spinner(true);

                    ui.centered_and_justified(|ui| {
                        let science_space_btn = Button::image_and_text(image, "");

                        ui.add(science_space_btn);
                    });
                });
                columns[2].with_layout(Layout::top_down(Align::Center), |ui| {
                    let image: Image = Image::new(image_resolver("berret_expo.svg"))
                        .fit_to_original_size(1.0)
                        .max_width(ui.available_width() - 20.0)
                        .maintain_aspect_ratio(true)
                        .show_loading_spinner(true)
                        .show_loading_spinner(true);

                    ui.centered_and_justified(|ui| {
                        let security_space_btn = Button::image_and_text(image, "");

                        ui.add(security_space_btn);
                    });
                });*/
                columns[2].with_layout(Layout::top_down(Align::Center), |ui| {
                    let image: Image = Image::new(image_resolver("ficherp_gestion_expo.svg"))
                        .fit_to_original_size(1.0)
                        .max_width(ui.available_width() - 20.0)
                        .maintain_aspect_ratio(true)
                        .show_loading_spinner(true);

                    ui.centered_and_justified(|ui| {
                        let fiche_space_btn = Button::image_and_text(image, "");

                        if ui.add(fiche_space_btn).clicked() {
                            SELECTED_SPACE.write().unwrap().selected_space = Space::EficheSpace;
                        }
                    });
                });
            });
        });
    }
}