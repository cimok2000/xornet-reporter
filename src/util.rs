use colored::Colorize;
use parking_lot::Mutex;
use serde_json::Value;
use std::sync::Arc;

/// The structure of the launch parameters.
pub struct LaunchParams {
    pub borderless: bool,
    pub prefix: String,
    pub colorless: bool,
}

impl LaunchParams {
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
    pub fn new() -> LaunchParams {
        let mut launch_params = LaunchParams {
            borderless: false,
            prefix: "●".to_string(),
            colorless: false,
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
                        "    -h, --help                         : {}",
                        "Show this help".white()
                    );
                    println!(
                        "    -v, --version                      : {}",
                        "Show version and exit".white()
                    );
                    println!(
                        "    -b, --borderless  {} : {}",
                        "(default: false)".bright_black(),
                        "Borderless style".white()
                    );
                    println!(
                        "    -p, --prefix      {}   : {}",
                        "(default: \"●\")".bright_black(),
                        "Prefix that is shown at the beginning of each field".white()
                    );
                    println!(
                        "    -c, --colorless   {} : {}",
                        "(default: false)".bright_black(),
                        "Colorless style".white()
                    );
                    println!("\n{} Examples:", "●".magenta());
                    println!("    $ xornet-reporter");
                    println!("    $ xornet-reporter -b ");
                    println!("    $ xornet-reporter -p \">\"");
                    println!();
                    std::process::exit(0);
                }
                "-b" | "--borderless" => {
                    launch_params.borderless = true;
                }
                "-c" | "--colorless" => {
                    launch_params.colorless = true;
                }
                "-v" | "--version" => {
                    println!("xornet-reporter v{}", env!("CARGO_PKG_VERSION"));
                    std::process::exit(0);
                }
                "-p" | "--prefix" => {
                    if args.len() > index + 1 {
                        index += 1;
                        launch_params.prefix = args[index].to_string();
                    } else {
                        println!(
                            "{}",
                            "Missing argument for option -p <prefix>, use -h for help".red()
                        );
                        std::process::exit(1);
                    }
                }
                _ => {}
            }
            index += 1;
        }
        return launch_params;
    }
}

pub fn bytes_to_kb(bytes: &Value) -> String {
    return (bytes.as_i64().unwrap() / 1024).to_string();
}

pub fn bytes_to_mb(bytes: &Value) -> String {
    return (bytes.as_i64().unwrap() / 1024 / 1024).to_string();
}

pub fn bytes_to_gb(bytes: &Value) -> String {
    return (bytes.as_i64().unwrap() / 1024 / 1024 / 1024).to_string();
}

pub fn arcmutex<T>(item: T) -> Arc<Mutex<T>> {
    return Arc::new(Mutex::new(item));
}
