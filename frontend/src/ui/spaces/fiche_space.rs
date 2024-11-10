use std::fmt;
use std::sync::{Arc, RwLock, RwLockReadGuard};

use egui::{hex_color, Align, CursorIcon, Image, Layout, Margin, Rounding, Sense, Stroke, Widget};
use egui_commonmark::CommonMarkCache;
use shared::discord::User;
use shared::fiche_rp::{FicheRP, FicheState, FicheVersion, Job, ReviewMessage};
use shared::permissions::DiscordRole;
use shared::user::FrontAccount;

use crate::app::{get_string, AuthInfo, ALL_ACCOUNTS, AUTH_INFO, SELECTED_ROLE};
use crate::ui::components::comment_components::{comment_bubble, edit_comment_window};
use crate::ui::components::fiche_components::{ficherp_bubble, ficherp_edit, ficherp_history_viewer_window, ficherp_viewer, ficherp_viewer_window};

pub struct FicheSpace {
    pub common_mark_cache: Arc<RwLock<CommonMarkCache>>,

    pub selected_fiche_account: Option<(FrontAccount, FicheRP)>,
    pub selected_fiche_version: Option<FicheVersion>,
    pub new_fiche: Option<FicheRP>,
    pub review_message: Option<ReviewMessage>,

    pub job_text_buffer: String,

    pub is_previewing_fiche: bool,
    pub is_writing_message: bool,
    pub is_viewing_fiche_history: bool,
    pub is_editing_existing_fiche: bool,

    pub background_image: Option<String>,

    pub fiche_filter: FilterEnum,
}
#[derive(Eq, PartialEq)]
pub enum FilterEnum {
    OWN,
    ALL,
    ACCEPTED_OTHER,
    WAITING,
}
impl fmt::Display for FilterEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FilterEnum::OWN => write!(f, "Mes Fiches"),
            FilterEnum::ALL => write!(f, "Toutes"),
            FilterEnum::ACCEPTED_OTHER => write!(f, "Fiches des autres"),
            FilterEnum::WAITING => write!(f, "En attente"),
        }
    }
}

impl eframe::App for FicheSpace {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //Global variables
        let role_binding = SELECTED_ROLE.clone();
        let user_role: RwLockReadGuard<DiscordRole> = role_binding.read().unwrap();

        let auth_binding: Arc<RwLock<AuthInfo>> = AUTH_INFO.clone();
        let auth_lock: RwLockReadGuard<AuthInfo> = auth_binding.read().unwrap();
        let user_account: FrontAccount = auth_lock.clone().account.unwrap();

        let is_staff: bool = *user_role == DiscordRole::PlatformAdmin || *user_role == DiscordRole::Admin || *user_role == DiscordRole::LeadScenarist || *user_role == DiscordRole::Scenarist || *user_role == DiscordRole::LeadMed;

        if self.is_previewing_fiche {
            egui::Window::new("Preview").open(&mut self.is_previewing_fiche).default_size([640.0, 960.0]).show(ctx, |ui| {
                let user: User = user_account.clone().discord_user;
                ficherp_viewer_window(ui, &self.new_fiche.clone().unwrap(), &user, self.common_mark_cache.clone());
            });
        }

        if self.is_viewing_fiche_history {
            egui::Window::new("Historique").open(&mut self.is_viewing_fiche_history).default_size([640.0, 960.0]).show(ctx, |ui| {
                let user: User = self.selected_fiche_account.clone().unwrap().0.discord_user;
                let ficherp: FicheRP = self.selected_fiche_account.clone().unwrap().1;

                if self.selected_fiche_version.is_none() {
                    self.selected_fiche_version = Option::from(ficherp.version.get(0).unwrap().to_owned());
                }

                if let Some(fiche_version) = &mut self.selected_fiche_version {
                    ficherp_history_viewer_window(ui, &ficherp, fiche_version, &user, self.common_mark_cache.clone());
                }
            });
        }

        // a bit a fuckery happening here :D
        if self.is_writing_message {
            if self.review_message.is_some() {
                let window = egui::Window::new("Ecriture commentaire").open(&mut self.is_writing_message).default_size([640.0, 600.0]).resizable(false).show(ctx, |ui| {
                    let user: User = user_account.clone().discord_user;

                    if let Some(review_message) = &mut self.review_message {
                        if edit_comment_window(ui, self.selected_fiche_account.clone().unwrap().1.id, review_message, self.common_mark_cache.clone(), &mut self.selected_fiche_account) {
                            self.review_message = None;
                            //self.is_viewing_fiche_history = false;
                            //self.is_previewing_fiche = false;
                        }
                    }
                });
            } else {
                self.is_writing_message = false
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut frame = egui::Frame::none()
                .fill(hex_color!("#262626"))
                .stroke(Stroke {
                    width: 2.0,
                    color: hex_color!("#404040"),
                }
                )
                .rounding(Rounding {
                    nw: 25.0,
                    ne: 25.0,
                    sw: 25.0,
                    se: 25.0,
                })
                .inner_margin(Margin {
                    left: 10.0,
                    right: 10.0,
                    top: 10.0,
                    bottom: 10.0,
                }).outer_margin(Margin {
                left: 5.0,
                right: 5.0,
                top: 0.0,
                bottom: 10.0,
            });

            ui.columns(3, |mut columns| {
                columns[0].with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.horizontal(|ui| {
                        if ui.button(get_string("ficherp.create.fiche")).clicked() {
                            self.selected_fiche_account = None;
                            self.new_fiche = Option::from(FicheRP {
                                id: "".to_string(),
                                name: "".to_string(),
                                job: Job::ClassD,
                                description: "".to_string(),
                                lore: "".to_string(),
                                submission_date: 0,
                                messages: vec![],
                                version: vec![],
                                state: FicheState::Waiting,
                            });

                            self.is_viewing_fiche_history = false;
                            self.is_writing_message = false;
                            self.is_previewing_fiche = false;
                            self.is_editing_existing_fiche = false;
                            self.background_image = None;
                        }
                        ui.label(get_string("ficherp.filter.own_fiche"));

                        egui::ComboBox::from_id_source("role_combo").selected_text(self.fiche_filter.to_string()).show_ui(ui, |ui| {
                            if is_staff {
                                ui.selectable_value(&mut self.fiche_filter, FilterEnum::ALL, FilterEnum::ALL.to_string());
                                ui.selectable_value(&mut self.fiche_filter, FilterEnum::WAITING, FilterEnum::WAITING.to_string());
                                ui.selectable_value(&mut self.fiche_filter, FilterEnum::ACCEPTED_OTHER, "Acceptée");
                            } else {
                                ui.selectable_value(&mut self.fiche_filter, FilterEnum::ACCEPTED_OTHER, FilterEnum::ACCEPTED_OTHER.to_string());
                                ui.selectable_value(&mut self.fiche_filter, FilterEnum::OWN, FilterEnum::OWN.to_string());
                            }
                        });
                    });

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let binding: Arc<RwLock<Vec<FrontAccount>>> = ALL_ACCOUNTS.clone();
                        if let Ok(all_account) = binding.read() {
                            ui.vertical(|ui| {
                                ui.add_space(10.0);
                                all_account.iter().for_each(|account| {
                                    ui.vertical(|ui| {
                                        // Set filter based on conditions
                                        if !account.fiches.is_empty() && is_staff && self.fiche_filter == FilterEnum::OWN {
                                            self.fiche_filter = FilterEnum::ALL;
                                        }

                                        account.fiches.iter().filter(|ficherp| match self.fiche_filter {
                                            FilterEnum::OWN => account.discord_user == user_account.discord_user,
                                            FilterEnum::ACCEPTED_OTHER => ficherp.state == FicheState::Accepted,
                                            FilterEnum::WAITING => ficherp.state == FicheState::Waiting,
                                            FilterEnum::ALL => true,
                                        }).for_each(|ficherp| {
                                            let account_ref: &FrontAccount = account;
                                            let ficherp_ref: &FicheRP = ficherp;

                                            frame.show(ui, |ui| {
                                                let bubble_rec = ficherp_bubble(ui, ficherp_ref, &account_ref.discord_user);
                                                let response = ui.allocate_rect(bubble_rec.rect, Sense::click());

                                                if response.on_hover_cursor(CursorIcon::PointingHand).clicked() {
                                                    self.new_fiche = None;
                                                    self.selected_fiche_account = Some((account_ref.clone(), ficherp_ref.clone()));
                                                    self.selected_fiche_version = None;
                                                    self.is_viewing_fiche_history = false;
                                                    self.is_writing_message = false;
                                                    self.is_previewing_fiche = false;
                                                    self.background_image = None;
                                                }
                                            });
                                        });
                                    });
                                });
                            });
                        };
                    });
                });

                columns[1].with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.vertical_centered(|ui| {
                            if let Some((account, ficherp)) = self.selected_fiche_account.clone() {
                                frame.show(ui, |ui| {
                                    ficherp_viewer(ui, &ficherp, &mut self.job_text_buffer, &account.discord_user, self.common_mark_cache.clone(), &mut self.is_viewing_fiche_history, &mut self.is_editing_existing_fiche, &mut self.new_fiche, &mut self.selected_fiche_account);
                                });
                            } else if let Some(bg_image) = self.background_image.clone() {
                                ui.add(Image::new(&*bg_image).fit_to_original_size(0.5));
                            } else if let Some(ficherp) = &mut self.new_fiche {
                                frame.show(ui, |ui| {
                                    if ficherp_edit(ui, ficherp, &mut self.is_previewing_fiche, &mut self.job_text_buffer, &mut self.is_editing_existing_fiche, &mut self.background_image, &None) {
                                        self.is_viewing_fiche_history = false;
                                        self.is_writing_message = false;
                                        self.is_previewing_fiche = false;
                                    }
                                });
                            }
                        });
                    });
                });

                columns[2].with_layout(Layout::top_down(Align::Center), |ui| {
                    let binding: Arc<RwLock<Vec<FrontAccount>>> = ALL_ACCOUNTS.clone();
                    if let Ok(all_account) = binding.read() {
                        ui.vertical(|ui| {
                            if let Some(selected_fiche_account) = &self.selected_fiche_account {
                                ui.horizontal(|ui| {
                                    if selected_fiche_account.0.discord_user != user_account.discord_user && !is_staff {
                                        ui.disable()
                                    }
                                    if ui.button(get_string("ficherp.review_message.create")).clicked() {
                                        self.review_message = Option::from(ReviewMessage {
                                            discord_id: user_account.clone().discord_user.id,
                                            content: "".to_string(),
                                            date: 0,
                                            is_private: false,
                                            is_comment: false,
                                            set_state: FicheState::Waiting,
                                        });
                                        self.is_writing_message = true;
                                    }
                                });
                                if selected_fiche_account.0.discord_user == user_account.discord_user || is_staff {
                                    egui::ScrollArea::vertical().show(ui, |ui| {
                                        selected_fiche_account.1.messages.iter().for_each(|review_message: &ReviewMessage| {
                                            if !review_message.is_private || (review_message.is_private && is_staff) {
                                                frame.show(ui, |ui| {
                                                    comment_bubble(ui, &review_message, self.common_mark_cache.clone())
                                                });
                                            }
                                        });
                                        ui.add_space(15.0);
                                    });
                                }
                            }
                        });
                    };
                });
            });
        });
    }
}