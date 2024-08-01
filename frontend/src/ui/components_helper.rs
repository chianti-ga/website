use std::ops::Add;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

use chrono::{NaiveDateTime, TimeZone, Utc};
use eframe::emath::Align;
use egui::{FontSelection, Image, InnerResponse, RichText, TextFormat, TextStyle};
use egui::text::LayoutJob;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

use shared::discord::User;
use shared::fiche_rp::{FicheRP, FicheState};

use crate::app::image_resolver;

/*
pub fn ficherp_bubble(ui: &mut egui::Ui, ficherp:&FicheRP, user:&User) -> InnerResponse<Response> {
    let avatar_url = format!("https://cdn.discordapp.com/avatars/{}/{}.png?size=128", &user.id, user.avatar);

    let avatar_image: Image = Image::new(avatar_url).fit_to_original_size(0.5).maintain_aspect_ratio(true);
    let datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(ficherp.submission_date as i64, 0));

    let formatted_date = datetime.format("%d-%m-%Y").to_string();

    // Put the buttons and label on the same row:
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.add(avatar_image);
            ui.add_space(10.0);
            ui.label(format!("{} | Fiche RP de {} | {}", user.username, ficherp.name, formatted_date));
            ui.add_space(10.0);
            state_badge(ui, &ficherp.state);
            ui.add_space(10.0);
        });
        ui.separator()

    })
}*/

pub fn ficherp_bubble(ui: &mut egui::Ui, ficherp: &FicheRP, user: &User) -> InnerResponse<()> {
    let avatar_url = format!("https://cdn.discordapp.com/avatars/{}/{}.png?size=128", &user.id, user.avatar);

    let avatar_image: Image = Image::new(avatar_url).fit_to_original_size(0.5).maintain_aspect_ratio(true).rounding(100.0);
    let datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(ficherp.submission_date as i64, 0));

    let formatted_date = datetime.format("%d-%m-%Y").to_string();
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.add(avatar_image);
            ui.add_space(10.0);
            ui.label(format!("{} | Fiche RP de {} | {}", user.username, ficherp.name, formatted_date));
            ui.add_space(10.0);
            state_badge(ui, &ficherp.state);
            ui.add_space(10.0);
        });
        ui.separator();
        let mut layout_job = LayoutJob::default();

        RichText::new("Job : ")
            .text_style(TextStyle::Name("heading3".into()))
            .append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::LEFT);

        layout_job.append(&*ficherp.job.to_string(), 0.0, TextFormat { ..Default::default() });
        ui.label(layout_job);
    })
}

pub fn ficherp_viewer(ui: &mut egui::Ui, ficherp: &FicheRP, user: &User, cache: Arc<RwLock<CommonMarkCache>>) {
    let avatar_url = format!("https://cdn.discordapp.com/avatars/{}/{}.png?size=128", &user.id, user.avatar);

    let avatar_image: Image = Image::new(avatar_url).fit_to_original_size(0.5).maintain_aspect_ratio(true).rounding(100.0);
    let datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(ficherp.submission_date as i64, 0));

    let formatted_date = datetime.format("%d-%m-%Y").to_string();
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.add(avatar_image);
            ui.add_space(10.0);
            ui.label(format!("{} | Fiche RP de {} | {}", user.username, ficherp.name, formatted_date));
            ui.add_space(10.0);
            state_badge(ui, &ficherp.state);
            ui.add_space(10.0);
        });

        ui.separator();

        let mut layout_job = LayoutJob::default();

        RichText::new("Job : ")
            .text_style(TextStyle::Name("heading3".into()))
            .append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::LEFT);

        layout_job.append(&*ficherp.job.to_string(), 0.0, TextFormat { ..Default::default() });
        ui.label(layout_job);

        ui.separator();

        let mut cache: RwLockWriteGuard<CommonMarkCache> = cache.write().expect("Can't access CommonMarkCache");

        egui::ScrollArea::vertical().show(ui, |ui| {
            CommonMarkViewer::new("lore_viewer").show(ui, &mut cache, &*ficherp.lore);
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
    let badge: Image = Image::new(image_resolver(img_to_load)).fit_to_original_size(1.0).maintain_aspect_ratio(true);
    ui.vertical(|ui| {
        ui.add(badge);
    });
}