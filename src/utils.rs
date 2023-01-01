use std::{ops::Deref, time::SystemTime};

use bincode::{Decode, Encode};
use chrono::{Datelike, NaiveDateTime};

/// use for cache
#[derive(Debug, Copy, Clone, Decode, Encode)]
pub struct SystemTimeWrapper(SystemTime);

impl Default for SystemTimeWrapper {
    fn default() -> Self {
        Self(SystemTime::now())
    }
}

impl Deref for SystemTimeWrapper {
    type Target = SystemTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait MonitorTime {
    fn create_at(&self) -> SystemTimeWrapper;
}

#[inline]
pub fn current_year() -> i32 {
    let d = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let date = NaiveDateTime::from_timestamp_millis(d.as_millis() as i64).unwrap();
    date.year()
}
