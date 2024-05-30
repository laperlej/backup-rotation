mod rotationplan;
mod rotator;
mod dated;
mod file;
mod action;
mod execute;
mod plan;
mod args;

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

fn get_backups(args: Args) -> Vec<File> {
    match args.format {
        Some(format) => {
            let mut files = vec![];
            for file in &args.files {
                let file = File::from_path(file.clone(), &format);
                files.push(file.unwrap());
            }
            files
        }
        None => {
            let mut files = vec![];
            for file in &args.files {
                let file = File::from_fs(file.clone());
                files.push(file.unwrap());
            }
            files
        }
    }
}

fn main() {
    let args = Args::parse();
    
    if args.files.is_empty() {
        println!("No files to rotate");
        return;
    }


    let backups = get_backups(args);
    let backups_after_rotation = files_after_rotation(&backups);
    apply_rotation(&backups, &backups_after_rotation);
}

#[cfg(test)]
mod test {
    use super::*;
    use file::utils::to_files;
    use std::fs;

    #[test]
    fn get_fs_backups_test() {
        let tmp_dir = tempdir::TempDir::new("example").unwrap();

        let args = Args {
            files: vec![
                tmp_dir.path().join("a").to_str().unwrap().to_string(),
                tmp_dir.path().join("b").to_str().unwrap().to_string(),
                tmp_dir.path().join("c").to_str().unwrap().to_string(),
            ],
            format: None
        };
        for file in &args.files {
            let f = fs::File::create(file).unwrap();
            drop(f);
        }
        let backups = get_backups(args);
        assert_eq!(backups.len(), 3);
    }

    #[test]
    fn get_path_backups_test() {
        let args = Args {
            files: vec![
                "pg_2024-02-17_03-00-01.tar".to_string(),
                "pg_2024-02-29_03-00-01.tar".to_string(),
                "pg_2024-03-12_03-00-01.tar".to_string(),
            ],
            format: Some("pg_%Y-%m-%d_%H-%M-%S.tar".to_string())
        };
        let backups = get_backups(args);
        assert_eq!(backups.len(), 3);
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

