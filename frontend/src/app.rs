use std::sync::{Arc, Mutex};

use eframe::egui;
use eframe::egui::{Style, TextStyle};
use egui::{Button, Color32, FontId, Image, RichText};
use egui::FontFamily::Proportional;
use json_gettext::{get_text, JSONGetText, static_json_gettext_build};
use lazy_static::lazy_static;

use shared::user::FrontAccount;

use crate::ui::select_space::SpacePanel;
use crate::ui::spaces::fiche_space::FicheSpace;

pub struct App {
    pub location_url: String,
    pub auth_info: AuthInfo,
}

pub struct AuthInfo {
    pub auth_url: String,
    pub location_url: String,
    pub authenticated: bool,
    pub account: Option<FrontAccount>,
}
impl Default for AuthInfo {
    fn default() -> Self {
        AuthInfo {
            auth_url: "".to_string(),
            location_url: "".to_string(),
            authenticated: false,
            account: None,
        }
    }
}
pub struct SelectedSpace {
    pub selected_space: Space,
}
impl Default for SelectedSpace {
    fn default() -> Self {
        SelectedSpace {
            selected_space: Space::eSelection
        }
    }
}
#[derive(Copy, Clone)]
pub enum Space {
    eSelection,
    eSpaceSelection,
    eAdminSpace,
    eFicheSpace,
    eScienceSpace,
    eSecuritySpace,
}

lazy_static! {
    pub static ref SELECTED_SPACE:Arc<Mutex<SelectedSpace>> = Arc::new(Mutex::new(SelectedSpace::default()));
    pub static ref get_text_ctx:Arc<JSONGetText<'static>>=Arc::new(static_json_gettext_build!("fr_FR";"fr_FR" => "assets/langs/fr_FR.json").unwrap());
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let mut style: Style = Style::default();
        style.visuals.override_text_color.insert(Color32::from_hex("#B8B8B8").unwrap());
        /*style.debug=DebugOptions{
            debug_on_hover: true,
            debug_on_hover_with_all_modifiers: true,
            hover_shows_next: true,
            show_expand_width: true,
            show_expand_height: true,
            show_resize: true,
            show_interactive_widgets: true,
            show_widget_hits: true,
        };*/
        style.text_styles.insert(TextStyle::Name("heading2".into()), FontId::new(16.0, Proportional));
        style.text_styles.insert(TextStyle::Name("heading3".into()), FontId::new(14.0, Proportional));
        cc.egui_ctx.set_style(Arc::new(style));

        egui_extras::install_image_loaders(&cc.egui_ctx);

        Self {
            location_url: cc.integration_info.web_info.location.url.clone(),
            auth_info: Default::default(),
        }
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if !self.auth_info.authenticated {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(move |ui| {
                    ui.label(RichText::new(get_text!(get_text_ctx, "auth.text").unwrap().to_string()).text_style(TextStyle::Heading));
                    let discord_button = Button::image_and_text(Image::new(format!("{}discord_steam_link.svg", &self.location_url)).fit_to_original_size(0.75).maintain_aspect_ratio(true), get_text!(get_text_ctx, "auth.btn.text").unwrap().to_string());
                    if ui.add(discord_button).clicked() {
                        self.auth_info.authenticated = true;
                    };
                });
            });
        } else {
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {});
            });
            let selected_space: Space = SELECTED_SPACE.lock().unwrap().selected_space;

            match selected_space {
                Space::eSelection => SpacePanel::new(self.location_url.clone()).update(ctx, frame),
                Space::eSpaceSelection => {}
                Space::eAdminSpace => {}
                Space::eFicheSpace => FicheSpace::new().update(ctx, frame),
                Space::eScienceSpace => {}
                Space::eSecuritySpace => {}
            }

            egui::TopBottomPanel::bottom("botton_panel").show(ctx, |ui| {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    footer(ui);
                    egui::warn_if_debug_build(ui);
                });
            });
        }
    }
}

pub fn image_resolver(image_name: &str) -> String {
    let mut path: String = web_sys::window()
        .expect("no global `window` exists")
        .location().href().expect("should have a href");
    path.push_str(image_name);
    path
}

fn footer(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("© Project Visualis 2024");
    });
}
