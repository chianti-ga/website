use std::sync::{Arc, RwLock, RwLockWriteGuard};

use chrono::{NaiveDateTime, TimeZone, Utc};
use eframe::emath::Align;
use egui::{Image, Layout, Response, RichText, TextStyle};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use strum::IntoEnumIterator;
use web_time::{SystemTime, UNIX_EPOCH};

use shared::discord::User;
use shared::fiche_rp::{FicheState, FicheStateIter, ReviewMessage};
use shared::permissions::DiscordRole;
use shared::user::FrontAccount;

use crate::app::ALL_ACCOUNTS;
use crate::backend_handler::post_comment;
use crate::ui::components::fiche_components::state_badge;

pub fn edit_comment_window(ui: &mut egui::Ui, ficherp_id: String, review_message: &mut ReviewMessage, cache: Arc<RwLock<CommonMarkCache>>) {
    let datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(review_message.date as i64, 0));

    let formatted_date = datetime.format("%d-%m-%Y").to_string();
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.checkbox(&mut review_message.is_comment, "Commentaire ?");
            if review_message.is_comment {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.checkbox(&mut review_message.is_private, "Commentaire privé ?");
                    review_message.set_state = FicheState::Comment;
                });
            } else {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    egui::ComboBox::from_label("Statue de la fiche").selected_text(review_message.set_state.get_text()).show_ui(ui, |ui| {
                        let state_iter: FicheStateIter = FicheState::iter();
                        state_iter.filter(|state| state != &FicheState::Comment).for_each(|state| {
                            ui.selectable_value(&mut review_message.set_state, state.clone(), state.get_text());
                        });
                    });
                });
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

            CommonMarkViewer::new("comment_viewer").show(ui, &mut cache, &review_message.content);
        });

        ui.vertical_centered(|ui| {
            if ui.button("Poster le commentaire/réponse").clicked() {
                review_message.date = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                post_comment(review_message, ficherp_id);
            }
        });
    });
}

pub fn comment_bubble(ui: &mut egui::Ui, review_message: &ReviewMessage, cache: Arc<RwLock<CommonMarkCache>>) -> Response {
    let binding = ALL_ACCOUNTS.read().unwrap();
    let account: &FrontAccount = binding.iter().find(|front_account| front_account.discord_user.id == review_message.discord_id).unwrap();
    let user: &User = &account.discord_user;
    let avatar_url: String = format!("https://cdn.discordapp.com/avatars/{}/{}.png?size=128", &user.id, user.avatar);

    let avatar_image: Image = Image::new(avatar_url).fit_to_original_size(0.5).maintain_aspect_ratio(true).rounding(100.0);
    let datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(review_message.date as i64, 0));

    let formatted_date = datetime.format("%d-%m-%Y").to_string();

    let role: DiscordRole = if let Some(roles) = DiscordRole::from_role_ids(&account.discord_roles) {
        let mut final_role = DiscordRole::Unknown;

        if roles.contains(&DiscordRole::LeadScenarist) {
            final_role = DiscordRole::LeadScenarist
        } else if roles.contains(&DiscordRole::Scenarist) {
            final_role = DiscordRole::Scenarist
        } else if roles.contains(&DiscordRole::Moderator) {
            final_role = DiscordRole::Scenarist
        };
        final_role
    } else {
        DiscordRole::Unknown
    };

    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.add(avatar_image);
            ui.add_space(ui.min_rect().min.x);

            ui.label(format!("[{}] {} ", role, user.username));
            ui.add_space(ui.min_rect().min.x);

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                state_badge(ui, &review_message.set_state);
            });
        });

        ui.separator();

        ui.label(RichText::new("Commentaire : ").text_style(TextStyle::Name("heading3".into())).strong());

        let mut cache: RwLockWriteGuard<CommonMarkCache> = cache.write().expect("Can't access common_mark_cache");

        egui::ScrollArea::vertical().id_source(&review_message.content).show(ui, |ui| {
            CommonMarkViewer::new("comment_viewer").show(ui, &mut cache, &review_message.content);
        });

        ui.label(formatted_date);
    }).response
}