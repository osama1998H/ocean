//! # Lexer Module (وحدة التحليل اللغوي)
//!
//! Tokenizes shell input with full Arabic support.
//! Inspired by the Tarqeem language lexer.

pub mod token;

pub use token::{Token, TokenKind, Span};

use unicode_normalization::UnicodeNormalization;

/// Lexer for tokenizing shell commands
///
/// Supports:
/// - Arabic and English text
/// - Quoted strings with escape sequences
/// - Arabic quotation marks («»)
/// - Pipe and redirection operators
/// - Command chaining (&&, ||, ;)
pub struct Lexer {
    /// Source characters (NFC normalized)
    source: Vec<char>,
    /// Current position in source
    position: usize,
    /// Start position of current token
    token_start: usize,
    /// Current line number (1-indexed)
    line: usize,
    /// Current column number (1-indexed)
    column: usize,
    /// Column at start of current token
    token_start_column: usize,
}

impl Lexer {
    /// Create a new lexer from source text
    ///
    /// Performs NFC Unicode normalization for consistent Arabic handling.
    pub fn new(source: &str) -> Self {
        // Normalize Unicode to NFC form (like Tarqeem)
        let normalized: String = source.nfc().collect();
        Self {
            source: normalized.chars().collect(),
            position: 0,
            token_start: 0,
            line: 1,
            column: 1,
            token_start_column: 1,
        }
    }

    /// Tokenize the entire input
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();
            let is_eof = matches!(token.kind, TokenKind::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }

        tokens
    }

    /// Get the next token
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.token_start = self.position;
        self.token_start_column = self.column;

        if self.is_at_end() {
            return self.make_token(TokenKind::Eof);
        }

        let c = self.advance();

        match c {
            // Newline
            '\n' => self.make_token(TokenKind::Newline),

            // String literals
            '"' | '\'' => self.scan_string(c),
            '«' => self.scan_string('«'),

            // Operators
            '|' => {
                if self.match_char('|') {
                    self.make_token(TokenKind::Or)
                } else {
                    self.make_token(TokenKind::Pipe)
                }
            }

            '&' => {
                if self.match_char('&') {
                    self.make_token(TokenKind::And)
                } else {
                    self.make_token(TokenKind::Background)
                }
            }

            '>' => {
                if self.match_char('>') {
                    self.make_token(TokenKind::Append)
                } else {
                    self.make_token(TokenKind::RedirectOut)
                }
            }

            '<' => self.make_token(TokenKind::RedirectIn),

            ';' => self.make_token(TokenKind::Semicolon),

            // Arabic semicolon (؛)
            '\u{061B}' => self.make_token(TokenKind::Semicolon),

            // Comments (skip to end of line)
            '#' => {
                self.skip_line();
                self.next_token()
            }

            // Word (command or argument)
            _ => self.scan_word(c),
        }
    }

    /// Scan a word (command name or unquoted argument)
    fn scan_word(&mut self, first: char) -> Token {
        let mut value = String::new();
        value.push(first);

        while !self.is_at_end() {
            let c = self.peek();
            if self.is_word_char(c) {
                value.push(self.advance());
            } else {
                break;
            }
        }

        self.make_token(TokenKind::Word(value))
    }

    /// Scan a quoted string
    fn scan_string(&mut self, opening: char) -> Token {
        // Determine closing quote
        let closing = match opening {
            '«' => '»',  // Arabic quotation marks
            _ => opening,
        };

        let mut value = String::new();

        while !self.is_at_end() && self.peek() != closing {
            if self.peek() == '\n' {
                // Unterminated string at end of line
                return self.make_error("نص غير مكتمل / Unterminated string");
            }

            if self.peek() == '\\' {
                self.advance(); // consume backslash
                if self.is_at_end() {
                    return self.make_error("تسلسل هروب غير مكتمل / Unterminated escape");
                }
                let escaped = self.advance();
                value.push(match escaped {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '\\' => '\\',
                    '"' => '"',
                    '\'' => '\'',
                    '0' => '\0',
                    _ => escaped,
                });
            } else {
                value.push(self.advance());
            }
        }

        if self.is_at_end() {
            return self.make_error("نص غير مكتمل / Unterminated string");
        }

        self.advance(); // consume closing quote
        self.make_token(TokenKind::String(value))
    }

    /// Check if character can be part of a word
    fn is_word_char(&self, c: char) -> bool {
        !matches!(c,
            // Whitespace
            ' ' | '\t' | '\n' | '\r' |
            // Operators
            '|' | '&' | '>' | '<' | ';' |
            // Quotes
            '"' | '\'' | '«' | '»' |
            // Comments
            '#' |
            // Arabic semicolon
            '\u{061B}'
        )
    }

    /// Check if character is Arabic letter
    #[allow(dead_code)]
    fn is_arabic_letter(&self, c: char) -> bool {
        matches!(c,
            '\u{0621}'..='\u{063A}' |  // Arabic letters (alef through za)
            '\u{0641}'..='\u{064A}' |  // Arabic letters (fa through ya)
            '\u{066E}'..='\u{066F}' |  // Arabic letter dotless beh/qaf
            '\u{0671}'..='\u{06D3}' |  // Arabic letters extended
            '\u{06D5}'              |  // Arabic letter ae
            '\u{06E5}'..='\u{06E6}' |  // Arabic small waw/ya
            '\u{06EE}'..='\u{06EF}' |  // Arabic letters dal/ra with inverted v
            '\u{06FA}'..='\u{06FC}' |  // Arabic letters sheen/dad/ghain with dot below
            '\u{06FF}'              |  // Arabic letter heh with inverted v
            '\u{0750}'..='\u{077F}' |  // Arabic Supplement
            '\u{08A0}'..='\u{08FF}' |  // Arabic Extended-A
            '\u{FB50}'..='\u{FDFF}' |  // Arabic Presentation Forms-A
            '\u{FE70}'..='\u{FEFF}'    // Arabic Presentation Forms-B
        )
    }

    /// Check if character is Arabic-Indic digit (٠-٩)
    #[allow(dead_code)]
    fn is_arabic_digit(&self, c: char) -> bool {
        matches!(c, '٠'..='٩')  // U+0660 - U+0669
    }

    /// Skip whitespace (but not newlines - they're significant)
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    /// Skip to end of line (for comments)
    fn skip_line(&mut self) {
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
    }

    /// Check if at end of input
    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }

    /// Peek at current character without consuming
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.position]
        }
    }

    /// Advance and return current character
    fn advance(&mut self) -> char {
        let c = self.source[self.position];
        self.position += 1;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        c
    }

    /// Match and consume expected character
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.position] != expected {
            false
        } else {
            self.position += 1;
            self.column += 1;
            true
        }
    }

    /// Create a token with the current lexeme
    fn make_token(&self, kind: TokenKind) -> Token {
        let lexeme: String = self.source[self.token_start..self.position].iter().collect();
        Token::new(
            kind,
            Span::new(self.token_start, self.position, self.line, self.token_start_column),
            lexeme,
        )
    }

    /// Create an error token
    fn make_error(&self, message: &str) -> Token {
        Token::new(
            TokenKind::Error(message.to_string()),
            Span::new(self.token_start, self.position, self.line, self.token_start_column),
            self.source[self.token_start..self.position].iter().collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_command() {
        let mut lexer = Lexer::new("اطبع مرحبا");
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 3); // Word, Word, Eof
        assert!(matches!(&tokens[0].kind, TokenKind::Word(s) if s == "اطبع"));
        assert!(matches!(&tokens[1].kind, TokenKind::Word(s) if s == "مرحبا"));
        assert!(matches!(tokens[2].kind, TokenKind::Eof));
    }

    #[test]
    fn test_quoted_string() {
        let mut lexer = Lexer::new("اطبع \"مرحبا بالعالم\"");
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 3);
        assert!(matches!(&tokens[0].kind, TokenKind::Word(s) if s == "اطبع"));
        assert!(matches!(&tokens[1].kind, TokenKind::String(s) if s == "مرحبا بالعالم"));
    }

    #[test]
    fn test_arabic_quotes() {
        let mut lexer = Lexer::new("اطبع «نص عربي»");
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 3);
        assert!(matches!(&tokens[1].kind, TokenKind::String(s) if s == "نص عربي"));
    }

    #[test]
    fn test_pipe_operator() {
        let mut lexer = Lexer::new("اقرأ ملف | ابحث كلمة");
        let tokens = lexer.tokenize();

        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Pipe)));
    }

    #[test]
    fn test_redirect_operators() {
        let mut lexer = Lexer::new("اطبع نص > ملف");
        let tokens = lexer.tokenize();

        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::RedirectOut)));
    }

    #[test]
    fn test_append_operator() {
        let mut lexer = Lexer::new("اطبع نص >> ملف");
        let tokens = lexer.tokenize();

        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Append)));
    }

    #[test]
    fn test_and_operator() {
        let mut lexer = Lexer::new("انشئ مجلد && انتقل مجلد");
        let tokens = lexer.tokenize();

        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::And)));
    }

    #[test]
    fn test_or_operator() {
        let mut lexer = Lexer::new("اقرأ ملف || اطبع خطأ");
        let tokens = lexer.tokenize();

        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Or)));
    }

    #[test]
    fn test_background_operator() {
        let mut lexer = Lexer::new("sleep 10 &");
        let tokens = lexer.tokenize();

        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Background)));
    }

    #[test]
    fn test_semicolon() {
        let mut lexer = Lexer::new("اطبع أ ; اطبع ب");
        let tokens = lexer.tokenize();

        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Semicolon)));
    }

    #[test]
    fn test_escape_sequences() {
        let mut lexer = Lexer::new(r#"اطبع "سطر1\nسطر2""#);
        let tokens = lexer.tokenize();

        assert!(matches!(&tokens[1].kind, TokenKind::String(s) if s == "سطر1\nسطر2"));
    }

    #[test]
    fn test_comment() {
        let mut lexer = Lexer::new("اطبع مرحبا # هذا تعليق");
        let tokens = lexer.tokenize();

        // Should only have: Word, Word, Eof (comment skipped)
        assert_eq!(tokens.len(), 3);
    }

    #[test]
    fn test_mixed_arabic_english() {
        let mut lexer = Lexer::new("ls -la | grep test");
        let tokens = lexer.tokenize();

        assert!(matches!(&tokens[0].kind, TokenKind::Word(s) if s == "ls"));
        assert!(matches!(&tokens[1].kind, TokenKind::Word(s) if s == "-la"));
        assert!(matches!(tokens[2].kind, TokenKind::Pipe));
    }
}
