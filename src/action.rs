use crate::file::File;

#[derive(Debug)]
pub struct Action {
    pub type_: ActionType,
    pub file: File,
}

impl Action {
    pub fn new(type_: ActionType, file: File) -> Action {
        Action {
            type_,
            file,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ActionType {
    Remove,
}
