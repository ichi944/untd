## What's this

This utility script provides easy interface to getting date time strings in various formats, with support for date ranges and time adjustments.

## Installation

```bash
cargo install untd
```

## Usage

Basic usage:
```bash
untd                    # Outputs today's date in default format (YYYY-MM-DD)
untd -f iso            # Outputs in ISO8601 format
untd -f jp             # Outputs in Japanese format
```

### Format Options (-f)

- Default: `YYYY-MM-DD`
- `iso`: ISO8601 format (`YYYY-MM-DDThh:mm:ss+zzzz`)
- `jp`: Japanese date format (`YYYY年MM月DD日`)
- `jpwd`: Japanese date with weekday (`YYYY年MM月DD日(曜)`)
- `jphm`: Japanese date with time (`YYYY年MM月DD日 HH時MM分`)
- `jphms`: Japanese date with time and seconds (`YYYY年MM月DD日 HH時MM分SS秒`)
- Custom format strings are also supported using strftime format

### Time Adjustments (-a)

Adjust the output date/time:
```bash
untd -a 1d             # Tomorrow
untd -a -1d            # Yesterday
untd -a 2h             # 2 hours from now
untd -a -30m           # 30 minutes ago
```

Supported units: d (days), h (hours), m (minutes), s (seconds)

### Date Ranges (-r)

Output multiple consecutive dates:
```bash
untd -r 5              # Output next 5 days
untd -r 5 -a -2d      # Output 5 days starting from 2 days ago
```

### Additional Options

- `-c`: Copy output to clipboard
- `-z`: Specify timezone (default: local timezone)

## Examples

```bash
# Get today's date in Japanese format with weekday
untd -f jpwd
2025年03月24日(月)

# Get next 3 days in ISO format
untd -f iso -r 3
2025-03-24T09:38:29+0900
2025-03-25T09:38:29+0900
2025-03-26T09:38:29+0900

# Get time 2 hours ago in Japanese format with time
untd -f jphm -a -2h
2025年03月24日 07時38分

# Get dates for next week and copy to clipboard
untd -r 7 -c
```
