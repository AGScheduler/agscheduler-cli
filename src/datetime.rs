use chrono::{DateTime, Local, ParseError, TimeZone};

pub fn parse_iso8601_to_local(iso8601_str: &str) -> Result<DateTime<Local>, ParseError> {
    let utc_datetime = DateTime::parse_from_rfc3339(iso8601_str)?;

    let local_datetime = Local.from_utc_datetime(&utc_datetime.naive_utc());

    Ok(local_datetime)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn it_parse_iso8601_to_local() {
        let iso8601_str = "2024-04-13T14:35:55Z";
        let utc_datetime = Utc.with_ymd_and_hms(2024, 4, 13, 14, 35, 55).unwrap();
        let local_datetime = Local.from_utc_datetime(&utc_datetime.naive_utc());

        assert_eq!(local_datetime, parse_iso8601_to_local(iso8601_str).unwrap());
    }
}
