use chrono::{TimeZone, Utc};
use clap::Parser;

#[derive(Parser)]
#[command(name = "untd")]
struct Args {
    timestamp: Option<i64>,
    /// Timezone (e.g., "UTC", "JST")
    #[arg(short = 'z', long = "timezone", default_value = "UTC")]
    timezone: String,
    /// Copy output to clipboard
    #[arg(short = 'c', long = "copy")]
    copy: bool,
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

    let output = specific_datetime.format("%Y-%m-%d %H:%M:%S%z").to_string();
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
