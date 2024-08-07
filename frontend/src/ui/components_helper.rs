use std::ops::Add;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use chrono::{NaiveDateTime, TimeZone, Utc};
use eframe::emath::Align;
use egui::{Button, FontSelection, Image, Layout, Response, RichText, TextBuffer, TextFormat, TextStyle};
use egui::scroll_area::ScrollBarVisibility;
use egui::text::LayoutJob;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use strum::IntoEnumIterator;

use shared::discord::User;
use shared::fiche_rp::{FicheRP, FicheState, Job, ReviewMessage, ScienceLevel, ScienceRole, SecurityLevel, SecurityRole};
use shared::permissions::DiscordRole;
use shared::user::FrontAccount;

use crate::app::{AuthInfo, image_resolver};
use crate::app::AUTH_INFO;

pub fn ficherp_bubble(ui: &mut egui::Ui, ficherp: &FicheRP, user: &User) -> Response {
    let avatar_url = format!("https://cdn.discordapp.com/avatars/{}/{}.png?size=128", &user.id, user.avatar);

    let avatar_image: Image = Image::new(avatar_url).fit_to_original_size(0.5).maintain_aspect_ratio(true).rounding(100.0);
    let datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(ficherp.submission_date as i64, 0));

    let formatted_date = datetime.format("%d-%m-%Y").to_string();
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.add(avatar_image);
            ui.add_space(ui.min_rect().min.x);
            ui.label(format!("{} | Fiche RP de {} | {}", user.username, ficherp.name, formatted_date));
            ui.add_space(ui.min_rect().min.x);
            state_badge(ui, &ficherp.state);
        });

        ui.separator();
        let mut layout_job = LayoutJob::default();

        RichText::new("Job : ").strong()
                               .text_style(TextStyle::Name("heading3".into()))
                               .append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::LEFT);

        layout_job.append(&*ficherp.job.to_string(), 0.0, TextFormat { ..Default::default() });
        ui.label(layout_job);
    }).response
}

pub fn ficherp_viewer(ui: &mut egui::Ui, ficherp: &FicheRP, user: &User, cache: Arc<RwLock<CommonMarkCache>>) {
    let avatar_url = format!("https://cdn.discordapp.com/avatars/{}/{}.png?size=128", &user.id, user.avatar);

    let avatar_image: Image = Image::new(avatar_url).fit_to_original_size(0.5).maintain_aspect_ratio(true).rounding(100.0);
    let datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(ficherp.submission_date as i64, 0));

    let formatted_date = datetime.format("%d-%m-%Y").to_string();
    ui.vertical(|ui| {
        ui.vertical_centered(|ui| {
            ui.label(format!("{} | Fiche RP de {} | {}", user.username, ficherp.name, formatted_date));
        });
        ui.horizontal(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.add(avatar_image);
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let history_btn = Button::image_and_text(Image::new(image_resolver("history.svg")).fit_to_original_size(1.0).shrink_to_fit().maintain_aspect_ratio(true), "Historique de la fiche");
                if ui.add(history_btn).clicked() {}

                state_badge(ui, &ficherp.state);
            });
        });

        ui.separator();

        let mut layout_job = LayoutJob::default();

        RichText::new("Job : ").strong()
                               .text_style(TextStyle::Name("heading3".into()))
                               .append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::LEFT);

        layout_job.append(&*ficherp.job.to_string(), 0.0, TextFormat { ..Default::default() });
        ui.label(layout_job);

        ui.separator();

        let mut cache: RwLockWriteGuard<CommonMarkCache> = cache.write().expect("Can't access common_mark_cache");

        egui::ScrollArea::vertical().id_source("scoll_text_viewer").show(ui, |ui| {
            ui.label(RichText::new("Description physique : ").strong().text_style(TextStyle::Name("heading3".into())));

            CommonMarkViewer::new("desc_viewer").show(ui, &mut cache, &ficherp.description);
            ui.separator();

            ui.label(RichText::new("Lore : ").strong().text_style(TextStyle::Name("heading3".into())));

            CommonMarkViewer::new("lore_viewer").show(ui, &mut cache, &ficherp.lore);
            ui.separator();
        });
    });
}

pub fn ficherp_edit(ui: &mut egui::Ui, ficherp: &mut FicheRP, is_previewing: &mut bool) {
    let binding: Arc<RwLock<AuthInfo>> = AUTH_INFO.clone();
    let auth_lock: RwLockReadGuard<AuthInfo> = binding.read().unwrap();
    let account = auth_lock.clone().account.unwrap();
    let user: User = account.discord_user;
    let avatar_url = format!("https://cdn.discordapp.com/avatars/{}/{}.png?size=128", &user.id, &user.avatar);

    let avatar_image: Image = Image::new(avatar_url).fit_to_original_size(0.5).maintain_aspect_ratio(true).rounding(100.0);

    ui.vertical(|ui| {
        ui.vertical_centered(|ui| {
            ui.label(format!("{} | Composition de votre Fiche RP", &user.username));
        });
        ui.horizontal(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.add(avatar_image);
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let preview_btn = Button::image_and_text(Image::new(image_resolver("eye_preview.svg")).fit_to_original_size(1.0).shrink_to_fit().maintain_aspect_ratio(true), "Preview de la fiche");
                if ui.add(preview_btn).clicked() {
                    *is_previewing = true;
                }
            });
        });

        ui.separator();

        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("Job : ").text_style(TextStyle::Name("heading3".into())));
            let mut job_string: String = ficherp.job.to_string();
            job_string = truncate_at_char_boundary(job_string, 20);
            egui::ComboBox::from_label("").selected_text(job_string).show_ui(ui, |ui| {
                ui.selectable_value(&mut ficherp.job, Job::ClassD, Job::ClassD.to_string());
                ui.selectable_value(&mut ficherp.job, Job::Security(SecurityRole::SecurityOfficier(SecurityLevel::Rct)), "Officier de Sécurité");
                ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Scientific(ScienceLevel::Beginner)), "Science");
                ui.selectable_value(&mut ficherp.job, Job::Doctor, Job::Doctor.to_string());
                ui.selectable_value(&mut ficherp.job, Job::Chaos, Job::Chaos.to_string());
            });

            if ficherp.job.to_string().contains("Sécurité") {
                let mut role_string: String = ficherp.job.get_security_role().unwrap().to_string();
                role_string = truncate_at_char_boundary(role_string, 20);
                egui::ComboBox::from_label("Role").selected_text(role_string).show_ui(ui, |ui| {
                    ui.selectable_value(&mut ficherp.job, Job::Security(SecurityRole::SecurityOfficier(SecurityLevel::Rct)), "Officier de Sécurité");
                    ui.selectable_value(&mut ficherp.job, Job::Security(SecurityRole::TacticalAgent(SecurityLevel::Rct)), "Agent Tactique");
                });

                let mut level: String = ficherp.job.clone().get_security_level().unwrap().to_string();
                level = truncate_at_char_boundary(level, 20);
                egui::ComboBox::from_label("Rang").selected_text(&level).show_ui(ui, |ui| {
                    match &ficherp.job {
                        Job::Security(role) => {
                            match role {
                                SecurityRole::SecurityOfficier(_) => {
                                    for level in SecurityLevel::iter() {
                                        let mut level_string = level.to_string();
                                        level_string = truncate_at_char_boundary(level_string, 20);
                                        ui.selectable_value(&mut ficherp.job, Job::Security(SecurityRole::SecurityOfficier(level.clone())), level_string);
                                    }
                                }
                                SecurityRole::TacticalAgent(_) => {
                                    for level in SecurityLevel::iter() {
                                        let mut level_string = level.to_string();
                                        level_string = truncate_at_char_boundary(level_string, 20);
                                        ui.selectable_value(&mut ficherp.job, Job::Security(SecurityRole::TacticalAgent(level.clone())), level_string);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                });
            }
            if ficherp.job.to_string().contains("Science") {
                egui::ComboBox::from_label("Role").selected_text(ficherp.job.get_science_role().unwrap().to_string()).show_ui(ui, |ui| {
                    ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Scientific(ScienceLevel::Beginner)), "Scientifique");
                    ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Researcher(ScienceLevel::Beginner)), "Chercheur");
                    //ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Supervisor(ScienceLevel::Beginner)), role.to_string());
                });
                let rank: String = ficherp.job.clone().get_science_level().unwrap().to_string();
                egui::ComboBox::from_label("Rang").selected_text(&rank).show_ui(ui, |ui| {
                    match &ficherp.job {
                        Job::Science(role) => {
                            match role {
                                ScienceRole::Scientific(_) => {
                                    ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Scientific(ScienceLevel::Beginner)), "Junior");
                                    ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Scientific(ScienceLevel::Confirmed)), "Confirmé");
                                    ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Scientific(ScienceLevel::Senior)), "Senior");
                                }
                                ScienceRole::Researcher(_) => {
                                    ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Researcher(ScienceLevel::Beginner)), "Junior");
                                    ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Researcher(ScienceLevel::Confirmed)), "Confirmé");
                                    ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Researcher(ScienceLevel::Senior)), "Senior");
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                });
            }
        });

        ui.separator();

        egui::ScrollArea::vertical().scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible).show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Description physique : ").strong().text_style(TextStyle::Name("heading3".into())));

                let mut size = ui.available_size();
                size.y = size.y / 3.0;
                size.x *= 0.98;

                ui.add_sized(size, egui::TextEdit::multiline(&mut ficherp.description));

                ui.label(RichText::new("Lore : ").strong().text_style(TextStyle::Name("heading3".into())));

                let mut size = ui.available_size();
                size.y *= 0.95;
                size.x *= 0.98;
                ui.add_sized(size, egui::TextEdit::multiline(&mut ficherp.lore));

                ui.label("TESTTTTTTTT")
            });
        });
    });
}

pub fn ficherp_viewer_window(ui: &mut egui::Ui, ficherp: &FicheRP, user: &User, cache: Arc<RwLock<CommonMarkCache>>) {
    let datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(ficherp.submission_date as i64, 0));

    let formatted_date = datetime.format("%d-%m-%Y").to_string();
    ui.vertical(|ui| {
        ui.vertical_centered(|ui| {
            ui.label(format!("{} | Fiche RP de {} | {}", user.username, ficherp.name, formatted_date));
        });

        ui.separator();

        let mut layout_job = LayoutJob::default();

        RichText::new("Job : ")
            .text_style(TextStyle::Name("heading3".into())).strong()
            .append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::LEFT);

        layout_job.append(&*ficherp.job.to_string(), 0.0, TextFormat { ..Default::default() });
        ui.label(layout_job);

        ui.separator();

        let mut cache: RwLockWriteGuard<CommonMarkCache> = cache.write().expect("Can't access common_mark_cache");

        egui::ScrollArea::vertical().id_source("scoll_text_viewer").show(ui, |ui| {
            ui.label(RichText::new("Description physique : ").strong().text_style(TextStyle::Name("heading3".into())));

            CommonMarkViewer::new("desc_viewer").show(ui, &mut cache, &ficherp.description);
            ui.separator();

            ui.label(RichText::new("Lore : ").strong().text_style(TextStyle::Name("heading3".into())));

            CommonMarkViewer::new("lore_viewer").show(ui, &mut cache, &ficherp.lore);
            ui.separator();
        });
    });
}

pub fn edit_comment_window(ui: &mut egui::Ui, review_message: &mut ReviewMessage, cache: Arc<RwLock<CommonMarkCache>>) {
    let datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(review_message.date as i64, 0));

    let formatted_date = datetime.format("%d-%m-%Y").to_string();
    ui.vertical(|ui| {
        ui.label(RichText::new("Commentaire : ").text_style(TextStyle::Name("heading3".into())).strong());

        let mut cache: RwLockWriteGuard<CommonMarkCache> = cache.write().expect("Can't access common_mark_cache");

        egui::ScrollArea::vertical().id_source("scoll_comment_viewer").show(ui, |ui| {
            let mut size = ui.available_size();
            size.y = size.y / 3.0;
            size.x *= 0.97;

            ui.add_sized(size, egui::TextEdit::multiline(&mut review_message.content));

            CommonMarkViewer::new("comment_viewer").show(ui, &mut cache, &review_message.content);
        });

        ui.label(formatted_date);
    });
}

pub fn comment_bubble(ui: &mut egui::Ui, review_message: &ReviewMessage, cache: Arc<RwLock<CommonMarkCache>>) -> Response {
    let account: &FrontAccount = &review_message.account;
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
            state_badge(ui, &review_message.set_state);
        });

        ui.separator();

        ui.label(RichText::new("Commentaire : ").text_style(TextStyle::Name("heading3".into())).strong());

        let mut cache: RwLockWriteGuard<CommonMarkCache> = cache.write().expect("Can't access common_mark_cache");

        egui::ScrollArea::vertical().id_source("scoll_comment_viewer").show(ui, |ui| {
            CommonMarkViewer::new("comment_viewer").show(ui, &mut cache, &review_message.content);
        });

        ui.label(formatted_date);
    }).response
}

pub fn state_badge(ui: &mut egui::Ui, state: &FicheState) {
    let img_to_load: &str = match state {
        FicheState::Waiting => "waiting.svg",
        FicheState::RequestModification => "modif.svg",
        FicheState::StaffValidated => "conform.svg",
        FicheState::Accepted => "accepted.svg",
        FicheState::Refused => "refused.svg",
        FicheState::Comment => "comment.svg"
    };

    let badge: Image = Image::new(image_resolver(format!("badges/{}", img_to_load).as_str())).fit_to_original_size(1.0).shrink_to_fit().maintain_aspect_ratio(true);
    ui.add(badge.clone());
    //ui.ctx().forget_image(badge.uri().unwrap());
}

fn truncate_at_char_boundary(s: String, index: usize) -> String {
    // Ensure the split index is at a character boundary
    let truncate_index = s.char_indices().nth(index).map(|(i, _)| i).unwrap_or(s.len());
    s[..truncate_index].to_string()
}
