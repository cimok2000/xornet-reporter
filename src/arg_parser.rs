use colored::Colorize;

/// The structure of the launch parameters.
pub struct ArgParser {
    pub borderless: bool,
    pub prefix: String,
    pub colorless: bool,
    pub interval: f64,
    pub no_clear: bool,
    pub silent: bool,
}

impl ArgParser {
    /// Handles the input arguments.
    /// Currently only custom config parameter is supported
    ///
    /// # Returns
    /// * `String` - Path to the custom config file
    ///
    ///
    /// # Errors
    /// * `std::env::VarError` - If the environment variable could not be read or parsed
    /// * `std::env::VarError` - If the environment variable is not set
    pub fn new() -> ArgParser {
        let mut arg_parser = ArgParser {
            borderless: false,
            prefix: "●".to_string(),
            interval: 1.0,
            colorless: false,
            silent: false,
            no_clear: false,
        };
        let args: Vec<String> = std::env::args().collect();
        let mut index: usize = 0;
        while args.len() > index {
            let arg: &str = &args[index];
            match arg {
                "-h" | "--help" => {
                    println!("\n{} Usage:", "●".green());
                    println!("    xornet-reporter {}", "[options]".bright_black());
                    println!("\n{} Options:", "●".blue());
                    println!(
                        "    -h,  --help                         : {}",
                        "Show this help".white()
                    );
                    println!(
                        "    -v,  --version                      : {}",
                        "Show version and exit".white()
                    );
                    println!(
                        "    -i,  --interval    {}     : {}",
                        "(default: 1)".bright_black(),
                        "Data collection interval in seconds".white()
                    );
                    println!(
                        "    -b,  --borderless  {} : {}",
                        "(default: false)".bright_black(),
                        "Borderless style".white()
                    );
                    println!(
                        "    -p,  --prefix      {}   : {}",
                        "(default: \"●\")".bright_black(),
                        "Prefix that is shown at the beginning of each field".white()
                    );
                    println!(
                        "    -nc, --no-clear    {} : {}",
                        "(default: false)".bright_black(),
                        "Disables the terminal clearing on each interval".white()
                    );
                    println!(
                        "    -s,  --silent   {} : {}",
                        "(default: false)".bright_black(),
                        "Enables simple terminal output".white()
                    );
                    println!(
                        "    -c,  --colorless   {} : {}",
                        "(default: false)".bright_black(),
                        "Colorless style".white()
                    );
                    println!("\n{} Examples:", "●".magenta());
                    println!("    {} xornet-reporter", "$".bright_black());
                    println!("    {} xornet-reporter -b ", "$".bright_black());
                    println!("    {} xornet-reporter -p \">\"", "$".bright_black());
                    println!();
                    std::process::exit(0);
                }
                "-b" | "--borderless" => {
                    arg_parser.borderless = true;
                }
                "-nc" | "--no-clear" => {
                    arg_parser.no_clear = true;
                }
                "-s" | "--silent" => {
                    arg_parser.silent = true;
                }
                "-c" | "--colorless" => {
                    println!("{}", "Colorless parameter isn't implemented".red());
                    std::process::exit(1);
                    arg_parser.colorless = true;
                }
                "-v" | "--version" => {
                    println!("xornet-reporter v{}", env!("CARGO_PKG_VERSION"));
                    std::process::exit(0);
                }
                "-p" | "--prefix" => {
                    if args.len() > index + 1 {
                        index += 1;
                        arg_parser.prefix = args[index].to_string();
                    } else {
                        println!(
                            "{}",
                            "Missing argument for option -p <prefix>, use -h for help".red()
                        );
                        std::process::exit(1);
                    }
                }
                "-i" | "--interval" => {
                    if args.len() > index + 1 {
                        index += 1;
                        arg_parser.interval = args[index]
                            .parse::<f64>()
                            .expect("Could not parse interval as integer");
                    } else {
                        println!(
                            "{}",
                            "Missing argument for option -i <interval>, use -h for help".red()
                        );
                        std::process::exit(1);
                    }
                }
                _ => {}
            }
            index += 1;
        }
        return arg_parser;
    }
}
