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
