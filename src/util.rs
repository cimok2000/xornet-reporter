use crossterm::{
  cursor, execute,
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use parking_lot::Mutex;
use std::{io::stdout, sync::Arc};

pub fn bytes_to_kb(bytes: u64) -> u64 {
  return bytes / 1024;
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

pub fn parse_time(reftime: u64) -> String {
  let time: u64 = reftime / 1000;
  let mut time_str: String = String::new();
  let mut time_vec: Vec<u64> = vec![];
  let unit_vec = [
    String::from("y"),
    String::from("w"),
    String::from("d"),
    String::from("h"),
    String::from("m"),
    String::from("s"),
  ];

  time_vec.push(time / 31536000);
  time_vec.push((time / 604800) % 52);
  time_vec.push((time / 86400) % 7);
  time_vec.push((time / 3600) % 24);
  time_vec.push((time / 60) % 60);
  time_vec.push(time % 60);

  for i in 0..time_vec.len() {
    if time_vec[i] != 0 {
      time_str += &format!("{}{} ", time_vec[i], unit_vec[i]);
    }
  }
  return time_str;
}

/// Returns the speed in megabytes per second
/// # Arguments
/// * `number` - The number to convert
/// * `speed` - The speed multiplier of the number
pub fn parse_speed(number: f32, speed: &str) -> f32 {
  match speed {
    "bps" => return number / 1000000f32,
    "Kbps" => return number / 1000f32,
    "Mbps" => return number,
    "Gbps" => return number * 1000f32,
    "Tbps" => return number * 1000000f32,
    _ => return number,
  }
}
