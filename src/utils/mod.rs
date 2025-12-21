//! # Utilities Module (وحدة الأدوات المساعدة)
//!
//! Common utilities used across the shell.

use std::path::PathBuf;

/// Expand ~ to home directory
pub fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            return home.join(path[1..].trim_start_matches('/'));
        }
    }
    PathBuf::from(path)
}

/// Shorten path for display (replace home with ~)
#[allow(dead_code)]
pub fn shorten_path(path: &PathBuf) -> String {
    if let Some(home) = dirs::home_dir() {
        if let Ok(relative) = path.strip_prefix(&home) {
            return format!("~/{}", relative.display());
        }
    }
    path.display().to_string()
}
