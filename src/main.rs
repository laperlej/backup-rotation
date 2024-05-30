mod rotationplan;
mod rotator;
mod test;
mod dated;
mod file;
mod action;
mod planexecutor;
mod planner;

use chrono::Utc;
use file::File;

fn files_after_rotation(files_before: &Vec<File>) -> Vec<File> {
    let mut rotator = rotator::BackupRotator::<File>::new(3, 2, 1);
    for backup in files_before {
        rotator.add_backup(backup.clone());
    }
    rotator.get_backups().as_vec()
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

fn apply_rotation(before: Vec<File>, after: Vec<File>) {
    let planner = planner::Planner::new(before, after);
    let plan = planner.plan();
    let executor = planexecutor::FsExecutor::new();
    let plan_executor = planexecutor::PlanExecutor::new(executor);
    plan_executor.execute(plan);
}

fn main() {
    let backups = get_backups();
    let backups_after_rotation = files_after_rotation(&backups);
    apply_rotation(backups, backups_after_rotation);
}
