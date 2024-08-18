use std::sync::{Arc, RwLock, RwLockReadGuard};

use eframe::egui;
use eframe::emath::Align;
use eframe::epaint::{Margin, Rounding};
use egui::{hex_color, Layout, Sense, Stroke, Widget};
use egui_commonmark::CommonMarkCache;

use shared::discord::User;
use shared::fiche_rp::{FicheRP, FicheState, FicheVersion, Job, ReviewMessage};
use shared::user::FrontAccount;

use crate::app::{ALL_ACCOUNTS, AUTH_INFO, AuthInfo, get_string};
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
}

impl eframe::App for FicheSpace {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.is_previewing_fiche {
            egui::Window::new("Preview").open(&mut self.is_previewing_fiche).default_size([640.0, 960.0]).show(ctx, |ui| {
                let binding: Arc<RwLock<AuthInfo>> = AUTH_INFO.clone();
                let auth_lock: RwLockReadGuard<AuthInfo> = binding.read().unwrap();
                let account = auth_lock.clone().account.unwrap();
                let user: User = account.discord_user;
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

        if self.is_writing_message {
            egui::Window::new("Ecriture commentaire").open(&mut self.is_writing_message).default_size([640.0, 600.0]).resizable(false).show(ctx, |ui| {
                let binding: Arc<RwLock<AuthInfo>> = AUTH_INFO.clone();
                let auth_lock: RwLockReadGuard<AuthInfo> = binding.read().unwrap();
                let account = auth_lock.clone().account.unwrap();
                let user: User = account.discord_user;

                if let Some(review_message) = &mut self.review_message {
                    edit_comment_window(ui, self.selected_fiche_account.clone().unwrap().1.id, review_message, self.common_mark_cache.clone());
                }
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let frame = egui::Frame::none()
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
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let binding: Arc<RwLock<Vec<FrontAccount>>> = ALL_ACCOUNTS.clone();
                        if let Ok(all_account) = binding.read() {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    //TODO: FILTERING, creation etc
                                    ui.label("MENUBAR");
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
                                    }
                                });

                                ui.add_space(10.0);

                                all_account.iter().filter(|account| !account.fiches.is_empty()).for_each(|account| {
                                    ui.vertical(|ui| {
                                        for ficherp in &account.fiches {
                                            let account_ref: &FrontAccount = account;
                                            let ficherp_ref: &FicheRP = ficherp;
                                            frame.show(ui, |ui| {
                                                let bubble_rec = ficherp_bubble(ui, ficherp_ref, &account_ref.discord_user);

                                                let response = ui.allocate_rect(bubble_rec.rect, Sense::click());

                                                if response.clicked() {
                                                    self.new_fiche = None;
                                                    self.selected_fiche_account = Some((account_ref.clone(), ficherp_ref.clone()));
                                                };
                                            });
                                        }
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
                                    ficherp_viewer(ui, &ficherp, &account.discord_user, self.common_mark_cache.clone(), &mut self.is_viewing_fiche_history, &mut self.is_editing_existing_fiche, &mut self.new_fiche, &mut self.selected_fiche_account);
                                });
                            }

                            if let Some(ficherp) = &mut self.new_fiche {
                                frame.show(ui, |ui| {
                                    ficherp_edit(ui, ficherp, &mut self.is_previewing_fiche, &mut self.job_text_buffer, &mut self.is_editing_existing_fiche)
                                });
                            }
                        });
                    });
                });
                columns[2].with_layout(Layout::top_down(Align::Center), |ui| {
                        let binding: Arc<RwLock<Vec<FrontAccount>>> = ALL_ACCOUNTS.clone();
                        if let Ok(all_account) = binding.read() {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label("MENUBAR");
                                    if ui.button(get_string("ficherp.review_message.create")).clicked() {
                                        let binding: Arc<RwLock<AuthInfo>> = AUTH_INFO.clone();
                                        let auth_lock: RwLockReadGuard<AuthInfo> = binding.read().unwrap();
                                        let account = auth_lock.clone().account.unwrap();
                                        self.review_message = Option::from(ReviewMessage {
                                            discord_id: account.discord_user.id,
                                            content: "".to_string(),
                                            date: 0,
                                            is_private: false,
                                            is_comment: false,
                                            set_state: FicheState::Waiting,
                                        });
                                        self.is_writing_message = true;
                                    }
                                });

                                ui.add_space(10.0);

                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    if self.selected_fiche_account.is_some() {
                                        self.selected_fiche_account.clone().unwrap().1.messages.iter().for_each(|review_message: &ReviewMessage| {
                                            frame.show(ui, |ui| {
                                                comment_bubble(ui, &review_message, self.common_mark_cache.clone())
                                            });
                                        });
                                    }
                                });
                            });
                        };
                    });
            });
        });
    }
}