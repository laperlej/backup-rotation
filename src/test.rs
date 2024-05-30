#[cfg(test)]
mod tests {

use crate::rotator::BackupRotator;
use crate::rotationplan::RotationPlan;
use std::collections::VecDeque;
use std::fmt::Debug;
use chrono::prelude::*;
use chrono::{DateTime, Utc};

use crate::dated::Dated;

#[derive(Eq, PartialEq, Clone)]
pub struct Time {
    time: DateTime<Utc>,
}

impl Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = Utc.with_ymd_and_hms(2024, 1, 1, 3, 0, 0).unwrap();
        let days = self.time - t;
        write!(f, "{}", days.num_days() + 1)
    }
}

impl Dated for Time {
    fn get_date(&self) -> DateTime<Utc> {
        self.time
    }
}

impl Time {
    pub fn new(days: u32) -> Time {
        let t = Utc.with_ymd_and_hms(2024, 1, 1, 3, 0, 0).unwrap();
        let extra_days = chrono::Duration::days(days as i64 - 1);
        let t = t + extra_days;
        Time {
            time: t,
        }
    }
}

fn to_time(v: Vec<usize>) -> VecDeque<Time> {
    v.into_iter().map(|x| Time::new(x as u32)).collect()
}

#[test]
fn backup_eq() {
    let backup1 = RotationPlan::<Time> {
        daily: to_time(vec![1, 2, 3, 4, 5, 6, 7]),
        weekly: to_time(vec![1, 8, 15]),
        monthly: to_time(vec![1]),
    };
    let backup2 = RotationPlan::<Time> {
        daily: to_time(vec![1, 2, 3, 4, 5, 6, 7]),
        weekly: to_time(vec![1, 8, 15]),
        monthly: to_time(vec![1]),
    };
    assert_eq!(backup1, backup2);
}

#[test]
fn backup_ne() {
    let backup1 = RotationPlan::<Time> {
        daily: to_time(vec![1, 3, 3, 4, 5, 6, 7]),
        weekly: to_time(vec![1, 8, 15]),
        monthly: to_time(vec![1]),
    };
    let backup2 = RotationPlan::<Time> {
        daily: to_time(vec![1, 2, 3, 4, 5, 6, 7]),
        weekly: to_time(vec![1, 8, 15]),
        monthly: to_time(vec![1]),
    };
    assert_ne!(backup1, backup2);
}

#[test]
fn backup_rotator() {
    let mut rotator = BackupRotator::<Time>::new(7, 3, 1);
    for backup in 0..34 {
        rotator.add_backup(Time::new(backup));
    }
    let result = rotator.get_backups();
    let expected = RotationPlan::<Time> {
        daily: to_time(vec![25, 26, 27, 28, 30, 31, 33]),
        weekly: to_time(vec![15, 22, 29]),
        monthly: to_time(vec![32]),
    };
    assert_eq!(result, expected);
}

}
