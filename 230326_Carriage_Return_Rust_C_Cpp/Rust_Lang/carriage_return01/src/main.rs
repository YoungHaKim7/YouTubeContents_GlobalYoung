use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let chars: [char; 4] = ['-', '\\', '|', '/'];
    let mut i = 0;

    loop {
        print!("{}\r", chars[i]);
        io::stdout().flush().unwrap(); // flush stdout to immediately show the character
        thread::sleep(Duration::from_millis(200)); // wait for 200 milliseconds before printing the next character
        i = (i + 1) % chars.len();
    }
}
