use chrono::{TimeZone, Utc};
use clap::Parser;

/// Determines the datetime format string based on the format option
fn get_format_string(format_option: Option<&str>) -> &str {
    match format_option {
        None => "%Y-%m-%d",                   // Default: date only
        Some("iso") => "%Y-%m-%dT%H:%M:%S%z", // ISO8601
        Some(fmt) => fmt,                     // Custom format
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

        // Test custom format
        assert_eq!(get_format_string(Some("%H:%M:%S")), "%H:%M:%S");
        assert_eq!(
            get_format_string(Some("Custom: %Y/%m/%d")),
            "Custom: %Y/%m/%d"
        );
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
    /// Output format (default: date only, "iso": ISO8601)
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

    let output = specific_datetime.format(format_str).to_string();
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
