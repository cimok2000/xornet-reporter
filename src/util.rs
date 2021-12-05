use colored::Colorize;
use crossterm::{
    cursor, execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use parking_lot::Mutex;
use serde_json::Value;
use std::{io::stdout, sync::Arc};

/// The structure of the launch parameters.
pub struct LaunchParams {
    pub borderless: bool,
    pub prefix: String,
    pub colorless: bool,
    pub interval: f64,
    pub no_clear: bool,
    pub silent: bool,
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
                    launch_params.borderless = true;
                }
                "-nc" | "--no-clear" => {
                    launch_params.no_clear = true;
                }
                "-s" | "--silent" => {
                    launch_params.silent = true;
                }
                "-c" | "--colorless" => {
                    println!("{}", "Colorless parameter isn't implemented".red());
                    std::process::exit(1);
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
                "-i" | "--interval" => {
                    if args.len() > index + 1 {
                        index += 1;
                        launch_params.interval = args[index]
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
        return launch_params;
    }
}

pub fn trim_one_character(string: &str) -> String {
    return string[1..string.len() - 1].to_string();
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

pub fn setup_terminal() {
    // Enter alternate screen, move to 0, 0 and hide the cursor
    execute!(
        stdout(),
        EnterAlternateScreen,
        cursor::MoveTo(0, 0),
        cursor::Hide
    )
    .unwrap();

    // Create the CTRL + C handler
    ctrlc::set_handler(move || {
        // Restore the cursor and leave alternate screen
        execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
        // Exit the program
        std::process::exit(0);
    })
    .expect("Ctrl + C handler failed to be set");
}

pub fn reset_cursor() {
    execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
}
