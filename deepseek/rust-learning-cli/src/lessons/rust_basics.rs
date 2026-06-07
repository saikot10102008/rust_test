use colored::*;
use crate::utils;

pub fn run() {
    utils::clear_screen();
    println!("{}", "=== Rust Basics ===".bold().underline());
    println!("Welcome to the Rust basics course! You'll learn fundamental concepts step by step.\n");

    // Lesson 1: Variables and mutability
    println!("{}", "1. Variables & Mutability".bold());
    println!("In Rust, variables are immutable by default. Use 'let' to declare a variable.");
    println!("Example:");
    println!("  let x = 5;      // immutable");
    println!("  let mut y = 10; // mutable");
    println!("  y = 15;         // allowed because y is mutable");
    println!("  // x = 6;       // ERROR: cannot assign twice to immutable variable");
    utils::pause();

    // Lesson 2: Data types
    println!("\n{}", "2. Data Types".bold());
    println!("Rust is statically typed. Common scalar types:");
    println!("  Integers: i8, i16, i32, i64, u8, u32, etc.");
    println!("  Floats: f32, f64");
    println!("  Boolean: bool");
    println!("  Character: char");
    println!("  String: String / &str");
    println!("Example:");
    println!("  let age: u8 = 30;");
    println!("  let pi: f64 = 3.14159;");
    println!("  let name: &str = \"Alice\";");
    println!("  let owned_string: String = String::from(\"Hello\");");
    utils::pause();

    // Lesson 3: Functions
    println!("\n{}", "3. Functions".bold());
    println!("Define functions with 'fn'. Parameters must have type annotations.");
    println!("Return type specified after '->'.");
    println!("Example:");
    println!("  fn add(a: i32, b: i32) -> i32 {{");
    println!("      a + b  // last expression is returned (no semicolon)");
    println!("  }}");
    utils::pause();

    // Lesson 4: Control flow
    println!("\n{}", "4. Control Flow".bold());
    println!("if/else, loop, while, for:");
    println!("  let num = 7;");
    println!("  if num < 5 {{");
    println!("      println!(\"small\");");
    println!("  }} else if num == 5 {{");
    println!("      println!(\"exactly five\");");
    println!("  }} else {{");
    println!("      println!(\"large\");");
    println!("  }}");
    println!("  for i in 0..5 {{ println!(\"{{}}\", i); }}");
    utils::pause();

    // Lesson 5: Ownership
    println!("\n{}", "5. Ownership (the heart of Rust)".bold().red());
    println!("Three rules:");
    println!("1. Each value has a single owner.");
    println!("2. When the owner goes out of scope, the value is dropped.");
    println!("3. You can move ownership or borrow (reference) it.");
    println!("Example:");
    println!("  let s1 = String::from(\"hello\");");
    println!("  let s2 = s1;            // ownership moved, s1 is no longer valid");
    println!("  let s3 = s2.clone();   // deep copy (expensive)");
    println!("  let len = calculate_length(&s3); // borrow, s3 still valid");
    utils::pause();

    // Lesson 6: Borrowing and references
    println!("\n{}", "6. References & Borrowing".bold());
    println!("Use & to create a reference (borrow).");
    println!("  fn calculate_length(s: &String) -> usize {{ s.len() }}");
    println!("Mutable references: &mut");
    println!("  fn change(s: &mut String) {{ s.push_str(\" world\"); }}");
    println!("Rules: You can have either one mutable reference OR any number of immutable references, but not both at the same time.");
    utils::pause();

    // Lesson 7: Structs
    println!("\n{}", "7. Structs".bold());
    println!("Custom data types grouping related data.");
    println!("  struct User {{");
    println!("      username: String,");
    println!("      email: String,");
    println!("      sign_in_count: u64,");
    println!("      active: bool,");
    println!("  }}");
    println!("  let user1 = User {{ ... }};");
    utils::pause();

    // Lesson 8: Enums and pattern matching
    println!("\n{}", "8. Enums & Pattern Matching".bold());
    println!("Enums define a type that can be one of several variants.");
    println!("  enum IpAddrKind {{ V4, V6 }}");
    println!("Pattern matching with 'match':");
    println!("  match ip_kind {{");
    println!("      IpAddrKind::V4 => println!(\"IPv4\"),");
    println!("      IpAddrKind::V6 => println!(\"IPv6\"),");
    println!("  }}");
    utils::pause();

    // Lesson 9: Error handling
    println!("\n{}", "9. Error Handling".bold());
    println!("Recoverable errors: Result<T, E>");
    println!("  let f = File::open(\"hello.txt\");");
    println!("  match f {{");
    println!("      Ok(file) => file,");
    println!("      Err(error) => panic!(\"Problem opening file: {{:?}}\", error),");
    println!("  }}");
    println!("Unrecoverable: panic!()");
    println!("The '?' operator propagates errors.");
    utils::pause();

    println!("\n{}", "Congratulations! You've covered the Rust basics.".green().bold());
    println!("Returning to main menu...\n");
    utils::pause();
}