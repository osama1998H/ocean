//! # REPL Module (وحدة حلقة القراءة-التقييم-الطباعة)
//!
//! Handles readline functionality including:
//! - Command history
//! - Auto-completion for commands and files
//! - Line editing

mod completer;

#[allow(unused_imports)]
pub use completer::OceanCompleter;
pub use completer::OceanHelper;
