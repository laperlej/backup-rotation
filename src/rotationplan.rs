use std::collections::VecDeque;
use std::fmt::Debug;
use std::cmp::Eq;

use crate::dated::Dated;


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RotationPlan<T: Clone + Dated> {
    pub daily: VecDeque<T>,
    pub weekly: VecDeque<T>,
    pub monthly: VecDeque<T>,
}

impl<T: Clone + Dated> RotationPlan<T> {
    pub fn new() -> RotationPlan<T> {
        RotationPlan {
            daily: VecDeque::new(),
            weekly: VecDeque::new(),
            monthly: VecDeque::new(),
        }
    }

    pub fn as_vec(&self) -> Vec<T> {
        let mut v = Vec::new();
        for backup in &self.daily {
            v.push(backup.clone());
        }
        for backup in &self.weekly {
            v.push(backup.clone());
        }
        for backup in &self.monthly {
            v.push(backup.clone());
        }
        v
    }
}

#[cfg(test)]
mod test {
    use crate::file::utils::test_file;
    use crate::rotationplan::RotationPlan;
    use crate::file::File;
    use crate::file::utils::to_files;


    #[test]
    fn backup_eq() {
        let backup1 = RotationPlan::<File> {
            daily: to_files(vec![1, 2, 3, 4, 5, 6, 7]),
            weekly: to_files(vec![1, 8, 15]),
            monthly: to_files(vec![1]),
        };
        let backup2 = RotationPlan::<File> {
            daily: to_files(vec![1, 2, 3, 4, 5, 6, 7]),
            weekly: to_files(vec![1, 8, 15]),
            monthly: to_files(vec![1])
        };
        assert_eq!(backup1, backup2);
    }

    #[test]
    fn backup_ne() {
        let backup1 = RotationPlan::<File> {
            daily: to_files(vec![1, 3, 3, 4, 5, 6, 7]),
            weekly: to_files(vec![1, 8, 15]),
            monthly: to_files(vec![1]),
        };
        let backup2 = RotationPlan::<File> {
            daily: to_files(vec![1, 2, 3, 4, 5, 6, 7]),
            weekly: to_files(vec![1, 8, 15]),
            monthly: to_files(vec![1]),
        };
        assert_ne!(backup1, backup2);
    }

    #[test]
    fn test_rotation_plan() {
        let daily = vec![2, 3, 4, 5, 6, 7];
        let weekly = vec![8, 15];
        let monthly = vec![1];
        let plan = RotationPlan {
            daily: daily.into_iter().map(|x| test_file("", x)).collect(),
            weekly: weekly.into_iter().map(|x| test_file("", x)).collect(),
            monthly: monthly.into_iter().map(|x| test_file("", x)).collect(),
        };
        let v = plan.as_vec();
        assert_eq!(v.len(), 9, "Expected 9 backups, got {}", v.len());
    }
}
