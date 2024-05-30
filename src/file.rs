use chrono::{DateTime, Utc, NaiveDateTime, TimeZone};
use std::fs;

use crate::dated::Dated;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct File {
    pub path: String,
    timestamp: DateTime<Utc>,
}

trait FileFactory {
    fn create_file(&self, path: &str) -> Result<File, ()>;
}

pub struct PathFileFactory {
    format: String,
}

impl PathFileFactory {
    pub fn new(format: String) -> Self {
        PathFileFactory {
            format,
        }
    }
}

impl FileFactory for PathFileFactory {
    fn create_file(&self, path: &str) -> Result<File, ()> {
        let timestamp = get_date(path, &self.format);
        Ok(File::new(path.to_string(), timestamp))
    }

}

struct FsFileFactory {
}

impl FsFileFactory {
    pub fn new() -> Self {
        FsFileFactory {}
    }
}

impl FileFactory for FsFileFactory {
    fn create_file(&self, path: &str) -> Result<File, ()> {
        let timestamp = fs::metadata(path).unwrap().modified().unwrap();
        Ok(File::new(path.to_string(), timestamp.into()))
    }
}

impl File {
    pub fn new(path: String, timestamp: DateTime<Utc>) -> File {
        File {
            path,
            timestamp,
        }
    }
}

impl Dated for File {
    fn get_date(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

fn from_naive_datetime(path: &str, format: &str) -> DateTime<Utc> {
    let timestamp = NaiveDateTime::parse_from_str(path, format).unwrap();
    Utc.from_utc_datetime(&timestamp)
}

fn from_datetime(path: &str, format: &str) -> DateTime<Utc> {
    let timestamp = DateTime::parse_from_str(path, format).unwrap();
    timestamp.with_timezone(&Utc)

}

fn get_date(path: &str, format: &str) -> DateTime<Utc> {
    match format.contains("%z") {
        true => from_datetime(path, format),
        false => from_naive_datetime(path, format),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_file_from_str1() {
        let input = vec![
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
        let expected = vec![
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
        let file_factory = PathFileFactory::new(format);
        input.iter().zip(expected.iter()).for_each(|(i, e)| {
            let file = file_factory.create_file(i).unwrap();
            assert_eq!(file.get_date(), *e);
        });
    }

    #[test]
    fn test_file_from_str2() {
        let input = vec![
            "influx_2024-05-23_03-00-01",
            "influx_2024-05-24_03-00-01",
            "influx_2024-05-25_03-00-01",
            "influx_2024-05-26_03-00-01",
            "influx_2024-05-27_03-00-02",
            "influx_2024-05-28_03-00-01"
        ];
        let expected = vec![
            Utc.with_ymd_and_hms(2024, 5, 23, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 5, 24, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 5, 25, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 5, 26, 3, 0, 1).unwrap(),
            Utc.with_ymd_and_hms(2024, 5, 27, 3, 0, 2).unwrap(),
            Utc.with_ymd_and_hms(2024, 5, 28, 3, 0, 1).unwrap()
        ];
        let format = "influx_%Y-%m-%d_%H-%M-%S".to_string();
        let file_factory = PathFileFactory::new(format);
        input.iter().zip(expected.iter()).for_each(|(i, e)| {
            let file = file_factory.create_file(i).unwrap();
            assert_eq!(file.get_date(), *e);
        });
    }
}
