use chrono::{DateTime, Local, ParseError, TimeZone};

pub fn parse_iso8601_to_local(iso8601_str: &str) -> Result<DateTime<Local>, ParseError> {
    let utc_datetime = DateTime::parse_from_rfc3339(iso8601_str)?;

    let local_datetime = Local.from_utc_datetime(&utc_datetime.naive_utc());

    Ok(local_datetime)
}
