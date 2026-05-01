use chrono::{DateTime, NaiveTime, Utc};

use crate::{Error, Result};

pub(super) fn timestamp_from_millis(millis: i64) -> Result<DateTime<Utc>> {
    DateTime::from_timestamp_millis(millis)
        .ok_or_else(|| Error::ConversionError(format!("timestamp {millis} is out of range")))
}

pub(super) fn naive_time_from_parts(hour: u32, minute: u32) -> Result<NaiveTime> {
    NaiveTime::from_hms_opt(hour, minute, 0).ok_or_else(|| {
        Error::ConversionError(format!(
            "naive time (hour: {hour}, minute: {minute}) is out of range"
        ))
    })
}
