use std::result::Result as StdResult;

use chrono::{DateTime, NaiveTime, Utc};
use serde::de::Error as _;

pub(super) fn timestamp_from_millis(millis: i64) -> StdResult<DateTime<Utc>, serde_json::Error> {
    DateTime::from_timestamp_millis(millis)
        .ok_or_else(|| serde_json::Error::custom(format!("timestamp {millis} is out of range")))
}

pub(super) fn naive_time_from_parts(
    hour: u32,
    minute: u32,
) -> StdResult<NaiveTime, serde_json::Error> {
    NaiveTime::from_hms_opt(hour, minute, 0).ok_or_else(|| {
        serde_json::Error::custom(format!(
            "naive time (hour: {hour}, minute: {minute}) is out of range"
        ))
    })
}
