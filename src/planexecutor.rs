use std::fs::remove_file;

use crate::action::ActionType;
use crate::planner::Plan;
use crate::action::Action;
use crate::file::File;

pub struct PlanExecutor<'a> {
    executor: Box<dyn Executor + 'a>,
}

impl<'a> PlanExecutor<'a> {
    pub fn new(executor: impl Executor + 'a) -> Self {
        PlanExecutor {
            executor: Box::new(executor),
        }
    }

    pub fn execute(&self, plan: Plan){
        for filemod in plan {
            self.executor.execute(filemod);
        }
    }
}

pub trait Executor {
    fn execute(&self, action: Action);
}

pub struct FsExecutor {}

impl FsExecutor {
    pub fn new() -> Self {
        FsExecutor {}
    }
}

impl Executor for FsExecutor {
    fn execute(&self, action: Action){
        match action.type_ {
            ActionType::Remove => {
                remove_file(action.file.path).unwrap();
            },
        }
    }
}

#[cfg(test)]
mod tests {

use super::*;
struct MockExecutor {
    files: Vec<File>,
}

impl MockExecutor {
    pub fn new(files: Vec<File>) -> Self {
        MockExecutor {
            files,
        }
    }

    pub fn result(&self) -> Vec<File> {
        self.files.clone()
    }

}

impl Executor for MockExecutor {
    fn execute(&self, action: Action){
        match action.type_ {
            ActionType::Remove => {
                println!("Would remove file: {}", action.file.path);
            },
        }
    }
}

}
