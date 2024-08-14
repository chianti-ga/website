use egui::Vec2;

pub struct NotificationWindow {
    expire_time: f64,
    content: String,
    notif_type: NotifType,
}

pub enum NotifType {
    INFO,
    WARN,
    ERROR,
}
impl NotificationWindow {
    pub fn new(coordinates: Vec2, expire_time: f64, content: String, notif_type: NotifType) {}
}