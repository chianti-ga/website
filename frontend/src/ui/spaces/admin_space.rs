use std::sync::{Arc, RwLock, RwLockReadGuard};

use egui::{hex_color, Align, CursorIcon, Image, Layout, Margin, Rounding, Sense, Stroke, Widget};
use egui_commonmark::CommonMarkCache;
use shared::discord::User;
use shared::fiche_rp::{FicheRP, FicheState, FicheVersion, Job, ReviewMessage};
use shared::user::FrontAccount;

use crate::app::{get_string, AuthInfo, ALL_ACCOUNTS, AUTH_INFO};
use crate::ui::components::comment_components::edit_comment_window;
use crate::ui::components::fiche_components::{ficherp_bubble, ficherp_edit, ficherp_history_viewer_window, ficherp_viewer, ficherp_viewer_window};

pub struct AdminSpace {
    pub common_mark_cache: Arc<RwLock<CommonMarkCache>>,

    pub selected_fiche_account: Option<(FrontAccount, FicheRP)>,
    pub selected_fiche_version: Option<FicheVersion>,
    pub selected_account: Option<FrontAccount>,

    pub new_fiche: Option<FicheRP>,
    pub review_message: Option<ReviewMessage>,

    pub job_text_buffer: String,

    pub is_previewing_fiche: bool,
    pub is_writing_message: bool,
    pub is_viewing_fiche_history: bool,
    pub is_editing_existing_fiche: bool,

    pub background_image: Option<String>,
}

impl eframe::App for AdminSpace {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //Global variables
        let auth_binding: Arc<RwLock<AuthInfo>> = AUTH_INFO.clone();
        let auth_lock: RwLockReadGuard<AuthInfo> = auth_binding.read().unwrap();
        let user_account: FrontAccount = auth_lock.clone().account.unwrap();

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

            ui.columns(2, |mut columns| {
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
                        let mut selected_account_name: String = "Choisir un utilisateur".to_string();
                        if let Some(selected_accout) = &mut self.selected_account {
                            selected_account_name = selected_accout.discord_user.global_name.clone();
                        }

                        ui.label("pour l'utilisateur :");

                        egui::ComboBox::from_id_source("ficherp_select_account").selected_text(selected_account_name).show_ui(ui, |ui| {
                            let binding: Arc<RwLock<Vec<FrontAccount>>> = ALL_ACCOUNTS.clone();
                            let all_account = binding.read().unwrap();
                            all_account.iter().for_each(|front_account| {
                                ui.selectable_value(&mut self.selected_account, Option::from(front_account.clone()), &front_account.discord_user.global_name);
                            });
                        });
                    });

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let binding: Arc<RwLock<Vec<FrontAccount>>> = ALL_ACCOUNTS.clone();
                        let all_account = binding.read().unwrap();

                        ui.vertical(|ui| {
                            all_account.iter().for_each(|account| {
                                ui.vertical(|ui| {
                                    account.fiches.iter().for_each(|ficherp| {
                                        ui.add_space(5.0);

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
                                            };
                                        });
                                    });
                                });
                            });
                        });
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
                                    if ficherp_edit(ui, ficherp, &mut self.is_previewing_fiche, &mut self.job_text_buffer, &mut self.is_editing_existing_fiche, &mut self.background_image, &self.selected_account) {
                                        self.is_viewing_fiche_history = false;
                                        self.is_writing_message = false;
                                        self.is_previewing_fiche = false;
                                    }
                                });
                            }
                        });
                    });
                });
            });
        });
    }
}