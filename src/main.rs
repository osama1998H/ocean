//! # محيط (Ocean) - Arabic Shell
//!
//! A modern, lightweight shell with Arabic commands.
//! Part of the Tarqeem Project - Arabic Programming Ecosystem.
//!
//! ## Architecture
//!
//! The shell uses a three-stage pipeline:
//! 1. **Lexer** - Tokenizes input with Arabic support
//! 2. **Parser** - Builds an AST from tokens
//! 3. **Executor** - Executes commands from the AST
//!
//! ## Commands (الأوامر)
//! - `اطبع` (echo) - Print text
//! - `اعرض` (ls) - List files
//! - `انتقل` (cd) - Change directory
//! - `اين` (pwd) - Current directory
//! - `امسح` (clear) - Clear screen
//! - `خروج` (exit) - Exit shell
//! - `مساعدة` (help) - Show help

mod commands;
mod lexer;
mod parser;
mod executor;
mod utils;

use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

use lexer::Lexer;
use parser::Parser;
use executor::{Executor, CommandResult};
use utils::{shape_arabic, shape_if_arabic, contains_arabic, enable_rtl_mode, right_align};

/// Shell name in Arabic
const SHELL_NAME: &str = "محيط";
/// Shell version
const VERSION: &str = "0.1.0";

fn main() {
    // Try to enable VTE RTL mode, track if we need padding fallback
    let vte_rtl_supported = enable_rtl_mode();
    let use_padding = !vte_rtl_supported;

    // Print welcome message
    print_welcome(use_padding);

    // Create executor with RTL padding setting
    let mut executor = Executor::new(use_padding);

    // Main REPL loop
    loop {
        // Print prompt with current directory
        print_prompt(use_padding);

        // Read input
        let input = match read_input() {
            Some(line) => line,
            None => continue,
        };

        // Skip empty input
        if input.is_empty() {
            continue;
        }

        // Tokenize
        let mut lexer = Lexer::new(&input);
        let tokens = lexer.tokenize();

        // Parse
        let mut parser = Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(cmd) => cmd,
            Err(e) => {
                print_rtl_line(&e.to_string(), use_padding);
                continue;
            }
        };

        // Execute
        let result = executor.execute(ast);
        match result {
            CommandResult::Exit(code) => {
                print_rtl_line(&shape_arabic("مع السلامة! (Goodbye!)"), use_padding);
                std::process::exit(code);
            }
            CommandResult::Success(output) => {
                if !output.is_empty() {
                    // Print each line with RTL alignment if needed
                    for line in output.lines() {
                        print_rtl_line(&shape_if_arabic(line), use_padding);
                    }
                }
            }
            CommandResult::Error(msg) => {
                print_rtl_line(&shape_if_arabic(&msg), use_padding);
            }
            CommandResult::None => {}
        }
    }
}

/// Print a line with RTL alignment if needed
fn print_rtl_line(text: &str, use_padding: bool) {
    if use_padding && contains_arabic(text) {
        println!("{}", right_align(text));
    } else {
        println!("{}", text);
    }
}

/// Print welcome message when shell starts
fn print_welcome(use_padding: bool) {
    // Build the welcome banner as a single block
    // The banner is a fixed-width box that should be displayed as-is
    let banner = format!(
        r#"
╔═══════════════════════════════════════════════════════════╗
║                                                           ║
║   {}  ║
║       Arabic Shell v{}                                  ║
║                                                           ║
║   {}                           ║
║   {} ║
║                                                           ║
║   {} ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝
"#,
        shape_arabic("🌊  محيط (Ocean) - الصدفة العربية"),
        VERSION,
        shape_arabic("مشروع ترقيم - Tarqeem Project"),
        shape_arabic("اكتب 'مساعدة' للمساعدة | Type 'مساعدة' for help"),
        shape_arabic("✨ دعم الأنابيب والتوجيه: cmd1 | cmd2, cmd > file")
    );

    if use_padding {
        // Right-align the entire banner as a block
        for line in banner.lines() {
            println!("{}", right_align(line));
        }
    } else {
        println!("{}", banner);
    }
}

/// Print the shell prompt
fn print_prompt(use_padding: bool) {
    // Get current directory
    let cwd = env::current_dir()
        .map(|p| shorten_path(&p))
        .unwrap_or_else(|_| "?".to_string());

    // Build prompt: محيط [path]>
    // Shape the Arabic shell name for proper display
    let prompt = format!("{} [{}]> ", shape_arabic(SHELL_NAME), cwd);

    if use_padding {
        // Right-align the prompt for RTL
        print!("{}", right_align(&prompt));
    } else {
        print!("{}", prompt);
    }
    io::stdout().flush().unwrap();
}

/// Shorten path for display (replace home with ~)
fn shorten_path(path: &PathBuf) -> String {
    if let Some(home) = dirs::home_dir() {
        if let Ok(relative) = path.strip_prefix(&home) {
            return format!("~/{}", relative.display());
        }
    }
    path.display().to_string()
}

/// Read a line of input from the user
fn read_input() -> Option<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => {
            // EOF (Ctrl+D)
            println!();
            Some("خروج".to_string())
        }
        Ok(_) => Some(input.trim().to_string()),
        Err(_) => None,
    }
}
