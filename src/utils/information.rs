use crate::utils::memory;
use std::env;
use sysinfo::System;

pub enum Information {
    Hostname,
    WindowManager,
    User,
    Shell,
    Memory,
}

pub fn get_information(info: &Information) -> Option<String> {
    match info {
        Information::Hostname => match hostname::get() {
            Ok(h) => h.into_string().ok(),
            Err(_) => None,
        },
        Information::WindowManager => {
            let wm = env::var("XDG_CURRENT_DESKTOP").ok();
            let session = env::var("XDG_SESSION_TYPE").ok();

            match (wm, session) {
                (Some(wm), Some(session)) => Some(format!("{} ({})", wm, session)),
                (Some(wm), None) => Some(wm),
                (None, Some(session)) => Some(session),
                _ => None,
            }
        }
        Information::User => env::var("USER").ok(),
        Information::Shell => match env::var("SHELL") {
            Ok(shell) => shell
                .split('/')
                .collect::<Vec<_>>()
                .last()
                .map(|&s| s.to_string()),
            Err(_) => None,
        },
        Information::Memory => {
            let mut sys = System::new();
            sys.refresh_memory();

            let total_memory = memory::Memory::new(sys.total_memory() as f64);
            let used_memory = memory::Memory::new(sys.used_memory() as f64);

            let percent = used_memory.value / total_memory.value * 100.0;

            Some(format!(
                "{}/{} ({:.1}%)",
                used_memory.human_size().format(),
                total_memory.human_size().format(),
                percent
            ))
        }
    }
}
