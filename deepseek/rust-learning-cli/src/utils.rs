use std::io::{self, Write};

pub fn pause() {
    let mut input = String::new();
    print!("\n{}", "Press Enter to continue...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
}

pub fn clear_screen() {
    // Print enough newlines to "clear" the terminal visually
    print!("{}[2J", 27 as char);
}