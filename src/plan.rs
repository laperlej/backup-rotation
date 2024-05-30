use crate::file::File;
use std::collections::HashSet;
use crate::action::{ActionType, Action};

pub type Plan = Vec<Action>;

pub fn create_plan(before: &Vec<File>, after: &Vec<File>) -> Plan {
        let mut plan = Plan::new();
        let mut seen = HashSet::new();
        for file in after {
            seen.insert(file.path.clone());
        }
        for file in before {
            if !seen.contains(&file.path) {
                plan.push(Action {
                    type_: ActionType::Remove,
                    file: file.clone(),
                });
            }
        }
        plan
}


#[cfg(test)]
mod test {

use super::*;
use crate::file::utils::test_file;
use crate::action::ActionType;

#[test]
fn test_plan() {
    let before = vec![
        test_file("a", 1),
        test_file("b", 2),
        test_file("c", 3),
        test_file("d", 4),
    ];
    let after = vec![
        test_file("b", 2),
        test_file("c", 3),
    ];
    let plan = create_plan(&before, &after);
    assert_eq!(plan.len(), 2, "Expected 2 actions, got {}", plan.len());
    assert_eq!(plan[0].type_, ActionType::Remove);
    assert_eq!(plan[0].file.path, "a");
    assert_eq!(plan[1].type_, ActionType::Remove);
    assert_eq!(plan[1].file.path, "d");
}

}

