use crate::file::File;

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

pub enum ActionType {
    Remove,
}
