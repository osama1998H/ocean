//! # Parser Module (وحدة التحليل النحوي)
//!
//! Parses tokens into an Abstract Syntax Tree (AST).
//!
//! ## Grammar (simplified)
//!
//! ```text
//! command_line  = sequence
//! sequence      = and_or (';' and_or)*
//! and_or        = pipeline (('&&' | '||') pipeline)*
//! pipeline      = simple_cmd ('|' simple_cmd)*
//! simple_cmd    = word (word | redirect)* ['&']
//! redirect      = ('>' | '>>' | '<') word
//! word          = WORD | STRING
//! ```

pub mod ast;

pub use ast::{Command, Redirect, RedirectKind};

use crate::lexer::{Token, TokenKind};

/// Parser error
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl ParseError {
    pub fn new(message: String, line: usize, column: usize) -> Self {
        Self { message, line, column }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "خطأ نحوي / Parse error [{}:{}]: {}", self.line, self.column, self.message)
    }
}

impl std::error::Error for ParseError {}

/// Result type for parsing
pub type ParseResult<T> = Result<T, ParseError>;

/// Parser for shell commands
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Create a new parser from tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Parse the entire input into a command
    pub fn parse(&mut self) -> ParseResult<Command> {
        self.skip_newlines();

        if self.is_at_end() {
            return Ok(Command::Empty);
        }

        let cmd = self.parse_sequence()?;
        self.skip_newlines();

        if !self.is_at_end() {
            let token = self.peek();
            return Err(ParseError::new(
                format!("رمز غير متوقع / Unexpected token: {}", token.kind),
                token.span.line,
                token.span.column,
            ));
        }

        Ok(cmd)
    }

    /// Parse a sequence of commands (separated by ;)
    fn parse_sequence(&mut self) -> ParseResult<Command> {
        let mut commands = vec![self.parse_and_or()?];

        while self.check(&TokenKind::Semicolon) {
            self.advance();
            self.skip_newlines();
            if self.is_at_end() || self.check(&TokenKind::Eof) {
                break;
            }
            commands.push(self.parse_and_or()?);
        }

        if commands.len() == 1 {
            Ok(commands.pop().unwrap())
        } else {
            Ok(Command::Sequence(commands))
        }
    }

    /// Parse && and || operators
    fn parse_and_or(&mut self) -> ParseResult<Command> {
        let mut left = self.parse_pipeline()?;

        loop {
            if self.check(&TokenKind::And) {
                self.advance();
                self.skip_newlines();
                let right = self.parse_pipeline()?;
                left = Command::And(Box::new(left), Box::new(right));
            } else if self.check(&TokenKind::Or) {
                self.advance();
                self.skip_newlines();
                let right = self.parse_pipeline()?;
                left = Command::Or(Box::new(left), Box::new(right));
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse a pipeline (commands connected by |)
    fn parse_pipeline(&mut self) -> ParseResult<Command> {
        let mut commands = vec![self.parse_simple_command()?];

        while self.check(&TokenKind::Pipe) {
            self.advance();
            self.skip_newlines();
            commands.push(self.parse_simple_command()?);
        }

        if commands.len() == 1 {
            Ok(commands.pop().unwrap())
        } else {
            Ok(Command::Pipeline(commands))
        }
    }

    /// Parse a simple command with arguments and redirections
    fn parse_simple_command(&mut self) -> ParseResult<Command> {
        let name = self.expect_word()?;
        let mut args = Vec::new();
        let mut redirects = Vec::new();

        loop {
            if self.check_redirect() {
                redirects.push(self.parse_redirect()?);
            } else if let Some(word) = self.try_word() {
                args.push(word);
            } else {
                break;
            }
        }

        let mut cmd = Command::Simple { name, args, redirects };

        // Check for background operator
        if self.check(&TokenKind::Background) {
            self.advance();
            cmd = Command::Background(Box::new(cmd));
        }

        Ok(cmd)
    }

    /// Parse a redirection
    fn parse_redirect(&mut self) -> ParseResult<Redirect> {
        let kind = match &self.peek().kind {
            TokenKind::RedirectOut => RedirectKind::Out,
            TokenKind::RedirectIn => RedirectKind::In,
            TokenKind::Append => RedirectKind::Append,
            _ => {
                let token = self.peek();
                return Err(ParseError::new(
                    "متوقع عامل إعادة توجيه / Expected redirect operator".to_string(),
                    token.span.line,
                    token.span.column,
                ));
            }
        };

        self.advance();
        let target = self.expect_word()?;

        Ok(Redirect::new(kind, target))
    }

    /// Check if current token is a redirect operator
    fn check_redirect(&self) -> bool {
        matches!(
            self.peek().kind,
            TokenKind::RedirectOut | TokenKind::RedirectIn | TokenKind::Append
        )
    }

    /// Expect and consume a word token
    fn expect_word(&mut self) -> ParseResult<String> {
        let token = self.peek().clone();
        match &token.kind {
            TokenKind::Word(s) => {
                self.advance();
                Ok(s.clone())
            }
            TokenKind::String(s) => {
                self.advance();
                Ok(s.clone())
            }
            _ => Err(ParseError::new(
                format!("متوقع كلمة / Expected word, got: {}", token.kind),
                token.span.line,
                token.span.column,
            )),
        }
    }

    /// Try to consume a word token (returns None if not a word)
    fn try_word(&mut self) -> Option<String> {
        match &self.peek().kind {
            TokenKind::Word(s) => {
                let s = s.clone();
                self.advance();
                Some(s)
            }
            TokenKind::String(s) => {
                let s = s.clone();
                self.advance();
                Some(s)
            }
            _ => None,
        }
    }

    /// Skip newline tokens
    fn skip_newlines(&mut self) {
        while self.check(&TokenKind::Newline) {
            self.advance();
        }
    }

    /// Check if current token matches expected kind
    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }

    /// Peek at current token
    fn peek(&self) -> &Token {
        &self.tokens[self.position.min(self.tokens.len() - 1)]
    }

    /// Advance to next token
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.position += 1;
        }
        self.previous()
    }

    /// Get previous token
    fn previous(&self) -> &Token {
        &self.tokens[self.position.saturating_sub(1)]
    }

    /// Check if at end of tokens
    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(input: &str) -> ParseResult<Command> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_simple_command() {
        let cmd = parse("اطبع مرحبا").unwrap();
        match cmd {
            Command::Simple { name, args, .. } => {
                assert_eq!(name, "اطبع");
                assert_eq!(args, vec!["مرحبا"]);
            }
            _ => panic!("Expected simple command"),
        }
    }

    #[test]
    fn test_command_with_multiple_args() {
        let cmd = parse("انسخ ملف1 ملف2").unwrap();
        match cmd {
            Command::Simple { name, args, .. } => {
                assert_eq!(name, "انسخ");
                assert_eq!(args, vec!["ملف1", "ملف2"]);
            }
            _ => panic!("Expected simple command"),
        }
    }

    #[test]
    fn test_pipeline() {
        let cmd = parse("اقرأ ملف | ابحث نص").unwrap();
        match cmd {
            Command::Pipeline(cmds) => {
                assert_eq!(cmds.len(), 2);
            }
            _ => panic!("Expected pipeline"),
        }
    }

    #[test]
    fn test_redirect_out() {
        let cmd = parse("اطبع نص > output.txt").unwrap();
        match cmd {
            Command::Simple { redirects, .. } => {
                assert_eq!(redirects.len(), 1);
                assert_eq!(redirects[0].kind, RedirectKind::Out);
                assert_eq!(redirects[0].target, "output.txt");
            }
            _ => panic!("Expected simple command with redirect"),
        }
    }

    #[test]
    fn test_redirect_in() {
        let cmd = parse("اقرأ < input.txt").unwrap();
        match cmd {
            Command::Simple { redirects, .. } => {
                assert_eq!(redirects.len(), 1);
                assert_eq!(redirects[0].kind, RedirectKind::In);
            }
            _ => panic!("Expected simple command with redirect"),
        }
    }

    #[test]
    fn test_append() {
        let cmd = parse("اطبع نص >> log.txt").unwrap();
        match cmd {
            Command::Simple { redirects, .. } => {
                assert_eq!(redirects.len(), 1);
                assert_eq!(redirects[0].kind, RedirectKind::Append);
            }
            _ => panic!("Expected simple command with append"),
        }
    }

    #[test]
    fn test_and_operator() {
        let cmd = parse("انشئ مجلد && انتقل مجلد").unwrap();
        assert!(matches!(cmd, Command::And(_, _)));
    }

    #[test]
    fn test_or_operator() {
        let cmd = parse("اقرأ ملف || اطبع خطأ").unwrap();
        assert!(matches!(cmd, Command::Or(_, _)));
    }

    #[test]
    fn test_sequence() {
        let cmd = parse("اطبع أ ; اطبع ب").unwrap();
        match cmd {
            Command::Sequence(cmds) => {
                assert_eq!(cmds.len(), 2);
            }
            _ => panic!("Expected sequence"),
        }
    }

    #[test]
    fn test_background() {
        let cmd = parse("sleep 10 &").unwrap();
        assert!(matches!(cmd, Command::Background(_)));
    }

    #[test]
    fn test_quoted_args() {
        let cmd = parse(r#"اطبع "مرحبا بالعالم""#).unwrap();
        match cmd {
            Command::Simple { args, .. } => {
                assert_eq!(args, vec!["مرحبا بالعالم"]);
            }
            _ => panic!("Expected simple command"),
        }
    }

    #[test]
    fn test_complex_pipeline() {
        let cmd = parse("اقرأ ملف | ابحث كلمة | اعرض > output.txt").unwrap();
        match cmd {
            Command::Pipeline(cmds) => {
                assert_eq!(cmds.len(), 3);
                // Last command should have a redirect
                if let Command::Simple { redirects, .. } = &cmds[2] {
                    assert_eq!(redirects.len(), 1);
                }
            }
            _ => panic!("Expected pipeline"),
        }
    }

    #[test]
    fn test_empty_input() {
        let cmd = parse("").unwrap();
        assert!(matches!(cmd, Command::Empty));
    }
}
