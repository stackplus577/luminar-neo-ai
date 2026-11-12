//! Windows registry read helpers for Luminar Neo Ai.

pub const APP_KEY: &str = "Software\\Luminar\\Luminar Neo Ai";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Luminar Neo Ai";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Luminar Neo Ai") },
        RegValue { name: "Version",     value: String::from("v2.4.19") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Luminar Neo Ai") },
        RegValue { name: "Publisher",       value: String::from("Luminar") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Luminar Neo Ai\\uninstall.exe\"") },
    ]
}
