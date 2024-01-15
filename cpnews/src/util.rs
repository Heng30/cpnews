use chrono::{FixedOffset, TimeZone, Utc};

pub fn timestamp() -> i64 {
    Utc::now().timestamp()
}

pub fn time_from_utc_seconds(sec: i64) -> String {
    let time = FixedOffset::east_opt(0)
        .unwrap()
        .timestamp_opt(sec, 0)
        .unwrap();
    format!("{}", time.format("%Y-%m-%d %H:%M"))
}
