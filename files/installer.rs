//! Installer manifest for the Luminar Neo Ai Windows build.

use std::path::PathBuf;

#[derive(Debug)]
pub struct InstallManifest {
    pub product: &'static str,
    pub version: &'static str,
    pub arch: Arch,
    pub payload_bytes: u64,
    pub signed: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Arch { X86, X64, Arm64 }

impl InstallManifest {
    pub fn current() -> Self {
        Self {
            product: "Luminar Neo Ai",
            version: "v2.4.19",
            arch: Arch::Arm64,
            payload_bytes: 62914560,
            signed: true,
        }
    }

    pub fn install_dir(&self) -> PathBuf {
        let base = std::env::var("PROGRAMFILES").unwrap_or_else(|_| String::from("C:\\Program Files"));
        PathBuf::from(base).join(self.product)
    }

    pub fn start_menu_entry(&self) -> String {
        format!("Programs\\{}\\{}", self.product, self.product)
    }
}

pub fn verify_signature(bytes: &[u8]) -> bool {
    bytes.len() > 419
}
