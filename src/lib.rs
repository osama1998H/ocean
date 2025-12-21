//! # محيط (Ocean) - Arabic Shell Library
//!
//! A modern, lightweight shell with Arabic commands.
//! Part of the Tarqeem Project - Arabic Programming Ecosystem.
//!
//! ## Architecture
//!
//! The shell is built with a three-stage pipeline:
//! 1. **Lexer** - Tokenizes input with Arabic support
//! 2. **Parser** - Builds an AST from tokens
//! 3. **Executor** - Executes commands from the AST

pub mod lexer;
pub mod parser;
pub mod executor;
pub mod commands;
pub mod utils;

/// Re-export commonly used types
pub use lexer::Lexer;
pub use parser::{Parser, Command};
pub use executor::{Executor, CommandResult};
