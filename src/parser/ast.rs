//! # Abstract Syntax Tree (شجرة بناء الجملة المجردة)
//!
//! Defines the AST types for shell command parsing.

use std::fmt;

/// A shell command or pipeline
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Simple command: name with arguments and optional redirections
    /// Example: `اطبع مرحبا > output.txt`
    Simple {
        name: String,
        args: Vec<String>,
        redirects: Vec<Redirect>,
    },

    /// Pipeline: chain of commands connected by pipes
    /// Example: `اقرأ ملف | ابحث كلمة | اعرض`
    Pipeline(Vec<Command>),

    /// Logical AND: run second only if first succeeds
    /// Example: `انشئ مجلد && انتقل مجلد`
    And(Box<Command>, Box<Command>),

    /// Logical OR: run second only if first fails
    /// Example: `اقرأ ملف || اطبع "ملف غير موجود"`
    Or(Box<Command>, Box<Command>),

    /// Sequence: run commands in order
    /// Example: `اطبع أ ; اطبع ب ; اطبع ج`
    Sequence(Vec<Command>),

    /// Background execution
    /// Example: `sleep 10 &`
    Background(Box<Command>),

    /// Empty command (for blank lines)
    Empty,
}

/// I/O Redirection
#[derive(Debug, Clone, PartialEq)]
pub struct Redirect {
    pub kind: RedirectKind,
    pub target: String,
}

impl Redirect {
    pub fn new(kind: RedirectKind, target: String) -> Self {
        Self { kind, target }
    }
}

/// Types of I/O redirection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectKind {
    /// Output redirection: > (إلى)
    Out,
    /// Input redirection: < (من)
    In,
    /// Append output: >> (الحق)
    Append,
}

impl fmt::Display for RedirectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RedirectKind::Out => write!(f, ">"),
            RedirectKind::In => write!(f, "<"),
            RedirectKind::Append => write!(f, ">>"),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Simple { name, args, redirects } => {
                write!(f, "{}", name)?;
                for arg in args {
                    write!(f, " {}", arg)?;
                }
                for redir in redirects {
                    write!(f, " {} {}", redir.kind, redir.target)?;
                }
                Ok(())
            }
            Command::Pipeline(cmds) => {
                let strs: Vec<String> = cmds.iter().map(|c| c.to_string()).collect();
                write!(f, "{}", strs.join(" | "))
            }
            Command::And(left, right) => {
                write!(f, "{} && {}", left, right)
            }
            Command::Or(left, right) => {
                write!(f, "{} || {}", left, right)
            }
            Command::Sequence(cmds) => {
                let strs: Vec<String> = cmds.iter().map(|c| c.to_string()).collect();
                write!(f, "{}", strs.join(" ; "))
            }
            Command::Background(cmd) => {
                write!(f, "{} &", cmd)
            }
            Command::Empty => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_command_display() {
        let cmd = Command::Simple {
            name: "اطبع".to_string(),
            args: vec!["مرحبا".to_string()],
            redirects: vec![],
        };
        assert_eq!(cmd.to_string(), "اطبع مرحبا");
    }

    #[test]
    fn test_redirect_display() {
        let cmd = Command::Simple {
            name: "اطبع".to_string(),
            args: vec!["نص".to_string()],
            redirects: vec![Redirect::new(RedirectKind::Out, "output.txt".to_string())],
        };
        assert_eq!(cmd.to_string(), "اطبع نص > output.txt");
    }

    #[test]
    fn test_pipeline_display() {
        let cmd = Command::Pipeline(vec![
            Command::Simple {
                name: "اقرأ".to_string(),
                args: vec!["ملف".to_string()],
                redirects: vec![],
            },
            Command::Simple {
                name: "ابحث".to_string(),
                args: vec!["كلمة".to_string()],
                redirects: vec![],
            },
        ]);
        assert_eq!(cmd.to_string(), "اقرأ ملف | ابحث كلمة");
    }
}
