use chrono::{DateTime, Utc};

pub fn transform_date(date_str: &str) -> String {
    let datetime = DateTime::parse_from_rfc3339(date_str)
        .expect("Failed to parse date string")
        .with_timezone(&Utc);
    datetime.format("%B %d, %Y at %I:%M %p").to_string()
}
