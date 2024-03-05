use std::env;

pub enum Information {
    Hostname,
    WindowManager,
    SessionType,
    User
}

pub fn get_information(info: &Information) -> Option<String> {
    match info {
        Information::Hostname => match hostname::get() {
            Ok(h) => h.into_string().ok(),
            Err(_) => None
        },
        Information::WindowManager => env::var("XDG_CURRENT_DESKTOP").ok(),
        Information::SessionType => env::var("XDG_SESSION_TYPE").ok(),
        Information::User => env::var("USER").ok(),
    }
}
