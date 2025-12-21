//! # Pipeline Executor (منفذ الأنابيب)
//!
//! Handles execution of command pipelines.

use crate::parser::Command;
use super::{CommandResult, Executor};

/// Pipeline executor for chained commands
#[allow(dead_code)]
pub struct PipelineExecutor<'a> {
    executor: &'a mut Executor,
}

#[allow(dead_code)]
impl<'a> PipelineExecutor<'a> {
    /// Create a new pipeline executor
    pub fn new(executor: &'a mut Executor) -> Self {
        Self { executor }
    }

    /// Execute a pipeline of commands
    pub fn execute(&mut self, commands: Vec<Command>) -> CommandResult {
        self.executor.execute(Command::Pipeline(commands))
    }
}
