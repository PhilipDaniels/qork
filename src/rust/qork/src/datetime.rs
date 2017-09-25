/*
use chrono::{Utc, TimeZone, DateTime};

pub fn system_time_to_date_time(t: SystemTime) -> DateTime<Utc> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => { // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        },
    };

    Utc.timestamp(sec, nsec)
}
*/

use std::time::{SystemTime, Duration, UNIX_EPOCH};
use time::{Timespec, Tm, at_utc, strftime};

pub fn system_time_to_duration(value: &SystemTime) -> Duration {
    value.duration_since(UNIX_EPOCH).expect("value should be after the start of the Unix epoch")
}

pub fn duration_to_timespec(value: &Duration) -> Timespec {
    Timespec::new(value.as_secs() as i64, value.subsec_nanos() as i32)
}

pub fn system_time_to_utc_tm(value: &SystemTime) -> Tm {
    let dur = system_time_to_duration(&value);
    let ts = duration_to_timespec(&dur);
    at_utc(ts)
}

pub fn format_system_time_as_utc(value: &SystemTime) -> String {
    let value = system_time_to_utc_tm(&value);
    format_tm_as_utc(&value)
}

pub fn format_duration_as_utc(value: &Duration) -> String {
    let ts = duration_to_timespec(&value);
    format_tm_as_utc(&at_utc(ts))
}

pub fn format_tm_as_utc(value: &Tm) -> String {
    strftime("%Y-%m-%d %H:%M:%S UTC", &value).expect("strftime must succeed")
}
