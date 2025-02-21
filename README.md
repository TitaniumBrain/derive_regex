# derive_regex

`derive_regex` is a simple crate that exports a trait that you can derive on an enum or struct to construct it from a string using a regular expression.

## Features

- Extract data into a struct's fields using regex capturing groups.
- Compile time validation of regular expressions, including:
  - testing that the `regex::Regex` will build
  - checking that the correct amount and name of capturing groups were defined in the expression


## Installation

You can use `cargo` to add this crate as a dependency or just manually edit `Cargo.toml`.

```bash
cargo add derive_regex
```

You'll also need to depend on the [`regex`] crate as that's what's used to parse the strings. 

## Example Usage

A simple example use case is parsing lines from log files into a Rust type:

```rust
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
```

Another, more powerful application is destructuring a string into one of an enum's variants.
For example, take this hypothetical text based cooking game, where the player can type cooking instructions:

```rust
use derive_regex::FromRegex;

#[derive(Debug, FromRegex, PartialEq)]
enum CookingCommand {
    // Parses a command like "chop 3 carrots"
    #[regex(pattern = r"chop (?P<quantity>\d+) (?P<ingredient>\w+)")]
    Chop { quantity: u32, ingredient: String },

    // Parses a command like "boil for 10 minutes"
    #[regex(pattern = r"boil for (?P<minutes>\d+) minutes")]
    Boil(u32),

    // Parses a command like "bake at 375.0 degrees for 25 minutes"
    #[regex(pattern = r"bake at (?P<temperature>\d+\.\d+) degrees for (?P<minutes>\d+) minutes")]
    Bake { temperature: f64, minutes: u32 },

    // Parses a command like "mix salt and pepper"
    #[regex(pattern = r"mix (?P<ingredient1>\w+) and (?P<ingredient2>\w+)")]
    Mix {
        ingredient1: String,
        ingredient2: String,
    },
}

fn main() {
    let commands = [
        "First, chop 3 carrots",
        "Don't forget to boil for 10 minutes",
        "I guess I'll bake at 375.0 degrees for 25 minutes",
        "mix salt and pepper now",
    ];

    for cmd in &commands {
        if let Ok(command) = CookingCommand::parse(cmd) {
            match command {
                CookingCommand::Chop {
                    quantity,
                    ingredient,
                } => {
                    println!("Chop {} {}(s)", quantity, ingredient);
                }
                CookingCommand::Boil(minutes) => {
                    println!("Boil for {} minutes", minutes);
                }
                CookingCommand::Bake {
                    temperature,
                    minutes,
                } => {
                    println!("Bake at {} degrees for {} minutes", temperature, minutes);
                }
                CookingCommand::Mix {
                    ingredient1,
                    ingredient2,
                } => {
                    println!("Mix {} and {}", ingredient1, ingredient2);
                }
            }
        } else {
            eprintln!("Failed to parse command: {}", cmd);
        }
    }
    // Chop 3 carrots(s)
    // Boil for 10 minutes
    // Bake at 375 degrees for 25 minutes
    // Mix salt and pepper
}
```
