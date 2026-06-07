use colored::*;
use crate::utils;

pub fn run() {
    utils::clear_screen();
    println!("{}", "=== Advanced Rust ===".bold().underline());

    // Traits
    println!("{}", "1. Traits".bold());
    println!("Traits define shared behaviour (like interfaces).");
    println!("  trait Summary {{");
    println!("      fn summarize(&self) -> String;");
    println!("  }}");
    println!("  impl Summary for NewsArticle {{ ... }}");
    println!("Traits can have default implementations. Use 'impl Trait' or generics bounded by traits.");
    utils::pause();

    // Generics
    println!("\n{}", "2. Generics".bold());
    println!("Write code that works for multiple types without duplication.");
    println!("  fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {{ ... }}");
    println!("Also used with structs and enums.");
    utils::pause();

    // Lifetimes
    println!("\n{}", "3. Lifetimes".bold());
    println!("Lifetimes ensure references are valid as long as needed.");
    println!("  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {{");
    println!("      if x.len() > y.len() {{ x }} else {{ y }}");
    println!("  }}");
    println!("The compiler infers lifetimes in many cases; explicit annotations prevent dangling references.");
    utils::pause();

    // Closures and iterators
    println!("\n{}", "4. Closures & Iterators".bold());
    println!("Closures are anonymous functions that can capture their environment.");
    println!("  let add_one = |x| x + 1;");
    println!("  let numbers = vec![1, 2, 3];");
    println!("  let doubled: Vec<_> = numbers.iter().map(|n| n * 2).collect();");
    utils::pause();

    // Smart pointers
    println!("\n{}", "5. Smart Pointers".bold());
    println!("Box<T> for heap allocation; Rc<T> for reference counting; RefCell<T> for interior mutability.");
    println!("  let b = Box::new(5); // stored on heap");
    println!("  let a = Rc::new(Cons(5, Rc::new(Nil))); // shared ownership");
    utils::pause();

    // Concurrency
    println!("\n{}", "6. Concurrency".bold());
    println!("Rust's ownership prevents data races at compile time.");
    println!("Threads: std::thread::spawn(|| {{ ... }});");
    println!("Message passing: mpsc::channel();");
    println!("Shared state: Arc<Mutex<T>>.");
    utils::pause();

    // Async
    println!("\n{}", "7. Async/Await".bold());
    println!("For non-blocking I/O. Uses futures and tokio runtime.");
    println!("  async fn fetch_data() -> Result<String, Error> {{");
    println!("      let resp = reqwest::get(\"https://...\").await?;");
    println!("      resp.text().await");
    println!("  }}");
    utils::pause();

    // Macros
    println!("\n{}", "8. Macros".bold());
    println!("Metaprogramming: write code that writes code.");
    println!("Declarative macros: macro_rules! (e.g., vec![])");
    println!("Procedural macros: #[derive(...)], attribute macros, function-like macros.");
    utils::pause();

    println!("\n{}", "Advanced topics mastered!".green().bold());
    utils::pause();
}