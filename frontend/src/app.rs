use std::sync::{Arc, Mutex};

use eframe::egui;
use eframe::egui::{Style, TextStyle};
use egui::FontFamily::Proportional;
use egui::FontId;
use lazy_static::lazy_static;

use crate::ui::auth::AuthPanel;
use crate::ui::space::SpacePanel;

pub struct App {
    location_url: String,
}

pub struct AuthInfo {
    pub auth_url: String,
    pub location_url: String,
    pub authenticated: bool,
}
impl Default for AuthInfo {
    fn default() -> Self {
        AuthInfo {
            auth_url: "".to_string(),
            location_url: "".to_string(),
            authenticated: false,
        }
    }
}

lazy_static! {
     pub static ref AUTH_INFO:Arc<Mutex<AuthInfo>> = Arc::new(Mutex::new(AuthInfo::default()));
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let mut style: Style = Style::default();
        style.text_styles.insert(TextStyle::Name("heading2".into()), FontId::new(16.0, Proportional));
        cc.egui_ctx.set_style(Arc::new(style));

        egui_extras::install_image_loaders(&cc.egui_ctx);

        Self {
            location_url: cc.integration_info.web_info.location.url.clone(),
        }
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        let mut auth_panel: AuthPanel = AuthPanel::new(self.location_url.clone());
        if !AUTH_INFO.lock().unwrap().authenticated {
            auth_panel.update(ctx, frame);
        }

        SpacePanel::new(self.location_url.clone()).update(ctx, frame);

        egui::TopBottomPanel::bottom("botton_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                footer(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn footer(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("© Project Visualis 2024");
    });
}
