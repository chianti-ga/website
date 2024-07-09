use eframe::egui;
use eframe::egui::{Align, Image, Layout};
use egui::{Button, Color32};

use crate::app::{SELECTED_SPACE, Space};

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
                    let image: Image = Image::new("https://upload.wikimedia.org/wikipedia/commons/thumb/8/8f/Logo_SCP_Foundation.jpg/600px-Logo_SCP_Foundation.jpg")
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
                    let image: Image = Image::new("https://media.discordapp.net/attachments/748634619334885387/1259690529835323453/250.png?ex=668c99ba&is=668b483a&hm=eea42f79e788b3e312c0ff030601a08224130be9b7b5c563b4edc9d2957f0794&=&format=webp&quality=lossless")
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
                    let image: Image = Image::new("https://thumbs.dreamstime.com/z/salutation-de-soldat-18883133.jpg")
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
                    let image: Image = Image::new("https://i.servimg.com/u/f19/19/22/33/31/tm/naheul17.jpg")
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