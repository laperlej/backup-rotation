use crate::file::File;
use std::collections::HashSet;
use crate::action::{ActionType, Action};
use std::iter::Iterator;

pub struct Planner {
    before: Vec<File>,
    after: Vec<File>,
}

impl Planner {
    pub fn new(before: Vec<File>, after: Vec<File>) -> Planner {
        Planner {
            before,
            after,
        }
    }

    pub fn plan(&self) -> Plan {
        let mut plan = Plan::new();
        let mut seen = HashSet::new();
        for file in self.before.as_slice() {
            seen.insert(file);
        }
        for file in self.after.as_slice() {
            if !seen.contains(&file) {
                plan.push(Action {
                    type_: ActionType::Remove,
                    file: file.clone(),
                });
            }
        }
        plan
    }
}

pub struct Plan {
    vec: Vec<Action>,
}

impl Plan {
    pub fn new() -> Plan {
        Plan {
            vec: vec![],
        }
    }
    
    pub fn push(&mut self, filemod: Action) {
        self.vec.push(filemod);
    }
}

impl Iterator for Plan {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        self.vec.pop()
    }
}

