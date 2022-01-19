use crossterm::{
  cursor, execute,
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use parking_lot::Mutex;
use std::{io::stdout, sync::Arc};

pub fn bytes_to_kb(bytes: u64) -> u64 {
  return bytes / 1024;
}

pub fn bytes_to_mb(bytes: u64) -> u64 {
  return bytes / 1024 / 1024;
}

pub fn bytes_to_gb(bytes: u64) -> u64 {
  return bytes / 1024 / 1024 / 1024;
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

pub fn clear_screen() {
  execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
}

pub fn reset_cursor() {
  execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
}
