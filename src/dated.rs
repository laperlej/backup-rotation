use chrono::{DateTime, Utc};

pub trait Dated {
    fn get_date(&self) -> DateTime<Utc>;
}
