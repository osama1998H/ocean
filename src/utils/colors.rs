//! # Color Utilities (أدوات الألوان)
//!
//! Provides color formatting for terminal output.

use colored::Colorize;

/// Format the shell prompt with colors
///
/// # Arguments
/// * `shell_name` - The shell name (shaped Arabic text)
/// * `cwd` - Current working directory path
///
/// # Returns
/// Colored prompt string
pub fn colored_prompt(shell_name: &str, cwd: &str) -> String {
    format!(
        "{} [{}]> ",
        shell_name.cyan().bold(),
        cwd.blue()
    )
}

/// Format an error message in red
#[allow(dead_code)]
pub fn colored_error(msg: &str) -> String {
    msg.red().to_string()
}

/// Format a success message in green
#[allow(dead_code)]
pub fn colored_success(msg: &str) -> String {
    msg.green().to_string()
}

/// Colorize a file/directory entry for ls output
///
/// # Arguments
/// * `name` - File or directory name
/// * `is_dir` - Whether it's a directory
/// * `is_exec` - Whether it's executable (Unix only)
///
/// # Returns
/// Colorized name string
#[allow(dead_code)]
pub fn colorize_entry(name: &str, is_dir: bool, is_exec: bool) -> String {
    if is_dir {
        format!("{}/", name.blue().bold())
    } else if is_exec {
        name.green().bold().to_string()
    } else {
        name.to_string()
    }
}

/// Colorize file entry with symlink support
#[allow(dead_code)]
pub fn colorize_entry_full(name: &str, is_dir: bool, is_exec: bool, is_symlink: bool) -> String {
    if is_symlink {
        name.magenta().to_string()
    } else if is_dir {
        format!("{}/", name.blue().bold())
    } else if is_exec {
        name.green().bold().to_string()
    } else {
        name.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colored_prompt() {
        let prompt = colored_prompt("محيط", "~/test");
        assert!(!prompt.is_empty());
        // Contains the actual text (without ANSI codes check)
        assert!(prompt.contains("[") && prompt.contains("]"));
    }

    #[test]
    fn test_colorize_directory() {
        let result = colorize_entry("mydir", true, false);
        assert!(result.contains("mydir"));
        assert!(result.ends_with('/') || result.contains('/'));
    }
}
