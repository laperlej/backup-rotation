use std::fs::remove_file;
use std::path::Path;

use crate::action::ActionType;
use crate::plan::Plan;
use crate::action::Action;

pub fn execute_plan(plan: Plan) {
    for action in plan {
        println!("Executing action: {:?}", action);
        execute_action(&action);
    }
}

fn execute_action(action: &Action){
    match action.type_ {
        ActionType::Remove => {
            if Path::new(&action.file.path).exists() {
                println!("Removing file: {}", action.file.path);
                remove_file(action.file.path.clone()).unwrap();
            }
        },
    }
}

#[cfg(test)]
mod tests {

use super::*;
use crate::file::utils::test_file;
use tempdir::TempDir;
use std::fs::File;

fn pathbuf_to_str(pb: std::path::PathBuf) -> String {
    pb.into_os_string().into_string().unwrap()
}

#[test]
fn test_execute_action() {
    let tmp_dir = TempDir::new("example").unwrap();
    let tmp_path = tmp_dir.path().join("a");
    let file = test_file(pathbuf_to_str(tmp_path.clone()).as_str(), 1);
    let f = File::create(tmp_path.clone()).unwrap();
    drop(f);
    assert!(tmp_path.exists());
    let action = Action::new(ActionType::Remove, file.clone());
    execute_action(&action);

    assert!(!tmp_path.exists());
}

#[test]
fn test_execute_plan() {
    let tmp_dir = TempDir::new("example").unwrap();
    let files = [
        test_file(pathbuf_to_str(tmp_dir.path().join("a").clone()).as_str(), 1),
        test_file(pathbuf_to_str(tmp_dir.path().join("b").clone()).as_str(), 2),
        test_file(pathbuf_to_str(tmp_dir.path().join("c").clone()).as_str(), 3),
        test_file(pathbuf_to_str(tmp_dir.path().join("d").clone()).as_str(), 4),
    ];
    for item in files.iter() {
        let f = File::create(item.path.clone()).unwrap();
        drop(f);
    }
    let plan = vec![
        Action::new(ActionType::Remove, files[0].clone()),
        Action::new(ActionType::Remove, files[3].clone()),
    ];
    files.iter().for_each(|f| {
        assert!(Path::new(&f.path).exists());
    });

    execute_plan(plan);

    assert!(!tmp_dir.path().join("a").exists());
    assert!(tmp_dir.path().join("b").exists());
    assert!(tmp_dir.path().join("c").exists());
    assert!(!tmp_dir.path().join("d").exists());
}

}
