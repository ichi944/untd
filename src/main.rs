use chrono::{TimeZone, Utc};
use clap::Parser;

/// Determines the datetime format string based on the format option
fn get_format_string(format_option: Option<&str>) -> &str {
    match format_option {
        None => "%Y-%m-%d",                   // Default: date only
        Some("iso") => "%Y-%m-%dT%H:%M:%S%z", // ISO8601
        Some("jp") => "%Y年%m月%d日",
        Some("jpwd") => "%Y年%m月%d日(%w)", // Japanese date with weekday placeholder
        Some("jphm") => "%Y年%m月%d日 %H時%M分",
        Some("jphms") => "%Y年%m月%d日 %H時%M分%S秒",
        Some(fmt) => fmt, // Custom format
    }
}

/// Converts a weekday number to Japanese character
/// 0 = Sunday (日), 1 = Monday (月), etc.
fn get_japanese_weekday(weekday_num: char) -> &'static str {
    match weekday_num {
        '0' => "日",
        '1' => "月",
        '2' => "火",
        '3' => "水",
        '4' => "木",
        '5' => "金",
        '6' => "土",
        _ => "?",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_format_string() {
        // Test default format (None)
        assert_eq!(get_format_string(None), "%Y-%m-%d");

        // Test ISO format
        assert_eq!(get_format_string(Some("iso")), "%Y-%m-%dT%H:%M:%S%z");

        // Test Japanese formats
        assert_eq!(get_format_string(Some("jp")), "%Y年%m月%d日");
        assert_eq!(get_format_string(Some("jpwd")), "%Y年%m月%d日(%w)");
        assert_eq!(get_format_string(Some("jphm")), "%Y年%m月%d日 %H時%M分");
        assert_eq!(
            get_format_string(Some("jphms")),
            "%Y年%m月%d日 %H時%M分%S秒"
        );

        // Test custom format
        assert_eq!(get_format_string(Some("%H:%M:%S")), "%H:%M:%S");
        assert_eq!(
            get_format_string(Some("Custom: %Y/%m/%d")),
            "Custom: %Y/%m/%d"
        );
    }

    #[test]
    fn test_get_japanese_weekday() {
        assert_eq!(get_japanese_weekday('0'), "日");
        assert_eq!(get_japanese_weekday('1'), "月");
        assert_eq!(get_japanese_weekday('2'), "火");
        assert_eq!(get_japanese_weekday('3'), "水");
        assert_eq!(get_japanese_weekday('4'), "木");
        assert_eq!(get_japanese_weekday('5'), "金");
        assert_eq!(get_japanese_weekday('6'), "土");
        assert_eq!(get_japanese_weekday('9'), "?");
    }

    #[test]
    fn test_parse_time_adjustment() {
        use chrono::Duration;

        // Test positive adjustments
        assert_eq!(parse_time_adjustment("30s").unwrap(), Duration::seconds(30));
        assert_eq!(parse_time_adjustment("5m").unwrap(), Duration::minutes(5));
        assert_eq!(parse_time_adjustment("2h").unwrap(), Duration::hours(2));
        assert_eq!(parse_time_adjustment("1d").unwrap(), Duration::days(1));
        assert_eq!(parse_time_adjustment("3w").unwrap(), Duration::weeks(3));

        // Test with explicit plus sign
        assert_eq!(
            parse_time_adjustment("+45s").unwrap(),
            Duration::seconds(45)
        );
        assert_eq!(
            parse_time_adjustment("+10m").unwrap(),
            Duration::minutes(10)
        );

        // Test negative adjustments
        assert_eq!(
            parse_time_adjustment("-15s").unwrap(),
            Duration::seconds(-15)
        );
        assert_eq!(parse_time_adjustment("-3m").unwrap(), Duration::minutes(-3));
        assert_eq!(parse_time_adjustment("-1h").unwrap(), Duration::hours(-1));
        assert_eq!(parse_time_adjustment("-2d").unwrap(), Duration::days(-2));
        assert_eq!(parse_time_adjustment("-1w").unwrap(), Duration::weeks(-1));

        // Test error cases
        assert!(parse_time_adjustment("").is_err()); // Empty string
        assert!(parse_time_adjustment("s").is_err()); // Missing numeric part
        assert!(parse_time_adjustment("10").is_err()); // Missing unit
        assert!(parse_time_adjustment("10x").is_err()); // Invalid unit
    }
}

/// Parse a time adjustment string like "1m", "-30s", "2d"
fn parse_time_adjustment(adj: &str) -> Result<chrono::Duration, String> {
    if adj.is_empty() {
        return Err("Empty time adjustment string".to_string());
    }

    // Check if it's a negative adjustment
    let (is_negative, adj_str) = if adj.starts_with('-') {
        (true, &adj[1..])
    } else if adj.starts_with('+') {
        (false, &adj[1..])
    } else {
        (false, adj)
    };

    // Parse the numeric part and unit
    let mut numeric_part = String::new();
    let mut unit_part = String::new();

    for c in adj_str.chars() {
        if c.is_digit(10) {
            numeric_part.push(c);
        } else {
            unit_part.push(c);
        }
    }

    if numeric_part.is_empty() {
        return Err(format!("Missing numeric part in '{}'", adj));
    }

    let value: i64 = numeric_part
        .parse()
        .map_err(|e| format!("Invalid number: {}", e))?;
    let value = if is_negative { -value } else { value };

    match unit_part.as_str() {
        "s" => Ok(chrono::Duration::seconds(value)),
        "m" => Ok(chrono::Duration::minutes(value)),
        "h" => Ok(chrono::Duration::hours(value)),
        "d" => Ok(chrono::Duration::days(value)),
        "w" => Ok(chrono::Duration::weeks(value)),
        _ => Err(format!("Unknown time unit '{}'. Use s (seconds), m (minutes), h (hours), d (days), or w (weeks)", unit_part)),
    }
}

#[derive(Parser)]
#[command(name = "untd")]
struct Args {
    timestamp: Option<i64>,
    /// Timezone (e.g., "UTC", "JST")
    #[arg(short = 'z', long = "timezone", default_value = "JST")]
    timezone: String,
    /// Copy output to clipboard
    #[arg(short = 'c', long = "copy", default_value = "true")]
    copy: bool,
    /// Output format (default: date only, "iso": ISO8601, "jp": Japanese date, "jpwd": Japanese date with weekday, "jphm": Japanese date with time, "jphms": Japanese date with time and seconds)
    #[arg(short = 'f', long = "format")]
    format: Option<String>,
    /// Adjust time (e.g., "1m" adds 1 minute, "-30s" subtracts 30 seconds, "2d" adds 2 days)
    /// Supported units: s (seconds), m (minutes), h (hours), d (days), w (weeks)
    #[arg(short = 'a', long = "adjust")]
    adjust: Option<String>,
}

fn main() {
    let args: Args = Args::parse();

    let mut datetime = if let Some(dt) = args.timestamp {
        match Utc.timestamp_opt(dt, 0) {
            chrono::LocalResult::Single(dt) => dt,
            _ => {
                println!("Invalid timestamp");
                std::process::exit(1);
            }
        }
    } else {
        Utc::now()
    };

    // Apply time adjustment if specified
    if let Some(adj_str) = &args.adjust {
        match parse_time_adjustment(adj_str) {
            Ok(duration) => {
                datetime = datetime + duration;
            }
            Err(e) => {
                println!("Error in time adjustment: {}", e);
                std::process::exit(1);
            }
        }
    }

    let tz = match args.timezone.as_str() {
        "UTC" => chrono_tz::UTC,
        "JST" => chrono_tz::Asia::Tokyo,
        _ => {
            println!("Invalid timezone");
            std::process::exit(1);
        }
    };
    let specific_datetime = datetime.with_timezone(&tz);

    let format_str = get_format_string(args.format.as_deref());

    let formatted = specific_datetime.format(format_str).to_string();

    // Special handling for Japanese weekday format
    let output = if args.format.as_deref() == Some("jpwd") {
        // Replace the %w placeholder with the Japanese weekday character
        formatted
            .chars()
            .enumerate()
            .fold(String::new(), |mut result, (i, c)| {
                if i > 0
                    && formatted.chars().nth(i - 1) == Some('(')
                    && c.is_digit(10)
                    && i + 1 < formatted.len()
                    && formatted.chars().nth(i + 1) == Some(')')
                {
                    result.push_str(get_japanese_weekday(c));
                } else if !(i > 0
                    && formatted.chars().nth(i - 1) == Some('(')
                    && formatted.chars().nth(i) == Some(')'))
                {
                    result.push(c);
                }
                result
            })
    } else {
        formatted
    };

    println!("{}", output);

    if args.copy {
        match arboard::Clipboard::new() {
            Ok(mut clipboard) => {
                if let Err(e) = clipboard.set_text(&output) {
                    eprintln!("Failed to copy to clipboard: {}", e);
                } else {
                    println!("Copied to clipboard!");
                }
            }
            Err(e) => eprintln!("Failed to access clipboard: {}", e),
        }
    }
}
