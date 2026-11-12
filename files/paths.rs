//! Windows-specific path helpers used across Luminar Neo Ai.

use std::path::PathBuf;

pub fn program_files() -> PathBuf {
    env_dir("PROGRAMFILES", "C:\\Program Files")
}

pub fn program_data() -> PathBuf {
    env_dir("PROGRAMDATA", "C:\\ProgramData")
}

pub fn local_appdata() -> PathBuf {
    env_dir("LOCALAPPDATA", "C:\\Users\\Default\\AppData\\Local")
}

pub fn roaming_appdata() -> PathBuf {
    env_dir("APPDATA", "C:\\Users\\Default\\AppData\\Roaming")
}

pub fn product_install_dir() -> PathBuf {
    program_files().join("Luminar Neo Ai")
}

pub fn product_user_dir() -> PathBuf {
    local_appdata().join("Luminar Neo Ai")
}

pub fn product_cache_dir() -> PathBuf {
    product_user_dir().join("cache")
}

fn env_dir(key: &str, fallback: &str) -> PathBuf {
    PathBuf::from(std::env::var(key).unwrap_or_else(|_| String::from(fallback)))
}
