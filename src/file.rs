use chrono::{DateTime, Utc, NaiveDateTime, TimeZone};
use std::fs;
use std::hash::{Hasher, Hash};

use crate::dated::Dated;

#[derive(Debug, Clone)]
pub struct File {
    pub path: String,
    timestamp: DateTime<Utc>,
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Hash for File {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

impl File {
    pub fn new(path: String, timestamp: DateTime<Utc>) -> File {
        File {
            path,
            timestamp,
        }
    }

    pub fn from_fs(path: String) -> Result<File, ()> {
        let timestamp = fs::metadata(&path).unwrap().modified().unwrap();
        Ok(File::new(path, timestamp.into()))
    }

    pub fn from_path(path: String, format: &str) -> Result<File, ()> {
        let timestamp = get_date(&path, format);
        Ok(File::new(path, timestamp))
    }
}

impl Dated for File {
    fn get_date(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

fn from_naive_datetime(path: &str, format: &str) -> DateTime<Utc> {
    let timestamp = match NaiveDateTime::parse_from_str(path, format) {
        Ok(timestamp) => timestamp,
        Err(_) => panic!("Failed to parse path: {}", path)
    };
    Utc.from_utc_datetime(&timestamp)
}

fn from_datetime(path: &str, format: &str) -> DateTime<Utc> {
    let timestamp = match DateTime::parse_from_str(path, format) {
        Ok(timestamp) => timestamp,
        Err(_) => panic!("Failed to parse path: {}", path)
    };
    timestamp.with_timezone(&Utc)

}

fn get_date(path: &str, format: &str) -> DateTime<Utc> {
    match format.contains("%z") {
        true => from_datetime(path, format),
        false => from_naive_datetime(path, format),
    }
}

#[cfg(test)]
pub mod utils {
use super::*;
use std::collections::VecDeque;

pub fn test_file(name: &str, days: usize) -> File {
    let base_time = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    File::new(
        name.to_string(),
        base_time + chrono::Duration::days(days as i64),
    )
}
pub fn to_files(v: Vec<usize>) -> VecDeque<File> {
    v.iter().map(|x| test_file(x.to_string().as_str(), *x)).collect()
}
}


#[cfg(test)]
mod test {
    use super::*;
    use chrono::Datelike;


    #[test]
    fn test_file_from_str1() {
        let input = [
            "pg_2024-02-17_03-00-01.tar",
            "pg_2024-02-29_03-00-01.tar",
            "pg_2024-03-12_03-00-01.tar",
            "pg_2024-03-24_03-00-01.tar",
            "pg_2024-04-05_03-00-01.tar",
            "pg_2024-04-17_03-00-01.tar",
            "pg_2024-04-29_03-00-01.tar",
            "pg_2024-05-11_03-00-01.tar",
            "pg_2024-05-23_03-00-01.tar",
            "pg_2024-02-18_03-00-01.tar",
            "pg_2024-03-01_03-00-01.tar",
            "pg_2024-03-13_03-00-01.tar",
            "pg_2024-03-25_03-00-01.tar"
        ];
        let expected = [
            Utc.with_ymd_and_hms(2024, 2, 17, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 2, 29, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 3, 12, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 3, 24, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 4, 5, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 4, 17, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 4, 29, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 5, 11, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 5, 23, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 2, 18, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 3, 1, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 3, 13, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 3, 25, 3, 0, 1).unwrap()
        ];
        let format = "pg_%Y-%m-%d_%H-%M-%S.tar".to_string();
        input.iter().zip(expected.iter()).for_each(|(i, e)| {
            let file = File::from_path(i.to_string(), &format).unwrap();
            assert_eq!(file.get_date(), *e);
        });
    }

    #[test]
    fn test_file_from_str2() {
        let input = [
            "influx_2024-05-23_03-00-01",
            "influx_2024-05-24_03-00-01",
            "influx_2024-05-25_03-00-01",
            "influx_2024-05-26_03-00-01",
            "influx_2024-05-27_03-00-02",
            "influx_2024-05-28_03-00-01"
        ];
        let expected = [
            Utc.with_ymd_and_hms(2024, 5, 23, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 5, 24, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 5, 25, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 5, 26, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 5, 27, 3, 0, 2).unwrap(),
            Utc.with_ymd_and_hms(2024, 5, 28, 3, 0, 1).unwrap()
        ];
        let format = "influx_%Y-%m-%d_%H-%M-%S".to_string();
        input.iter().zip(expected.iter()).for_each(|(i, e)| {
            let file = File::from_path(i.to_string(), &format).unwrap();
            assert_eq!(file.get_date(), *e);
        });
    }

    #[test]
    fn test_file_from_fs() {
        let tmp_dir = tempdir::TempDir::new("example").unwrap();
        let file_path = tmp_dir.path().join("a");
        let f = fs::File::create(file_path.clone()).unwrap();
        drop(f);
        assert!(file_path.exists());

        let fs_file = File::from_fs(file_path.to_str().unwrap().to_string()).unwrap();

        let memory_file = File::new(file_path.to_str().unwrap().to_string(), Utc::now());
        assert_eq!(fs_file.get_date().month(), memory_file.get_date().month());
    }
}
