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

/// Shell name in Arabic
const SHELL_NAME: &str = "محيط";
/// Shell version
const VERSION: &str = "0.1.0";

fn main() {
    // Print welcome message
    print_welcome();

    // Create executor
    let mut executor = Executor::new();

    // Main REPL loop
    loop {
        // Print prompt with current directory
        print_prompt();

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
                eprintln!("{}", e);
                continue;
            }
        };

        // Execute
        let result = executor.execute(ast);
        match result {
            CommandResult::Exit(code) => {
                println!("مع السلامة! (Goodbye!)");
                std::process::exit(code);
            }
            CommandResult::Success(output) => {
                if !output.is_empty() {
                    print!("{}", output);
                }
            }
            CommandResult::Error(msg) => {
                eprintln!("{}", msg);
            }
            CommandResult::None => {}
        }
    }
}

/// Print welcome message when shell starts
fn print_welcome() {
    println!();
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║                                                           ║");
    println!("║   🌊  محيط (Ocean) - الصدفة العربية                        ║");
    println!("║       Arabic Shell v{}                                  ║", VERSION);
    println!("║                                                           ║");
    println!("║   مشروع ترقيم - Tarqeem Project                           ║");
    println!("║   اكتب 'مساعدة' للمساعدة | Type 'مساعدة' for help          ║");
    println!("║                                                           ║");
    println!("║   ✨ دعم الأنابيب والتوجيه: cmd1 | cmd2, cmd > file        ║");
    println!("║                                                           ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();
}

/// Print the shell prompt
fn print_prompt() {
    // Get current directory
    let cwd = env::current_dir()
        .map(|p| shorten_path(&p))
        .unwrap_or_else(|_| "?".to_string());

    // Print prompt: محيط [path]>
    print!("{} [{}]> ", SHELL_NAME, cwd);
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
