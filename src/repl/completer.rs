//! # Auto-Completion Module (وحدة الإكمال التلقائي)
//!
//! Provides tab completion for:
//! - Built-in commands (Arabic and English)
//! - File and directory paths

use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};
use std::borrow::Cow;
use std::path::Path;

/// Ocean shell helper combining completion, hints, and highlighting
#[derive(Default)]
pub struct OceanHelper {
    completer: OceanCompleter,
}

impl OceanHelper {
    pub fn new() -> Self {
        Self {
            completer: OceanCompleter::new(),
        }
    }
}

impl Helper for OceanHelper {}

impl Validator for OceanHelper {}

impl Hinter for OceanHelper {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}

impl Highlighter for OceanHelper {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Owned(format!("\x1b[90m{}\x1b[0m", hint))
    }
}

impl Completer for OceanHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        self.completer.complete(line, pos, ctx)
    }
}

/// Auto-completer for Ocean shell commands and file paths
pub struct OceanCompleter {
    /// List of built-in commands (Arabic, English)
    commands: Vec<(&'static str, &'static str)>,
}

impl Default for OceanCompleter {
    fn default() -> Self {
        Self::new()
    }
}

impl OceanCompleter {
    /// Create a new completer with all built-in commands
    pub fn new() -> Self {
        Self {
            commands: vec![
                ("خروج", "exit"),
                ("مساعدة", "help"),
                ("اطبع", "echo"),
                ("امسح", "clear"),
                ("اين", "pwd"),
                ("انتقل", "cd"),
                ("اعرض", "ls"),
                ("اقرأ", "cat"),
                ("انشئ", "mkdir"),
                ("المس", "touch"),
                ("احذف", "rm"),
                ("انسخ", "cp"),
                ("انقل", "mv"),
                ("ابحث", "grep"),
                ("صلاحيات", "chmod"),
                ("مالك", "chown"),
                ("رابط", "ln"),
                ("اصدار", "version"),
            ],
        }
    }

    /// Complete a command name (Arabic or English)
    fn complete_command(&self, partial: &str) -> Vec<Pair> {
        let mut matches = Vec::new();
        let partial_lower = partial.to_lowercase();

        for (ar, en) in &self.commands {
            // Match Arabic commands
            if ar.starts_with(partial) {
                matches.push(Pair {
                    display: ar.to_string(),
                    replacement: ar.to_string(),
                });
            }
            // Match English commands
            if en.starts_with(&partial_lower) {
                matches.push(Pair {
                    display: en.to_string(),
                    replacement: en.to_string(),
                });
            }
        }

        matches
    }

    /// Complete a file or directory path
    fn complete_path(&self, partial: &str) -> Vec<Pair> {
        let mut matches = Vec::new();

        // Handle empty input - list current directory
        let (dir_path, prefix) = if partial.is_empty() {
            (Path::new("."), "")
        } else if partial.ends_with('/') || partial.ends_with('\\') {
            (Path::new(partial), "")
        } else {
            let path = Path::new(partial);
            (
                path.parent().unwrap_or(Path::new(".")),
                path.file_name().and_then(|s| s.to_str()).unwrap_or(""),
            )
        };

        // Handle ~ expansion
        let expanded_dir = if partial.starts_with('~') {
            if let Some(home) = dirs::home_dir() {
                if partial == "~" || partial == "~/" {
                    home
                } else {
                    home.join(partial.trim_start_matches("~/"))
                        .parent()
                        .unwrap_or(&home)
                        .to_path_buf()
                }
            } else {
                dir_path.to_path_buf()
            }
        } else {
            dir_path.to_path_buf()
        };

        // Read directory entries
        if let Ok(entries) = std::fs::read_dir(&expanded_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let name = entry.file_name().to_string_lossy().to_string();

                // Skip hidden files unless prefix starts with .
                if name.starts_with('.') && !prefix.starts_with('.') {
                    continue;
                }

                if name.starts_with(prefix) {
                    let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);

                    // Build the full replacement path
                    let replacement = if partial.is_empty() || partial == "." {
                        if is_dir {
                            format!("{}/", name)
                        } else {
                            name.clone()
                        }
                    } else if partial.ends_with('/') {
                        if is_dir {
                            format!("{}{}/", partial, name)
                        } else {
                            format!("{}{}", partial, name)
                        }
                    } else if dir_path == Path::new(".") {
                        if is_dir {
                            format!("{}/", name)
                        } else {
                            name.clone()
                        }
                    } else {
                        let parent = partial.rsplit_once('/').map(|(p, _)| p).unwrap_or("");
                        if is_dir {
                            format!("{}/{}/", parent, name)
                        } else {
                            format!("{}/{}", parent, name)
                        }
                    };

                    let display = if is_dir {
                        format!("{}/", name)
                    } else {
                        name
                    };

                    matches.push(Pair {
                        display,
                        replacement,
                    });
                }
            }
        }

        matches
    }

    /// Main completion function
    pub fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let line_to_cursor = &line[..pos];
        let words: Vec<&str> = line_to_cursor.split_whitespace().collect();

        // Determine if we're completing a command or a path
        if words.is_empty() || (words.len() == 1 && !line_to_cursor.ends_with(' ')) {
            // Complete command name
            let partial = words.first().copied().unwrap_or("");
            let start = line_to_cursor
                .rfind(char::is_whitespace)
                .map(|i| i + 1)
                .unwrap_or(0);
            Ok((start, self.complete_command(partial)))
        } else {
            // Complete file path (for command arguments)
            let partial = if line_to_cursor.ends_with(' ') {
                ""
            } else {
                words.last().copied().unwrap_or("")
            };
            let start = if line_to_cursor.ends_with(' ') {
                pos
            } else {
                line_to_cursor
                    .rfind(char::is_whitespace)
                    .map(|i| i + 1)
                    .unwrap_or(0)
            };
            Ok((start, self.complete_path(partial)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_completion() {
        let completer = OceanCompleter::new();
        let matches = completer.complete_command("ec");
        assert!(matches.iter().any(|p| p.replacement == "echo"));
    }

    #[test]
    fn test_arabic_command_completion() {
        let completer = OceanCompleter::new();
        let matches = completer.complete_command("اط");
        assert!(matches.iter().any(|p| p.replacement == "اطبع"));
    }

    #[test]
    fn test_empty_command_completion() {
        let completer = OceanCompleter::new();
        let matches = completer.complete_command("");
        // Should return all commands (18 pairs = 36 total)
        assert!(matches.len() >= 18);
    }
}
