use std::fmt;

pub struct NotificationWindow {
    expire_time: f64,
    content: String,
    notif_type: NotifType,
}

pub enum NotifType {
    INFO(String),
    WARN(String),
    ERROR(String),
    OK(String),
}

impl fmt::Display for NotifType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotifType::INFO(str) => write!(f, "{}", str),
            NotifType::WARN(str) => write!(f, "{}", str),
            NotifType::ERROR(str) => write!(f, "{}", str),
            NotifType::OK(str) => write!(f, "{}", str),
        }
    }
}
