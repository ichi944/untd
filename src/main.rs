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
}

fn main() {
    let args: Args = Args::parse();

    let datetime = if let Some(dt) = args.timestamp {
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
