use std::slice::Iter;
use std::sync::{Arc, LockResult, RwLock, RwLockReadGuard};

use eframe::egui;
use eframe::emath::Align;
use eframe::epaint::{Margin, Rounding};
use egui::{hex_color, Layout, Widget};
use egui_commonmark::CommonMarkCache;

use shared::discord::User;
use shared::fiche_rp::{FicheRP, FicheState, Job};
use shared::fiche_rp::ScienceLevel::Senior;
use shared::fiche_rp::ScienceRole::Researcher;
use shared::user::FrontAccount;
use crate::app::{ALL_ACCOUNTS, AUTH_INFO, AuthInfo, get_string};
use crate::ui::components_helper::{ficherp_bubble, ficherp_edit, ficherp_viewer, ficherp_viewer_window};

pub struct FicheSpace {
    pub common_mark_cache: Arc<RwLock<CommonMarkCache>>,
    pub selected_fiche_account: Option<(FrontAccount, FicheRP)>,
    pub new_fiche: Option<FicheRP>,
    pub preview_fiche: bool,
}


impl eframe::App for FicheSpace {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.preview_fiche {
            egui::Window::new("Preview").open(&mut self.preview_fiche).default_size([640.0, 960.0]).show(ctx, |ui| {
                let binding: Arc<RwLock<AuthInfo>> = AUTH_INFO.clone();
                let auth_lock: RwLockReadGuard<AuthInfo> = binding.read().unwrap();
                let account = auth_lock.clone().account.unwrap();
                let user: User = account.discord_user;
                ficherp_viewer_window(ui, &self.new_fiche.clone().unwrap(), &user, self.common_mark_cache.clone());
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let frame = egui::Frame::none()
                .fill(hex_color!("262626"))
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
                                egui::menu::bar(ui, |ui| {
                                    //TODO: FILTERING, creation etc
                                    ui.label("MENUBAR");
                                    if ui.button(get_string("ficherp.create.fiche")).clicked() {
                                        self.new_fiche = Option::from(FicheRP {
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
                                all_account.iter().filter(|account| !account.fiches.is_empty()).for_each(|account| {
                                    for ficherp in &account.fiches {
                                        let account_ref: &FrontAccount = account;
                                        let ficherp_ref: &FicheRP = ficherp;
                                        ui.vertical(|ui| {
                                            egui::menu::bar(ui, |ui| {
                                                //TODO: FILTERING, creation etc
                                                ui.label("MENUBAR");
                                            });
                                            frame.show(ui, |ui| {
                                                if ficherp_bubble(ui, ficherp_ref, &account_ref.discord_user).clicked() {
                                                    self.selected_fiche_account = Some((account_ref.clone(), ficherp_ref.clone()));
                                                }
                                            });
                                        });
                                    }
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
                                    ficherp_viewer(ui, &ficherp, &account.discord_user, self.common_mark_cache.clone());
                                });
                            }

                            if let Some(ficherp) = &mut self.new_fiche {
                                frame.show(ui, |ui| {
                                    ficherp_edit(ui, ficherp, self.common_mark_cache.clone(), &mut self.preview_fiche)
                                });
                            }
                        });
                    });
                });
                columns[2].with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.centered_and_justified(|ui| {
                        frame.show(ui, |ui| {});
                    });
                });
            });
        });
    }
}