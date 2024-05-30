use std::collections::VecDeque;
use std::fmt::Debug;

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


