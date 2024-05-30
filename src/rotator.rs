use crate::dated::Dated;
use crate::rotationplan::RotationPlan;
use chrono::{DateTime, Utc, Datelike};

#[derive(Debug)]
pub struct BackupRotator<T: Clone + Dated> {
    daily_max: usize,
    weekly_max: usize,
    monthly_max: usize,
    backups: RotationPlan<T>,
}

impl<T: Clone + Dated> BackupRotator<T> {
    pub fn new(daily_max: usize, weekly_max: usize, monthly_max: usize) -> BackupRotator<T> {
        BackupRotator {
            daily_max,
            weekly_max,
            monthly_max,
            backups: RotationPlan::new(),
        }
    }

    fn add_monthly(&mut self, backup: T) {
        self.backups.monthly.push_back(backup);
        while self.backups.monthly.len() > self.monthly_max {
            self.backups.monthly.pop_front();
        }
    }

    fn add_weekly(&mut self, backup: T) {
        self.backups.weekly.push_back(backup);
        while self.backups.weekly.len() > self.weekly_max {
            self.backups.weekly.pop_front();
        }
    }

    fn add_daily(&mut self, backup: T) {
        self.backups.daily.push_back(backup);
        while self.backups.daily.len() > self.daily_max {
            self.backups.daily.pop_front();
        }
    }

    fn is_new_month(&self, time: DateTime<Utc>) -> bool {
        if self.backups.monthly.is_empty() {
            return true;
        }
        let last_monthly = self.backups.monthly.back().unwrap();
        different_month(time, last_monthly.get_date())
    }

    fn is_new_week(&self, time: DateTime<Utc>) -> bool {
        let last_monthly = self.backups.monthly.back().unwrap();
        if self.backups.weekly.is_empty() && different_week(time, last_monthly.get_date()) {
            return true;
        }
        if self.backups.weekly.is_empty() {
            return false;
        }
        let last_weekly = self.backups.weekly.back().unwrap();
        different_week(time, last_weekly.get_date())
    }

    fn is_new_day(&self, time: DateTime<Utc>) -> bool {
        if self.backups.daily.is_empty() {
            return true;
        }
        let last_daily = self.backups.daily.back().unwrap();
        different_day(time, last_daily.get_date())
    }

    pub fn add_backup(&mut self, backup: T) {
        if self.is_new_month(backup.get_date()) {
            self.add_monthly(backup);
        } else if  self.is_new_week(backup.get_date()) {
            self.add_weekly(backup);
        } else if self.is_new_day(backup.get_date()) {
            self.add_daily(backup);
        }
    }

    pub fn get_backups(&self) -> RotationPlan<T> {
        self.backups.clone()
    }
}

fn different_month(dt1: DateTime<Utc>, dt2: DateTime<Utc>) -> bool {
    dt1.month() != dt2.month()
}

fn different_week(dt1: DateTime<Utc>, dt2: DateTime<Utc>) -> bool {
    dt1 - dt2 > chrono::Duration::weeks(1) - chrono::Duration::days(1)
}

fn different_day(dt1: DateTime<Utc>, dt2: DateTime<Utc>) -> bool {
    dt1.day() != dt2.day()
}
