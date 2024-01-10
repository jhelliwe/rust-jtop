use std::process;
use terminal_size::{terminal_size, Height, Width};

pub fn termsize() -> (usize, usize) {
    let mut session_width: usize = 0;
    let mut session_height: usize = 0;
    if let Some((Width(w), Height(h))) = terminal_size() {
        session_width = w as usize;
        session_height = h as usize;
    } else {
        eprintln!(
            "Unable to get terminal size {} {}",
            session_width, session_height
        );
        process::exit(1);
    }
    (session_width, session_height)
}
