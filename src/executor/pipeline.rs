//! # Pipeline Executor (منفذ الأنابيب)
//!
//! Handles execution of command pipelines.

use crate::parser::Command;
use super::{CommandResult, Executor};

#[allow(dead_code)]
pub struct PipelineExecutor<'a> {
    executor: &'a mut Executor,
}

#[allow(dead_code)]
impl<'a> PipelineExecutor<'a> {
    pub fn new(executor: &'a mut Executor) -> Self {
        Self { executor }
    }

    pub fn execute(&mut self, commands: Vec<Command>) -> CommandResult {
        self.executor.execute(Command::Pipeline(commands))
    }
}
