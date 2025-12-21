//! # Token Types (أنواع الرموز)
//!
//! Defines all token types for the Ocean shell lexer.

use std::fmt;

/// Position in source code for error reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self { start, end, line, column }
    }
}

/// A token with its kind and position
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, lexeme: String) -> Self {
        Self { kind, span, lexeme }
    }
}

/// Token types for shell commands
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // ═══════════════════════════════════════════════════════════
    // Words and Strings (الكلمات والنصوص)
    // ═══════════════════════════════════════════════════════════

    /// A word (command name or argument)
    /// Examples: `اطبع`, `مرحبا`, `file.txt`
    Word(String),

    /// A quoted string literal
    /// Supports: "text", 'text', «text»
    String(String),

    // ═══════════════════════════════════════════════════════════
    // Pipe and Redirection Operators (عوامل الأنابيب وإعادة التوجيه)
    // ═══════════════════════════════════════════════════════════

    /// Pipe operator: |
    Pipe,

    /// Redirect output: >
    RedirectOut,

    /// Redirect input: <
    RedirectIn,

    /// Append output: >>
    Append,

    // ═══════════════════════════════════════════════════════════
    // Logical Operators (العوامل المنطقية)
    // ═══════════════════════════════════════════════════════════

    /// Logical AND: &&
    And,

    /// Logical OR: ||
    Or,

    // ═══════════════════════════════════════════════════════════
    // Control Operators (عوامل التحكم)
    // ═══════════════════════════════════════════════════════════

    /// Semicolon for command separation: ;
    Semicolon,

    /// Background execution: &
    Background,

    // ═══════════════════════════════════════════════════════════
    // Special Tokens (رموز خاصة)
    // ═══════════════════════════════════════════════════════════

    /// Newline character
    Newline,

    /// End of input
    Eof,

    /// Error token for error recovery
    Error(String),
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Word(s) => write!(f, "Word({})", s),
            TokenKind::String(s) => write!(f, "String(\"{}\")", s),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::RedirectOut => write!(f, ">"),
            TokenKind::RedirectIn => write!(f, "<"),
            TokenKind::Append => write!(f, ">>"),
            TokenKind::And => write!(f, "&&"),
            TokenKind::Or => write!(f, "||"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Background => write!(f, "&"),
            TokenKind::Newline => write!(f, "\\n"),
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::Error(msg) => write!(f, "Error: {}", msg),
        }
    }
}
