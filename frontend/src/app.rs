use std::future::IntoFuture;
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, TryLockResult};

use crate::backend_handler::{authenticate, get_oath2_url};
use crate::ui::select_space::SpacePanel;
use crate::ui::spaces::fiche_space::FicheSpace;
use eframe::egui;
use eframe::egui::{Style, TextStyle};
use egui::style::ScrollStyle;
use egui::FontFamily::Proportional;
use egui::{hex_color, Align, Button, Color32, FontId, Image, Layout, RichText};
use egui_commonmark::CommonMarkCache;
use json_gettext::{get_text, static_json_gettext_build, JSONGetText};
use lazy_static::lazy_static;
use log::{error, warn};
use shared::permissions::DiscordRole;
use shared::user::FrontAccount;
use shared::website_meta::WebsiteMeta;

pub struct App {
    pub location_url: String,
    pub is_ui_debug: bool,
    // PANELS
    pub fiche_space: FicheSpace,
    pub space_panel: SpacePanel,
}
#[derive(Clone)]
pub struct AuthInfo {
    pub authenticated: bool,
    pub account: Option<FrontAccount>,
    pub website_meta: WebsiteMeta,
}
impl Default for AuthInfo {
    fn default() -> Self {
        AuthInfo {
            authenticated: false,
            account: None,
            website_meta: Default::default(),
        }
    }
}
pub struct SelectedSpace {
    pub selected_space: Space,
}
impl Default for SelectedSpace {
    fn default() -> Self {
        SelectedSpace {
            selected_space: Space::Eselection
        }
    }
}
#[derive(Copy, Clone)]
pub enum Space {
    Eselection,
    EspaceSelection,
    EadminSpace,
    EficheSpace,
    EscienceSpace,
    EsecuritySpace,
}

lazy_static! {
    pub static ref SELECTED_ROLE:Arc<RwLock<DiscordRole>> = Arc::new(RwLock::new(DiscordRole::User));
    pub static ref SELECTED_SPACE:Arc<RwLock<SelectedSpace>> = Arc::new(RwLock::new(SelectedSpace::default()));
    pub static ref GET_TEXT_CTX:Arc<JSONGetText<'static>>=Arc::new(static_json_gettext_build!("fr_FR";"fr_FR" => "assets/langs/fr_FR.json").unwrap());
    pub static ref AUTH_INFO:Arc<RwLock<AuthInfo>> = Arc::new(RwLock::new(AuthInfo::default()));
    pub static ref ALL_ACCOUNTS:Arc<RwLock<Vec<FrontAccount>>> = Arc::new(RwLock::new(vec![]));
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let mut style: Style = Style::default();
        style.visuals.override_text_color.insert(Color32::from_hex("#B8B8B8").unwrap());

        style.text_styles.insert(TextStyle::Name("heading2".into()), FontId::new(16.0, Proportional));
        style.text_styles.insert(TextStyle::Name("heading3".into()), FontId::new(14.0, Proportional));
        style.visuals.extreme_bg_color = hex_color!("#161616");
        style.spacing.scroll = ScrollStyle::solid();
        cc.egui_ctx.set_style(Arc::new(style));

        egui_extras::install_image_loaders(&cc.egui_ctx);

        authenticate();

        Self {
            location_url: cc.integration_info.web_info.location.url.clone(),
            is_ui_debug: false,

            fiche_space: FicheSpace {
                selected_role: DiscordRole::User,
                common_mark_cache: Arc::new(RwLock::new(CommonMarkCache::default())),
                selected_fiche_account: None,
                selected_fiche_version: None,
                new_fiche: None,
                review_message: None,
                job_text_buffer: "".to_string(),
                is_previewing_fiche: false,
                is_writing_message: false,
                is_viewing_fiche_history: false,
                is_editing_existing_fiche: false,
                background_image: None,
            },

            space_panel: SpacePanel::new(),
        }
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(debug_assertions)]
        if self.is_ui_debug {
            ctx.style_mut(move |style| {
                style.debug = egui::style::DebugOptions {
                    debug_on_hover: true,
                    debug_on_hover_with_all_modifiers: true,
                    hover_shows_next: true,
                    show_expand_width: true,
                    show_expand_height: true,
                    show_resize: true,
                    show_interactive_widgets: true,
                    show_widget_hits: true,
                };
            });
        } else {
            ctx.style_mut(move |style| {
                style.debug = egui::style::DebugOptions::default()
            });
        }

        let binding_auth_info: Arc<RwLock<AuthInfo>> = AUTH_INFO.clone();
        let auth_info: RwLockReadGuard<AuthInfo> = binding_auth_info.read().unwrap();

        if !auth_info.authenticated {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new(get_string("auth.text")).text_style(TextStyle::Heading));
                    let discord_button = Button::image_and_text(Image::new(image_resolver("discord_steam_link.svg")).fit_to_original_size(0.75).maintain_aspect_ratio(true), get_string("auth.btn.text").to_string());
                    if ui.add(discord_button).clicked() {
                        web_sys::window().expect("no global `window` exists").location().set_href(&*get_oath2_url()).expect("Can't redirect");
                    };
                });
            });
        } else {
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.horizontal(|ui| {
                        match AUTH_INFO.clone().try_read() {
                            Ok(auth_info) => {
                                let account: FrontAccount = auth_info.account.clone().unwrap();

                                if ui.button(get_string("nav.btn.home")).clicked() {
                                    SELECTED_SPACE.write().unwrap().selected_space = Space::Eselection;
                                };
                                if ui.button(get_string("nav.btn.ficherp")).clicked() {
                                    SELECTED_SPACE.write().unwrap().selected_space = Space::EficheSpace;
                                };
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    #[cfg(debug_assertions)]
                                    ui.label(format!("Connecté en tant que : {} ({})", account.discord_user.global_name, account.discord_user.id));

                                    #[cfg(debug_assertions)]
                                    ui.toggle_value(&mut self.is_ui_debug, "debug");

                                    match SELECTED_ROLE.try_write() {
                                        Ok(mut lock) => {
                                            egui::ComboBox::from_label("Voir en tant que").selected_text(lock.to_string()).show_ui(ui, |ui| {
                                                if let Some(roles) = DiscordRole::from_role_ids(&account.discord_roles) {
                                                    ui.selectable_value(&mut *lock, DiscordRole::User, DiscordRole::User.to_string());
                                                    roles.iter().for_each(|role| {
                                                        ui.selectable_value(&mut *lock, role.clone(), role.to_string());
                                                    });
                                                    if auth_info.website_meta.whitelist.contains(&account.discord_user.id) {
                                                        ui.selectable_value(&mut *lock, DiscordRole::PlatformAdmin, DiscordRole::PlatformAdmin.to_string());
                                                    }
                                                }
                                            });
                                        }
                                        Err(err) => {
                                            warn!("Waiting for lock : \n {}", err.to_string());
                                        }
                                    }
                                });
                            }
                            Err(err) => {
                                warn!("Waiting for lock : \n {}", err.to_string());
                            }
                        };
                    })
                });
            });

            let selected_space: Space = SELECTED_SPACE.read().unwrap().selected_space;
            match selected_space {
                Space::Eselection => self.space_panel.update(ctx, frame),
                Space::EspaceSelection => {}
                Space::EadminSpace => {}
                Space::EficheSpace => self.fiche_space.update(ctx, frame),
                Space::EscienceSpace => {}
                Space::EsecuritySpace => {}
            }

            egui::TopBottomPanel::bottom("botton_panel").show(ctx, |ui| {
                ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                    footer(ui);
                    egui::warn_if_debug_build(ui);
                });
            });
        }
    }
}

pub fn get_string(query: &str) -> String {
    match get_text!(GET_TEXT_CTX, query) {
        None => query.to_string(),
        Some(text) => text.to_string()
    }
}

pub fn image_resolver(image_name: &str) -> String {
    let mut path: String = web_sys::window()
        .expect("no global `window` exists")
        .location().href().expect("should have a href");
    path.push_str("app_img/");
    path.push_str(image_name);
    path
}

fn footer(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("© Project Visualis 2024");
    });
}

#[cfg(debug_assertions)]
pub fn setup_debug_ui(style: &mut Style) {
    style.debug = egui::style::DebugOptions {
        debug_on_hover: true,
        debug_on_hover_with_all_modifiers: true,
        hover_shows_next: true,
        show_expand_width: true,
        show_expand_height: true,
        show_resize: true,
        show_interactive_widgets: true,
        show_widget_hits: true,
    };
}
