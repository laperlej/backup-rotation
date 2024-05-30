mod rotationplan;
mod rotator;
mod dated;
mod file;
mod action;
mod execute;
mod plan;
mod args;

use chrono::Utc;
use file::File;
use plan::create_plan;
use execute::execute_plan;
use args::Args;
use clap::Parser;

fn files_after_rotation(files_before: &Vec<File>) -> Vec<File> {
    let mut rotator = rotator::Rotator::<File>::new(3, 2, 1);
    for backup in files_before {
        rotator.add_backup(backup.clone());
    }
    rotator.get_backups().as_vec()
}


fn apply_rotation(before: &Vec<File>, after: &Vec<File>) {
    let plan = create_plan(before, after);
    execute_plan(plan);
}

fn get_backups() -> Vec<File> {
    let mut backups = vec![];
    for i in 0..10 {
        let t = Utc::now();
        let t = t + chrono::Duration::days(i);
        let file = File::new("".to_string(), t);
        backups.push(file);
    }
    backups
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args.files);
    println!("{}", args.format);
    let backups = get_backups();
    let backups_after_rotation = files_after_rotation(&backups);
    apply_rotation(&backups, &backups_after_rotation);
}

#[cfg(test)]
mod test {
    use super::*;
    use file::utils::to_files;

    #[test]
    fn get_backups_test() {
        let backups = get_backups();
        assert_eq!(backups.len(), 10);
    }

    #[test]
    fn test_rotation() {
        let tmp = to_files(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let mut backups = vec![];
        for b in tmp {
            backups.push(b);
        }
        let backups_after_rotation = files_after_rotation(&backups);
        apply_rotation(&backups, &backups_after_rotation);
        assert_eq!(backups.len(), 10);
        assert_eq!(backups_after_rotation.len(), 5);
    }
}

