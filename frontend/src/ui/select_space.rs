use eframe::egui;
use eframe::egui::{Align, Image, Layout};
use egui::Button;

use crate::app::{image_resolver, SELECTED_SPACE, Space};

pub struct SpacePanel {}
impl SpacePanel {
    pub fn new() -> Self {
        SpacePanel {}
    }
}

impl eframe::App for SpacePanel {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(4, |mut columns| {
                columns[0].with_layout(Layout::top_down(Align::Center), |ui| {
                    let image: Image = Image::new(image_resolver("admin_expo.svg"))
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
                });
                columns[3].with_layout(Layout::top_down(Align::Center), |ui| {
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