use egui::{Color32, hex_color};
use lazy_static::lazy_static;

lazy_static!(
    pub static ref WAITING_BADGE_TEXT:Color32 = hex_color!("#00FFF2");
pub static ref WAITING_BADGE_FILL:Color32 = hex_color!("#00827B");
pub static ref WAITING_BADGE_BORDER:Color32 = hex_color!("#00C2B9");

pub static ref REFUSED_BADGE_TEXT:Color32 = hex_color!("#FF0000");
pub static ref REFUSED_BADGE_FILL:Color32 = hex_color!("#C20000");
pub static ref REFUSED_BADGE_BORDER:Color32 = hex_color!("#825000");

pub static ref ACCEPTED_BADGE_TEXT:Color32 = hex_color!("#00FF09");
pub static ref ACCEPTED_BADGE_FILL:Color32 = hex_color!("#008204");
pub static ref ACCEPTED_BADGE_BORDER:Color32 = hex_color!("#00C206");

pub static ref MODIF_BADGE_TEXT:Color32 = hex_color!("#FF00AA");
pub static ref MODIF_BADGE_FILL:Color32 = hex_color!("#820056");
pub static ref MODIF_BADGE_BORDER:Color32 = hex_color!("#C20082");

pub static ref CONFORM_BADGE_TEXT:Color32 = hex_color!("#0048FF");
pub static ref CONFORM_BADGE_FILL:Color32 = hex_color!("#002582");
pub static ref CONFORM_BADGE_BORDER:Color32 = hex_color!("#0037C2");

pub static ref COMMENT_BADGE_TEXT:Color32 = hex_color!("#595959");
pub static ref COMMENT_BADGE_FILL:Color32 = hex_color!("#262626");
pub static ref COMMENT_BADGE_BORDER:Color32 = hex_color!("#404040");

pub static ref TRAITEMENT_BADGE_TEXT:Color32 = hex_color!("#FF9D00");
pub static ref TRAITEMENT_BADGE_FILL:Color32 = hex_color!("#825000");
pub static ref TRAITEMENT_BADGE_BORDER:Color32 = hex_color!("#C27800");
);
