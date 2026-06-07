mod utils;
mod lessons;

use colored::*;
use std::io::{self, Write};

fn main() {
    loop {
        utils::clear_screen();
        println!("{}", "╔══════════════════════════════════╗".blue());
        println!("{}", "║   Rust Learning CLI Tutor        ║".blue().bold());
        println!("{}", "╚══════════════════════════════════╝".blue());
        println!();
        println!("1. {}", "Learn Rust (Basics)".yellow());
        println!("2. {}", "Learn Rust (Advanced)".yellow());
        println!("3. {}", "API Concepts".yellow());
        println!("4. {}", "Building APIs in Rust".yellow());
        println!("5. {}", "API Integration in Rust".yellow());
        println!("6. {}", "Algorithmic Trading with Rust".yellow());
        println!("7. {}", "Exit".red());
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => lessons::rust_basics::run(),
            "2" => lessons::rust_advanced::run(),
            "3" => lessons::api_concepts::run(),
            "4" => lessons::api_building::run(),
            "5" => lessons::api_integration::run(),
            "6" => lessons::algo_trading::run(),
            "7" => {
                println!("\nGoodbye! Keep learning 🦀");
                break;
            }
            _ => {
                println!("{}", "Invalid choice. Press Enter to continue.".red());
                utils::pause();
            }
        }
    }
}