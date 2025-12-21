//! # Executor Module (وحدة التنفيذ)
//!
//! Executes parsed commands with support for:
//! - Pipelines
//! - I/O redirection
//! - Command chaining (&&, ||, ;)
//! - Background execution

mod pipeline;

use crate::parser::{Command, Redirect, RedirectKind};
use crate::commands;

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process::{Command as ProcessCommand, Stdio};

/// Result of command execution
#[derive(Debug, Clone, PartialEq)]
pub enum CommandResult {
    /// Command succeeded with optional output
    Success(String),
    /// Command failed with error message
    Error(String),
    /// Shell should exit with code
    Exit(i32),
    /// No output (for commands that print directly)
    None,
}

impl CommandResult {
    /// Check if result indicates success
    pub fn is_success(&self) -> bool {
        matches!(self, CommandResult::Success(_) | CommandResult::None)
    }

    /// Check if result indicates exit
    pub fn is_exit(&self) -> bool {
        matches!(self, CommandResult::Exit(_))
    }

    /// Get output if available
    #[allow(dead_code)]
    pub fn output(&self) -> Option<&str> {
        match self {
            CommandResult::Success(s) => Some(s),
            _ => None,
        }
    }
}

/// Main executor for shell commands
pub struct Executor {
    /// Last exit code
    pub last_exit_code: i32,
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

impl Executor {
    /// Create a new executor
    pub fn new() -> Self {
        Self {
            last_exit_code: 0,
        }
    }

    /// Execute a command and return the result
    pub fn execute(&mut self, cmd: Command) -> CommandResult {
        self.execute_with_input(cmd, None)
    }

    /// Execute a command with optional stdin input
    pub fn execute_with_input(&mut self, cmd: Command, input: Option<String>) -> CommandResult {
        match cmd {
            Command::Empty => CommandResult::None,

            Command::Simple { name, args, redirects } => {
                self.execute_simple(&name, &args, &redirects, input)
            }

            Command::Pipeline(cmds) => {
                self.execute_pipeline(cmds)
            }

            Command::And(left, right) => {
                let result = self.execute(*left);
                if result.is_success() {
                    self.execute(*right)
                } else {
                    result
                }
            }

            Command::Or(left, right) => {
                let result = self.execute(*left);
                if !result.is_success() {
                    self.execute(*right)
                } else {
                    result
                }
            }

            Command::Sequence(cmds) => {
                let mut last_result = CommandResult::None;
                for cmd in cmds {
                    last_result = self.execute(cmd);
                    if last_result.is_exit() {
                        return last_result;
                    }
                }
                last_result
            }

            Command::Background(cmd) => {
                // For now, just execute normally
                // TODO: Implement proper background execution
                eprintln!("تحذير: التنفيذ في الخلفية غير مدعوم حالياً / Warning: Background execution not yet supported");
                self.execute(*cmd)
            }
        }
    }

    /// Execute a simple command
    fn execute_simple(
        &mut self,
        name: &str,
        args: &[String],
        redirects: &[Redirect],
        input: Option<String>,
    ) -> CommandResult {
        // Handle redirections
        let stdin_redirect = redirects.iter().find(|r| r.kind == RedirectKind::In);
        let stdout_redirect = redirects.iter().find(|r| r.kind == RedirectKind::Out || r.kind == RedirectKind::Append);

        // Get input from file if redirected
        let actual_input = if let Some(redir) = stdin_redirect {
            match std::fs::read_to_string(&redir.target) {
                Ok(content) => Some(content),
                Err(e) => {
                    return CommandResult::Error(format!(
                        "خطأ: لا يمكن قراءة '{}' - {} / Error: Cannot read '{}' - {}",
                        redir.target, e, redir.target, e
                    ));
                }
            }
        } else {
            input
        };

        // Execute the command
        let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        let result = self.execute_builtin_or_external(name, &args_str, actual_input);

        // Handle output redirection
        if let Some(redir) = stdout_redirect {
            if let CommandResult::Success(output) = &result {
                let file_result = if redir.kind == RedirectKind::Append {
                    OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&redir.target)
                } else {
                    File::create(&redir.target)
                };

                match file_result {
                    Ok(mut file) => {
                        if let Err(e) = file.write_all(output.as_bytes()) {
                            return CommandResult::Error(format!(
                                "خطأ: لا يمكن الكتابة إلى '{}' - {} / Error: Cannot write to '{}' - {}",
                                redir.target, e, redir.target, e
                            ));
                        }
                        return CommandResult::None;
                    }
                    Err(e) => {
                        return CommandResult::Error(format!(
                            "خطأ: لا يمكن فتح '{}' - {} / Error: Cannot open '{}' - {}",
                            redir.target, e, redir.target, e
                        ));
                    }
                }
            }
        }

        result
    }

    /// Execute a pipeline of commands
    fn execute_pipeline(&mut self, cmds: Vec<Command>) -> CommandResult {
        if cmds.is_empty() {
            return CommandResult::None;
        }

        let mut input: Option<String> = None;

        for cmd in cmds.into_iter() {
            let result = self.execute_with_input(cmd, input.take());

            match result {
                CommandResult::Success(output) => {
                    input = Some(output);
                }
                CommandResult::Error(_) | CommandResult::Exit(_) => {
                    return result;
                }
                CommandResult::None => {
                    // No output to pipe
                }
            }
        }

        // Return the final output
        match input {
            Some(output) => {
                // Print final output
                print!("{}", output);
                CommandResult::None
            }
            None => CommandResult::None,
        }
    }

    /// Execute a builtin command or external program
    fn execute_builtin_or_external(
        &mut self,
        name: &str,
        args: &[&str],
        input: Option<String>,
    ) -> CommandResult {
        // Try builtin command first
        if let Some(result) = commands::execute_builtin(name, args, input.as_deref()) {
            self.last_exit_code = if result.is_success() { 0 } else { 1 };
            return result;
        }

        // Fall back to external command
        self.execute_external(name, args, input)
    }

    /// Execute an external command
    fn execute_external(
        &mut self,
        name: &str,
        args: &[&str],
        input: Option<String>,
    ) -> CommandResult {
        let mut cmd = ProcessCommand::new(name);
        cmd.args(args);

        if input.is_some() {
            cmd.stdin(Stdio::piped());
        }
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        match cmd.spawn() {
            Ok(mut child) => {
                // Write input if provided
                if let Some(input_str) = input {
                    if let Some(ref mut stdin) = child.stdin {
                        let _ = stdin.write_all(input_str.as_bytes());
                    }
                }

                match child.wait_with_output() {
                    Ok(output) => {
                        self.last_exit_code = output.status.code().unwrap_or(1);

                        if output.status.success() {
                            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                            CommandResult::Success(stdout)
                        } else {
                            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                            if !stderr.is_empty() {
                                CommandResult::Error(stderr)
                            } else {
                                CommandResult::Error(format!(
                                    "الأمر انتهى برمز: {} / Command exited with code: {}",
                                    self.last_exit_code, self.last_exit_code
                                ))
                            }
                        }
                    }
                    Err(e) => {
                        self.last_exit_code = 1;
                        CommandResult::Error(format!(
                            "خطأ: فشل في انتظار الأمر - {} / Error: Failed to wait for command - {}",
                            e, e
                        ))
                    }
                }
            }
            Err(e) => {
                self.last_exit_code = 127;
                CommandResult::Error(format!(
                    "خطأ: الأمر '{}' غير موجود - {} / Error: Command '{}' not found - {}",
                    name, e, name, e
                ))
            }
        }
    }
}
