use crate::memory;
use std::env;
use sysinfo::System;

pub enum Information {
    Hostname,
    WindowManager,
    User,
    Shell,
    Memory,
}

impl Information {
    pub fn get_information(&self) -> Option<String> {
        match self {
            Self::Hostname => hostname::get().ok()?.into_string().ok(),
            Self::WindowManager => {
                let wm = env::var("XDG_CURRENT_DESKTOP").ok();
                let session = env::var("XDG_SESSION_TYPE").ok();

                match (wm, session) {
                    (Some(wm), Some(session)) => Some(format!("{} ({})", wm, session)),
                    (Some(wm), None) => Some(wm),
                    (None, Some(session)) => Some(session),
                    (None, None) => None,
                }
            }
            Self::User => env::var("USER").ok(),
            Self::Shell => env::var("SHELL").ok()?.split('/').last().map(String::from),
            Self::Memory => {
                let mut sys = System::new();
                sys.refresh_memory();

                let total_memory = memory::Memory::new(sys.total_memory() as f64);
                let used_memory = memory::Memory::new(sys.used_memory() as f64);

                let percent = used_memory.value / total_memory.value * 100.0;

                Some(format!(
                    "{}/{} ({:.1}%)",
                    used_memory.human_size(),
                    total_memory.human_size(),
                    percent
                ))
            }
        }
    }
}
