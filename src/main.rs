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
mod repl;

use std::env;
use std::path::PathBuf;

use rustyline::error::ReadlineError;
use rustyline::{Config, Editor};

use lexer::Lexer;
use parser::Parser;
use executor::{Executor, CommandResult};
use repl::OceanHelper;
use utils::{shape_arabic, shape_if_arabic, contains_arabic, enable_rtl_mode, right_align, colored_prompt};

const SHELL_NAME: &str = "محيط";
const VERSION: &str = "0.1.0";

fn main() {
    // Try to enable VTE RTL mode, track if we need padding fallback
    let vte_rtl_supported = enable_rtl_mode();
    let use_padding = !vte_rtl_supported;

    // Print welcome message
    print_welcome(use_padding);

    // Create executor with RTL padding setting
    let mut executor = Executor::new(use_padding);

    // Initialize rustyline with auto-completion
    let config = Config::builder()
        .auto_add_history(true)
        .build();

    let mut rl: Editor<OceanHelper, _> = match Editor::with_config(config) {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!("Failed to initialize readline: {}", e);
            // Fallback to basic REPL
            run_basic_repl(&mut executor, use_padding);
            return;
        }
    };

    // Set the completion helper
    rl.set_helper(Some(OceanHelper::new()));

    // Load history from file
    let history_path = dirs::home_dir()
        .map(|h| h.join(".ocean_history"))
        .unwrap_or_else(|| PathBuf::from(".ocean_history"));
    let _ = rl.load_history(&history_path);

    // Main REPL loop
    loop {
        // Build colored prompt
        let cwd = env::current_dir()
            .map(|p| shorten_path(&p))
            .unwrap_or_else(|_| "?".to_string());

        let prompt = if use_padding {
            // For RTL terminals, use right-aligned prompt
            right_align(&colored_prompt(&shape_arabic(SHELL_NAME), &cwd))
        } else {
            colored_prompt(&shape_arabic(SHELL_NAME), &cwd)
        };

        // Read input using rustyline
        let input = match rl.readline(&prompt) {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C - just continue
                continue;
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D - exit
                print_rtl_line(&shape_arabic("مع السلامة! (Goodbye!)"), use_padding);
                break;
            }
            Err(err) => {
                print_rtl_line(&format!("Error: {:?}", err), use_padding);
                continue;
            }
        };

        // Skip empty input
        if input.trim().is_empty() {
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
                // Save history before exit
                let _ = rl.save_history(&history_path);
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
                // Print errors in red
                use colored::Colorize;
                let error_msg = shape_if_arabic(&msg).red().to_string();
                print_rtl_line(&error_msg, use_padding);
            }
            CommandResult::None => {}
        }
    }

    // Save history on normal exit
    let _ = rl.save_history(&history_path);
}

fn run_basic_repl(executor: &mut Executor, use_padding: bool) {
    use std::io::{self, Write};

    loop {
        // Print prompt with current directory
        let cwd = env::current_dir()
            .map(|p| shorten_path(&p))
            .unwrap_or_else(|_| "?".to_string());

        let prompt = format!("{} [{}]> ", shape_arabic(SHELL_NAME), cwd);

        if use_padding {
            print!("{}", right_align(&prompt));
        } else {
            print!("{}", prompt);
        }
        io::stdout().flush().unwrap();

        // Read input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // EOF (Ctrl+D)
                println!();
                print_rtl_line(&shape_arabic("مع السلامة! (Goodbye!)"), use_padding);
                return;
            }
            Ok(_) => {}
            Err(_) => continue,
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Tokenize
        let mut lexer = Lexer::new(input);
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

fn print_rtl_line(text: &str, use_padding: bool) {
    if use_padding && contains_arabic(text) {
        println!("{}", right_align(text));
    } else {
        println!("{}", text);
    }
}

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


fn shorten_path(path: &PathBuf) -> String {
    if let Some(home) = dirs::home_dir() {
        if let Ok(relative) = path.strip_prefix(&home) {
            return format!("~/{}", relative.display());
        }
    }
    path.display().to_string()
}

