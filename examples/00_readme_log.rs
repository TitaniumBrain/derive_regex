use derive_regex::FromRegex;

#[derive(Debug, FromRegex, PartialEq, Eq)]
#[regex(
    pattern = r"^(?P<timestamp>\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}) \[(?P<level>[A-Z]+)\] (?P<message>.+)$"
)]
struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

fn main() {
    let log = "2025-02-20 15:30:00 [INFO] Server started successfully";
    let entry = LogEntry::parse(log).expect("Failed to parse log entry");
    println!("Parsed log entry: {:#?}", entry);
    // Parsed log entry: LogEntry {
    //     timestamp: "2025-02-20 15:30:00",
    //     level: "INFO",
    //     message: "Server started successfully",
    // }
}
