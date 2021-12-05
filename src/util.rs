use crossterm::{
    cursor, execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use parking_lot::Mutex;
use serde_json::Value;
use std::{io::stdout, sync::Arc};

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
