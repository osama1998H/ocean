//! # Utilities Module (وحدة الأدوات المساعدة)
//!
//! Common utilities used across the shell.

pub mod arabic;
pub mod colors;

pub use arabic::{
    shape_arabic,
    shape_if_arabic,
    contains_arabic,
    // RTL alignment functions
    enable_rtl_mode,
    right_align,
};

// Additional RTL functions available for future use
#[allow(unused_imports)]
pub use arabic::{get_terminal_width, display_width, format_rtl, println_rtl};

// Color utilities
pub use colors::colored_prompt;
#[allow(unused_imports)]
pub use colors::{colored_error, colorize_entry};

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
