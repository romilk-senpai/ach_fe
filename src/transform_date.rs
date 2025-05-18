use chrono::{DateTime, Utc, Duration};

pub fn transform_date(date_str: &str) -> String {
    let datetime = DateTime::parse_from_rfc3339(date_str)
        .expect("Failed to parse date string")
        .with_timezone(&Utc);
    
    let now = Utc::now();
    let duration = now.signed_duration_since(datetime);
    
    let days = duration.num_days();
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;
    
    let elapsed = if days > 0 {
        format!("{}d, {}h {}m and {}s ago", days, hours, minutes, seconds)
    } else if hours > 0 {
        format!("{}h {}m and {}s ago", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m and {}s ago", minutes, seconds)
    } else {
        format!("{}s ago", seconds)
    };
    
    format!("{} ({})", datetime.format("%B %d, %Y at %I:%M %p"), elapsed)
}
