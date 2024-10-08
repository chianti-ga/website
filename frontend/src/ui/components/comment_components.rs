use std::sync::{Arc, RwLock, RwLockWriteGuard, TryLockResult};

use chrono::{NaiveDateTime, TimeZone, Utc};
use eframe::emath::Align;
use egui::ecolor::color_hex::color_from_hex;
use egui::text::LayoutJob;
use egui::{hex_color, Color32, Image, Layout, Response, RichText, TextFormat, TextStyle};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use log::warn;
use strum::IntoEnumIterator;
use web_time::{SystemTime, UNIX_EPOCH};

use shared::discord::User;
use shared::fiche_rp::{FicheRP, FicheState, FicheStateIter, ReviewMessage};
use shared::permissions::DiscordRole;
use shared::user::FrontAccount;

use crate::app::{AuthInfo, ALL_ACCOUNTS, AUTH_INFO, SELECTED_ROLE};
use crate::backend_handler::post_comment;
use crate::ui::components::fiche_components::state_badge;

pub fn edit_comment_window(ui: &mut egui::Ui, ficherp_id: String, review_message: &mut ReviewMessage, cache: Arc<RwLock<CommonMarkCache>>, selected_fiche_account: &mut Option<(FrontAccount, FicheRP)>) -> bool {
    let mut close: bool = false;
    ui.vertical(|ui| {
        match SELECTED_ROLE.try_read() {
            Ok(role_lock) => {
                ui.horizontal(|ui| {
                    if *role_lock != DiscordRole::User {
                        ui.checkbox(&mut review_message.is_comment, "Commentaire ?");
                        if review_message.is_comment {
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.checkbox(&mut review_message.is_private, "Commentaire privé ?");
                                review_message.set_state = FicheState::Comment;
                            });
                        } else {
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                egui::ComboBox::from_label("Statut de la fiche").selected_text(review_message.set_state.get_text()).show_ui(ui, |ui| {
                                    let state_iter: FicheStateIter = FicheState::iter();

                                    if *role_lock == DiscordRole::Admin || *role_lock == DiscordRole::PlatformAdmin {
                                        state_iter.filter(|state| state != &FicheState::Comment).for_each(|state| {
                                            ui.selectable_value(&mut review_message.set_state, state.clone(), state.get_text());
                                        });
                                    } else if *role_lock == DiscordRole::Scenarist {
                                        ui.selectable_value(&mut review_message.set_state, FicheState::StaffValidated, FicheState::StaffValidated.get_text());
                                        ui.selectable_value(&mut review_message.set_state, FicheState::Refused, FicheState::Refused.get_text());
                                    } else if *role_lock == DiscordRole::LeadScenarist || *role_lock == DiscordRole::LeadMed {
                                        state_iter.filter(|state| state != &FicheState::Comment).for_each(|state| {
                                            ui.selectable_value(&mut review_message.set_state, state.clone(), state.get_text());
                                        });
                                    }
                                });
                            });
                        }
                    } else {
                        review_message.set_state = FicheState::Comment;
                    }
                });

                ui.label(RichText::new("Commentaire : ").text_style(TextStyle::Name("heading3".into())).strong());

                let mut cache: RwLockWriteGuard<CommonMarkCache> = cache.write().expect("Can't access common_mark_cache");

                egui::ScrollArea::vertical().id_source("scoll_comment_viewer").show(ui, |ui| {
                    let mut size = ui.available_size();
                    size.y = size.y / 3.0;
                    size.x *= 0.99;

                    ui.add_sized(size, egui::TextEdit::multiline(&mut review_message.content));

                    ui.label(RichText::new("Preview : ").text_style(TextStyle::Name("heading3".into())).strong());

                    CommonMarkViewer::new().show(ui, &mut cache, &review_message.content);
                });

                ui.vertical_centered(|ui| {
                    if ui.button("Poster le commentaire/réponse").clicked() {
                        review_message.date = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                        post_comment(review_message, ficherp_id);

                        *selected_fiche_account = None; // Close current view
                        close = true;
                    }
                });
            }
            Err(err) => {
                warn!("Can't get a read lock : \n {}", err)
            }
        }
    });
    close
}

pub fn comment_bubble(ui: &mut egui::Ui, review_message: &ReviewMessage, cache: Arc<RwLock<CommonMarkCache>>) -> Response {
    let binding = ALL_ACCOUNTS.read().unwrap();
    let account: &FrontAccount = binding.iter().find(|front_account| front_account.discord_user.id == review_message.discord_id).unwrap();
    let user: &User = &account.discord_user;
    let avatar_url: String = format!("https://cdn.discordapp.com/avatars/{}/{}.png?size=128", &user.id, user.avatar);

    let avatar_image: Image = Image::new(avatar_url).fit_to_original_size(0.5).maintain_aspect_ratio(true).rounding(100.0);
    let datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(review_message.date as i64, 0));

    let formatted_date = datetime.format("%d-%m-%Y %H:%M:%S").to_string();

    // we privilege certain roles because a user can have several roles that match
    let user_role = if AUTH_INFO.try_read().unwrap().website_meta.whitelist.contains(&account.discord_user.id) {
        DiscordRole::PlatformAdmin
    } else if let Some(roles) = DiscordRole::from_role_ids(&account.discord_roles) {
        if roles.contains(&DiscordRole::LeadScenarist) {
            DiscordRole::LeadScenarist
        } else if roles.contains(&DiscordRole::Admin) {
            DiscordRole::Admin
        } else if roles.contains(&DiscordRole::Scenarist) {
            DiscordRole::Scenarist
        } else {
            DiscordRole::User
        }
    } else {
        DiscordRole::User
    };

    ui.vertical(|ui| {
        ui.vertical_centered(|ui| {
            let mut job = LayoutJob::default();
            if review_message.is_private {
                job.append(
                    &*format!("[{}] ", "PRIVÉE"),
                    0.0,
                    TextFormat {
                        color: Color32::YELLOW,
                        ..Default::default()
                    },
                );
            }
            job.append(
                &*format!("[{}] ", user_role.to_string()),
                0.0,
                TextFormat {
                    color: Color32::from_hex(user_role.get_color()).unwrap_or(Color32::WHITE),
                    ..Default::default()
                },
            );
            job.append(
                &*format!("\n{}", user.global_name),
                0.0,
                TextFormat::default(),
            );

            ui.label(job);
        });
        ui.horizontal(|ui| {
            ui.add(avatar_image);

            ui.add_space(ui.min_rect().min.x * 0.065);

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                state_badge(ui, &review_message.set_state);
            });
        });
        ui.separator();

        ui.label(RichText::new(user_role.role_summary()).strong().color(Color32::from_hex(user_role.get_color()).unwrap_or(Color32::WHITE)));

        ui.separator();

        ui.label(RichText::new("Commentaire : ").text_style(TextStyle::Name("heading3".into())).strong());

        let mut cache: RwLockWriteGuard<CommonMarkCache> = cache.write().expect("Can't access common_mark_cache");

        egui::ScrollArea::vertical().id_source((&review_message.content, &review_message.date)).show(ui, |ui| {
            CommonMarkViewer::new().show(ui, &mut cache, &review_message.content);
        });
        ui.separator();

        ui.vertical_centered(|ui| {
            ui.label(format!("Publié le {}", formatted_date));
        });
    }).response
}
