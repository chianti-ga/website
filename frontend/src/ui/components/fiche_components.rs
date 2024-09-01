use std::ops::Add;
use std::str::SplitWhitespace;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use chrono::{NaiveDateTime, TimeZone, Utc};
use eframe::emath::Align;
use egui::scroll_area::ScrollBarVisibility;
use egui::text::LayoutJob;
use egui::{Button, Color32, FontSelection, Image, Layout, OpenUrl, Response, RichText, TextBuffer, TextEdit, TextFormat, TextStyle};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use strum::IntoEnumIterator;
use web_time::{SystemTime, UNIX_EPOCH};

use shared::discord::User;
use shared::fiche_rp::{FicheRP, FicheState, FicheVersion, Job, MedicRank, MedicRole, ScienceRank, ScienceRole, SecurityRank, SecurityRole};
use shared::user::FrontAccount;

use crate::app::AUTH_INFO;
use crate::app::{get_string, image_resolver, AuthInfo};
use crate::backend_handler::{post_ficherp, post_ficherp_modif};

pub fn ficherp_bubble(ui: &mut egui::Ui, ficherp: &FicheRP, user: &User) -> Response {
    let avatar_url = format!("https://cdn.discordapp.com/avatars/{}/{}.png?size=128", &user.id, user.avatar);

    let avatar_image: Image = Image::new(avatar_url).fit_to_original_size(0.5).maintain_aspect_ratio(true).rounding(100.0);
    let datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(ficherp.submission_date as i64, 0));

    let formatted_date = datetime.format("%d-%m-%Y %H:%M:%S").to_string();
    ui.vertical(|ui| {
        ui.vertical_centered(|ui| {
            ui.label(format!("{} | Fiche RP de {} | {}", user.global_name, ficherp.name, formatted_date));
        });
        ui.horizontal(|ui| {
            ui.add(avatar_image);

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                state_badge(ui, &ficherp.state);
            });
        });

        ui.separator();
        let mut layout_job = LayoutJob::default();

        RichText::new("Job : ").strong().text_style(TextStyle::Name("heading3".into())).append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::LEFT);

        layout_job.append(&*ficherp.job.to_string(), 0.0, TextFormat { ..Default::default() });
        ui.label(layout_job);
    }).response
}

pub fn ficherp_viewer(ui: &mut egui::Ui, ficherp: &FicheRP, job_text_buffer: &mut String, user: &User, cache: Arc<RwLock<CommonMarkCache>>, is_viewing: &mut bool, mut is_editing_existing_fiche: &mut bool, new_fiche: &mut Option<FicheRP>, selected_fiche_account: &mut Option<(FrontAccount, FicheRP)>) {
    let avatar_url = format!("https://cdn.discordapp.com/avatars/{}/{}.png?size=128", &user.id, user.avatar);

    let avatar_image: Image = Image::new(avatar_url).fit_to_original_size(0.5).maintain_aspect_ratio(true).rounding(100.0);
    let datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(ficherp.submission_date as i64, 0));

    let formatted_date = datetime.format("%d-%m-%Y %H:%M:%S").to_string();
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.vertical_centered(|ui| {
                ui.label(format!("{} | Fiche RP de {} | {}", user.global_name, ficherp.name, formatted_date));
            });
        });

        ui.vertical_centered(|ui| {
            let history_btn = Button::image_and_text(Image::new(image_resolver("history.svg")).fit_to_original_size(1.0).shrink_to_fit().maintain_aspect_ratio(true), "Historique de la fiche");
            if ui.add(history_btn).clicked() {
                *is_viewing = true;
            }
        });

        ui.horizontal(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.add(avatar_image);
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                state_badge(ui, &ficherp.state);
            });
        });

        ui.separator();

        let mut layout_job = LayoutJob::default();

        RichText::new("Job : ").strong().text_style(TextStyle::Name("heading3".into()))
                               .append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::LEFT);

        layout_job.append(&*ficherp.job.to_string(), 0.0, TextFormat { ..Default::default() });
        ui.label(layout_job);

        ui.separator();

        let mut cache: RwLockWriteGuard<CommonMarkCache> = cache.write().expect("Can't access common_mark_cache");

        let height = ui.available_size().y * 0.90;

        egui::ScrollArea::vertical().max_height(height).id_source("scoll_text_viewer").show(ui, |ui| {
            ui.label(RichText::new("Description physique : ").strong().text_style(TextStyle::Name("heading3".into())));

            CommonMarkViewer::new("desc_viewer").show(ui, &mut cache, &ficherp.description);
            ui.separator();

            ui.label(RichText::new("Lore : ").strong().text_style(TextStyle::Name("heading3".into())));

            CommonMarkViewer::new("lore_viewer").show(ui, &mut cache, &ficherp.lore);
        });

        ui.add_space(5.0);

        ui.vertical_centered(|ui| {
            if ficherp.state == FicheState::RequestModification {
                let auth_binding: Arc<RwLock<AuthInfo>> = AUTH_INFO.clone();
                let auth_lock: RwLockReadGuard<AuthInfo> = auth_binding.read().unwrap();
                let logged_user_account: FrontAccount = auth_lock.clone().account.unwrap();
                if *user == logged_user_account.discord_user {
                    if ui.button(get_string("ficherp.modif.invite")).clicked() {
                        *new_fiche = Option::from(selected_fiche_account.clone().unwrap().1);
                        //Set Job::Other inner string to the job buffer for editing
                        if let Some(job_string) = ficherp.job.get_other_string() {
                            *job_text_buffer = job_string.clone();
                        }

                        *selected_fiche_account = None;
                        *is_editing_existing_fiche = true;
                    }
                }
            }
        });
    });
}

pub fn ficherp_edit(ui: &mut egui::Ui, ficherp: &mut FicheRP, is_previewing: &mut bool, job_text_buffer: &mut String, is_editing_existing_fiche: &mut bool, background_image: &mut Option<String>) -> bool {
    let mut can_be_closed: bool = false;
    let mut valid_entries: bool = false;

    let binding: Arc<RwLock<AuthInfo>> = AUTH_INFO.clone();
    let auth_lock: RwLockReadGuard<AuthInfo> = binding.read().unwrap();
    let account = auth_lock.clone().account.unwrap();
    let user: User = account.discord_user;
    let avatar_url = format!("https://cdn.discordapp.com/avatars/{}/{}.png?size=128", &user.id, &user.avatar);

    let avatar_image: Image = Image::new(avatar_url).fit_to_original_size(0.5).maintain_aspect_ratio(true).rounding(100.0);

    ui.vertical(|ui| {
        ui.vertical_centered(|ui| {
            ui.label(format!("{} | Composition de votre Fiche RP", &user.global_name));
        });
        ui.horizontal(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.add(avatar_image);
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                //ui.vertical(|ui| {
                let preview_btn = Button::image_and_text(Image::new(image_resolver("eye_preview.svg")).fit_to_original_size(1.0).shrink_to_fit().maintain_aspect_ratio(true), "Preview de la fiche");
                let markdown_btn = Button::image_and_text(Image::new(image_resolver("markdown-mark.svg")).fit_to_original_size(1.0).shrink_to_fit().maintain_aspect_ratio(true), "Formatage ?");

                if ui.add(preview_btn).clicked() {
                    *is_previewing = true;
                }

                if ui.add(markdown_btn).clicked() {
                    ui.ctx().open_url(OpenUrl::new_tab("https://commonmark.org/help/"));
                }
                //});
            });
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label(RichText::new("Nom : ").text_style(TextStyle::Name("heading3".into())));
            let name_text_edit = TextEdit::singleline(&mut ficherp.name).char_limit(70).hint_text("François LePortier");
            ui.add(name_text_edit);
            if ficherp.name.chars().count() < 5 {
                valid_entries = false;
                ui.label(RichText::new("⚠ Trop court... (<5 caractères)").strong().color(Color32::YELLOW));
            } else {
                valid_entries = true;
            }
        });

        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("Job : ").text_style(TextStyle::Name("heading3".into())));
            let mut job_string: String = ficherp.job.to_string();
            let mut words: SplitWhitespace = job_string.split_whitespace();

            egui::ComboBox::from_label("").selected_text(words.next().unwrap()).show_ui(ui, |ui| {
                ui.selectable_value(&mut ficherp.job, Job::ClassD, Job::ClassD.to_string());
                ui.selectable_value(&mut ficherp.job, Job::Security(SecurityRole::SecurityOfficier(SecurityRank::Rct)), "Officier de Sécurité");
                ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Scientific(ScienceRank::Beginner)), "Science");
                ui.selectable_value(&mut ficherp.job, Job::Medic(MedicRole::Doctor(MedicRank::Beginner)), "Médecine");
                ui.selectable_value(&mut ficherp.job, Job::Chaos, Job::Chaos.to_string());
                ui.selectable_value(&mut ficherp.job, Job::Other("".to_string()), "Autres");
            });

            if ficherp.job.to_string().contains("Sécurité") {
                let mut role_string: String = ficherp.job.get_security_role().unwrap().to_string();
                role_string = truncate_at_char_boundary(role_string, 20);
                ui.label("Role");
                egui::ComboBox::from_id_source("role_combo").selected_text(role_string).show_ui(ui, |ui| {
                    ui.selectable_value(&mut ficherp.job, Job::Security(SecurityRole::SecurityOfficier(SecurityRank::Rct)), "Officier de Sécurité");
                    ui.selectable_value(&mut ficherp.job, Job::Security(SecurityRole::Gunsmith(SecurityRank::Rct)), "Armurier");
                });

                let mut level: String = ficherp.job.clone().get_security_level().unwrap().to_string();
                level = truncate_at_char_boundary(level, 20);
                ui.label("Rang");
                egui::ComboBox::from_id_source("rang_combo").selected_text(&level).show_ui(ui, |ui| {
                    match &ficherp.job {
                        Job::Security(role) => {
                            match role {
                                SecurityRole::SecurityOfficier(_) => {
                                    for level in SecurityRank::iter() {
                                        let mut level_string = level.to_string();
                                        level_string = truncate_at_char_boundary(level_string, 20);
                                        ui.selectable_value(&mut ficherp.job, Job::Security(SecurityRole::SecurityOfficier(level.clone())), level_string);
                                    }
                                }
                                SecurityRole::Gunsmith(_) => {
                                    for level in SecurityRank::iter() {
                                        let mut level_string = level.to_string();
                                        level_string = truncate_at_char_boundary(level_string, 20);
                                        ui.selectable_value(&mut ficherp.job, Job::Security(SecurityRole::Gunsmith(level.clone())), level_string);
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                });
            }

            if ficherp.job.to_string().contains("Science") {
                ui.label("Role");
                egui::ComboBox::from_id_source("role_combo").selected_text(ficherp.job.get_science_role().unwrap().to_string()).show_ui(ui, |ui| {
                    ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Scientific(ScienceRank::Beginner)), "Scientifique");
                    ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Researcher(ScienceRank::Beginner)), "Chercheur");
                    //ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Supervisor(ScienceLevel::Beginner)), role.to_string());
                });
                let rank: String = ficherp.job.clone().get_science_level().unwrap().to_string();
                ui.label("Rang");
                egui::ComboBox::from_id_source("rank_combo").selected_text(&rank).show_ui(ui, |ui| {
                    match &ficherp.job {
                        Job::Science(role) => {
                            match role {
                                ScienceRole::Scientific(_) => {
                                    ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Scientific(ScienceRank::Beginner)), "Junior");
                                    ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Scientific(ScienceRank::NoLevel)), "Confirmé");
                                    ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Scientific(ScienceRank::Senior)), "Senior");
                                }
                                ScienceRole::Researcher(_) => {
                                    ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Researcher(ScienceRank::Beginner)), "Junior");
                                    ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Researcher(ScienceRank::NoLevel)), "Confirmé");
                                    ui.selectable_value(&mut ficherp.job, Job::Science(ScienceRole::Researcher(ScienceRank::Senior)), "Senior");
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                });
            }

            if ficherp.job.to_string().contains("Médecine") {
                let mut role_string: String = ficherp.job.get_medic_role().unwrap().to_string();
                role_string = truncate_at_char_boundary(role_string, 20);
                ui.label("Role");
                egui::ComboBox::from_id_source("role_combo").selected_text(role_string).show_ui(ui, |ui| {
                    ui.selectable_value(&mut ficherp.job, Job::Medic(MedicRole::Nurse), "Infirmier");
                    ui.selectable_value(&mut ficherp.job, Job::Medic(MedicRole::Doctor(MedicRank::Beginner)), "Médecin");
                    ui.selectable_value(&mut ficherp.job, Job::Medic(MedicRole::Surgeon(MedicRank::Beginner)), "Chirurgien");
                    ui.selectable_value(&mut ficherp.job, Job::Medic(MedicRole::Psychiatrist(MedicRank::Beginner)), "Psychiatre");
                    ui.selectable_value(&mut ficherp.job, Job::Medic(MedicRole::Manager), "Responsable");
                    ui.selectable_value(&mut ficherp.job, Job::Medic(MedicRole::DirectorAdj), "Directeur Adjoint");
                    ui.selectable_value(&mut ficherp.job, Job::Medic(MedicRole::Director), "Directeur");
                });

                if let Some(level) = ficherp.job.clone().get_medic_level() {
                    let mut level_string: String = level.to_string();
                    level_string = truncate_at_char_boundary(level_string, 20);

                    ui.label("Rang");
                    egui::ComboBox::from_id_source("rank_combo").selected_text(&level_string).show_ui(ui, |ui| {
                        match &ficherp.job {
                            Job::Medic(role) => {
                                match role {
                                    MedicRole::Psychiatrist(_) => {
                                        for level in MedicRank::iter() {
                                            let mut level_string = level.to_string();
                                            level_string = truncate_at_char_boundary(level_string, 20);
                                            ui.selectable_value(&mut ficherp.job, Job::Medic(MedicRole::Psychiatrist(level.clone())), level_string);
                                        }
                                    }
                                    MedicRole::Surgeon(_) => {
                                        for level in MedicRank::iter() {
                                            let mut level_string = level.to_string();
                                            level_string = truncate_at_char_boundary(level_string, 20);
                                            ui.selectable_value(&mut ficherp.job, Job::Medic(MedicRole::Surgeon(level.clone())), level_string);
                                        }
                                    }
                                    MedicRole::Doctor(_) => {
                                        for level in MedicRank::iter() {
                                            let mut level_string = level.to_string();
                                            level_string = truncate_at_char_boundary(level_string, 20);
                                            ui.selectable_value(&mut ficherp.job, Job::Medic(MedicRole::Doctor(level.clone())), level_string);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    });
                }
            }

            if ficherp.job.to_string().contains("Autres") {
                let job_text_edit = TextEdit::singleline(job_text_buffer).char_limit(70).hint_text("Technicien");
                ui.add(job_text_edit);
                if job_text_buffer.chars().count() < 5 {
                    valid_entries = false;
                    ui.label(RichText::new("⚠ Trop court... (<5 caractères)").strong().color(Color32::YELLOW));
                } else {
                    valid_entries = true;
                }
            }
        });

        ui.separator();

        /** SYNTAX HIGHLIGHT **/

        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            let mut theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
            let mut layout_job =
                egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, string, "markdown");
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };

        ui.label(RichText::new("Description physique : ").strong().text_style(TextStyle::Name("heading3".into())));

        if ficherp.description.chars().count() > 15000 {
            valid_entries = false;
            ui.label(RichText::new("⚠ Trop long (>15 000 caractères)").strong().color(Color32::YELLOW));
        } else if ficherp.description.chars().count() < 200 {
            valid_entries = false;
            ui.label(RichText::new("⚠ Trop court... (< 200 cractères)").strong().color(Color32::YELLOW));
        } else {
            valid_entries = true;
        }

        let height = ui.available_size().y * 0.25;

        egui::ScrollArea::vertical().id_source("scroll_physic").max_height(height).scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible).show(ui, |ui| {
            let size = ui.available_size();
            ui.add_sized(size, TextEdit::multiline(&mut ficherp.description).code_editor().layouter(&mut layouter));
        });

        ui.label(RichText::new("Lore : ").strong().text_style(TextStyle::Name("heading3".into())));

        if ficherp.lore.chars().count() > 15000 {
            valid_entries = false;
            ui.label(RichText::new("⚠ Trop long (>15 000 caractères)").strong().color(Color32::YELLOW));
        } else if ficherp.lore.chars().count() < 200 {
            valid_entries = false;
            ui.label(RichText::new("⚠ Trop court... (< 200 cractères)").strong().color(Color32::YELLOW));
        } else {
            valid_entries = true;
        }

        let height = ui.available_size().y * 0.85;

        egui::ScrollArea::vertical().id_source("scroll_lore").max_height(height).scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible).show(ui, |ui| {
            let size = ui.available_size();
            ui.add_sized(size, TextEdit::multiline(&mut ficherp.lore).code_editor().layouter(&mut layouter));
        });
        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            if *is_editing_existing_fiche {
                if ui.button(get_string("ficherp.modif.invite")).clicked() {
                    if ficherp.job.to_string().contains("Autres") {
                        ficherp.job = Job::Other(job_text_buffer.clone());
                    }

                    ficherp.submission_date = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

                    ficherp.version.push(FicheVersion {
                        name: ficherp.name.clone(),
                        job: ficherp.job.clone(),
                        description: ficherp.description.clone(),
                        lore: ficherp.lore.clone(),
                        submission_date: ficherp.submission_date.clone(),
                    });

                    post_ficherp_modif(ficherp);
                    *background_image = Option::from(image_resolver("checkmark_expo.svg"));
                    can_be_closed = true;
                }
            } else {
                ui.add_enabled_ui(valid_entries, |ui| {
                    if ui.button(get_string("ficherp.create.submit")).clicked() {
                        if ficherp.job.to_string().contains("Autres") {
                            ficherp.job = Job::Other(job_text_buffer.clone());
                        }

                        ficherp.submission_date = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

                        ficherp.version.push(FicheVersion {
                            name: ficherp.name.clone(),
                            job: ficherp.job.clone(),
                            description: ficherp.description.clone(),
                            lore: ficherp.lore.clone(),
                            submission_date: ficherp.submission_date.clone(),
                        });

                        post_ficherp(ficherp);
                        *background_image = Option::from(image_resolver("checkmark_expo.svg"));
                        can_be_closed = true;
                    }
                });
            }
        });
    });
    can_be_closed
}

pub fn ficherp_viewer_window(ui: &mut egui::Ui, ficherp: &FicheRP, user: &User, cache: Arc<RwLock<CommonMarkCache>>) {
    let datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(ficherp.submission_date as i64, 0));

    let formatted_date = datetime.format("%d-%m-%Y %H:%M:%S").to_string();
    ui.vertical(|ui| {
        ui.vertical_centered(|ui| {
            ui.label(format!("{} | Fiche RP de {} | {}", user.global_name, ficherp.name, formatted_date));
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

pub fn ficherp_history_viewer_window(ui: &mut egui::Ui, ficherp: &FicheRP, selected_fiche_account_version: &mut FicheVersion, user: &User, cache: Arc<RwLock<CommonMarkCache>>) {
    ui.horizontal(|ui| {
        let label: RichText = RichText::new("Version").strong().text_style(TextStyle::Name("heading3".into()));

        let datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(selected_fiche_account_version.clone().submission_date as i64, 0));
        let selected_text: String = datetime.format("%d-%m-%Y %H:%M:%S").to_string();

        egui::ComboBox::from_label(label).selected_text(&selected_text).show_ui(ui, |ui| {
            ficherp.version.iter().for_each(|fiche_version: &FicheVersion| {
                let datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(fiche_version.submission_date as i64, 0));
                let formatted_date = datetime.format("%d-%m-%Y").to_string();
                ui.selectable_value(selected_fiche_account_version, fiche_version.clone().to_owned(), formatted_date);
            });
        });
    });

    let datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(selected_fiche_account_version.submission_date as i64, 0));

    let formatted_date = datetime.format("%d-%m-%Y %H:%M:%S").to_string();
    ui.vertical(|ui| {
        ui.vertical_centered(|ui| {
            ui.label(format!("{} | Fiche RP de {} | {}", user.global_name, selected_fiche_account_version.name, formatted_date));
        });

        ui.separator();

        let mut layout_job = LayoutJob::default();

        RichText::new("Job : ")
            .text_style(TextStyle::Name("heading3".into())).strong()
            .append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::LEFT);

        layout_job.append(&*selected_fiche_account_version.job.to_string(), 0.0, TextFormat { ..Default::default() });
        ui.label(layout_job);

        ui.separator();

        let mut cache: RwLockWriteGuard<CommonMarkCache> = cache.write().expect("Can't access common_mark_cache");

        egui::ScrollArea::vertical().id_source("scoll_text_viewer").show(ui, |ui| {
            ui.label(RichText::new("Description physique : ").strong().text_style(TextStyle::Name("heading3".into())));

            CommonMarkViewer::new("desc_viewer").show(ui, &mut cache, &selected_fiche_account_version.description);
            ui.separator();

            ui.label(RichText::new("Lore : ").strong().text_style(TextStyle::Name("heading3".into())));

            CommonMarkViewer::new("lore_viewer").show(ui, &mut cache, &selected_fiche_account_version.lore);
            ui.separator();
        });
    });
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

    let badge: Image = Image::new(image_resolver(format!("badges/{}", img_to_load).as_str())).fit_to_original_size(1.0).maintain_aspect_ratio(true);
    ui.add(badge.clone());
}

fn truncate_at_char_boundary(s: String, index: usize) -> String {
    // Ensure the split index is at a character boundary
    let truncate_index = s.char_indices().nth(index).map(|(i, _)| i).unwrap_or(s.len());
    s[..truncate_index].to_string()
}
