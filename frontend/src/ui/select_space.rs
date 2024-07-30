use eframe::egui;
use eframe::egui::{Align, Image, Layout};
use egui::{Button, Color32};

use crate::app::{image_resolver, SELECTED_SPACE, Space};

pub struct SpacePanel {
    location_url: String,
}
impl SpacePanel {
    pub fn new(location_url: String) -> Self {
        SpacePanel {
            location_url,
        }
    }
}

impl eframe::App for SpacePanel {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().widgets.noninteractive.bg_fill = Color32::BLUE;
            ui.visuals_mut().extreme_bg_color = Color32::GREEN;
            ui.visuals_mut().window_fill = Color32::RED;
            ui.visuals_mut().panel_fill = Color32::YELLOW;

            ui.columns(4, |mut columns| {
                columns[0].with_layout(Layout::top_down(Align::Center), |ui| {
                    let image: Image = Image::new(image_resolver(&self.location_url, "admin_expo.svg"))
                        .fit_to_original_size(1.0)
                        .max_width(ui.available_width() - 20.0)
                        .maintain_aspect_ratio(true)
                        .show_loading_spinner(true);

                    ui.centered_and_justified(|ui| {
                        let admin_space_btn = Button::image_and_text(image, "");

                        if ui.add(admin_space_btn).clicked() {};
                    });
                });

                columns[1].with_layout(Layout::top_down(Align::Center), |ui| {
                    let image: Image = Image::new(image_resolver(&self.location_url, "rapport_exp.svg"))
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
                    let image: Image = Image::new(image_resolver(&self.location_url, "berret_expo.svg"))
                        .fit_to_original_size(1.0)
                        .max_width(ui.available_width() - 20.0)
                        .maintain_aspect_ratio(true)
                        .show_loading_spinner(true)
                        .show_loading_spinner(true);

                    ui.centered_and_justified(|ui| {
                        let security_space_btn = Button::image_and_text(image, "");

                        ui.add(security_space_btn);
                    });
                });
                columns[3].with_layout(Layout::top_down(Align::Center), |ui| {
                    let image: Image = Image::new(image_resolver(&self.location_url, "ficherp_gestion_expo.svg"))
                        .fit_to_original_size(1.0)
                        .max_width(ui.available_width() - 20.0)
                        .maintain_aspect_ratio(true)
                        .show_loading_spinner(true);

                    ui.centered_and_justified(|ui| {
                        let fiche_space_btn = Button::image_and_text(image, "");

                        if ui.add(fiche_space_btn).clicked() {
                            SELECTED_SPACE.lock().unwrap().selected_space = Space::eFicheSpace;
                        }
                    });
                });
            });
        });
    }
}