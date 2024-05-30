use chrono::{DateTime, Utc};
use std::str::FromStr;

use crate::dated::Dated;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct File {
    pub path: String,
    timestamp: DateTime<Utc>,
}

impl File {
    pub fn new(path: String, timestamp: DateTime<Utc>) -> File {
        File {
            path: String::new(),
            timestamp,
        }
    }
}

impl FromStr for File {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let timestamp = DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc);
        Ok(File::new(s.to_string(), timestamp))
    }
}

impl Dated for File {
    fn get_date(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn test_file_from_str1() {
        let input = vec![
            "pg_2024-02-17_03--00-01.tar",
            "pg_2024-02-29_03--00-01.tar",
            "pg_2024-03-12_03--00-01.tar",
            "pg_2024-03-24_03--00-01.tar",
            "pg_2024-04-05_03--00-01.tar",
            "pg_2024-04-17_03--00-01.tar",
            "pg_2024-04-29_03--00-01.tar",
            "pg_2024-05-11_03--00-01.tar",
            "pg_2024-05-23_03--00-01.tar",
            "pg_2024-02-18_03--00-01.tar",
            "pg_2024-03-01_03--00-01.tar",
            "pg_2024-03-13_03--00-01.tar",
            "pg_2024-03-25_03--00-01.tar"
        ];
        let expected = vec![
            Utc.with_ymd_and_hms(2024, 2, 17, 3, 0, 1),
            Utc.with_ymd_and_hms(2024, 2, 29, 3, 0, 1),
            Utc.with_ymd_and_hms(2024, 3, 12, 3, 0, 1),
            Utc.with_ymd_and_hms(2024, 3, 24, 3, 0, 1),
            Utc.with_ymd_and_hms(2024, 4, 5, 3, 0, 1),
            Utc.with_ymd_and_hms(2024, 4, 17, 3, 0, 1),
            Utc.with_ymd_and_hms(2024, 4, 29, 3, 0, 1),
            Utc.with_ymd_and_hms(2024, 5, 11, 3, 0, 1),
            Utc.with_ymd_and_hms(2024, 5, 23, 3, 0, 1),
            Utc.with_ymd_and_hms(2024, 2, 18, 3, 0, 1),
            Utc.with_ymd_and_hms(2024, 3, 1, 3, 0, 1),
            Utc.with_ymd_and_hms(2024, 3, 13, 3, 0, 1),
            Utc.with_ymd_and_hms(2024, 3, 25, 3, 0, 1)
        ];
        let format = "pg_\\%Y-\\%m-\\%d_\\%H--\\%M-\\%S.tar";
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
            Utc.with_ymd_and_hms(2024, 5, 23, 3, 0, 1),
            Utc.with_ymd_and_hms(2024, 5, 24, 3, 0, 1),
            Utc.with_ymd_and_hms(2024, 5, 25, 3, 0, 1),
            Utc.with_ymd_and_hms(2024, 5, 26, 3, 0, 1),
            Utc.with_ymd_and_hms(2024, 5, 27, 3, 0, 2),
            Utc.with_ymd_and_hms(2024, 5, 28, 3, 0, 1)
        ];
        let format = "influx_\\%Y-\\%m-\\%d_\\%H-\\%M-\\%S";
    }
}

// "pg_\\%Y-\\%m-\\%d_\\%H--\\%M-\\%S.tar"
// "influx_\\%Y-\\%m-\\%d_\\%H-\\%M-\\%S"
//
// pg_2024-02-17_03--00-01.tar
// pg_2024-02-29_03--00-01.tar
// pg_2024-03-12_03--00-01.tar
// pg_2024-03-24_03--00-01.tar
// pg_2024-04-05_03--00-01.tar
// pg_2024-04-17_03--00-01.tar
// pg_2024-04-29_03--00-01.tar
// pg_2024-05-11_03--00-01.tar
// pg_2024-05-23_03--00-01.tar
// pg_2024-02-18_03--00-01.tar
// pg_2024-03-01_03--00-01.tar
// pg_2024-03-13_03--00-01.tar
// pg_2024-03-25_03--00-01.tar
//
// influx_2024-05-23_03-00-01
// influx_2024-05-24_03-00-01
// influx_2024-05-25_03-00-01
// influx_2024-05-26_03-00-01
// influx_2024-05-27_03-00-02
// influx_2024-05-28_03-00-01
