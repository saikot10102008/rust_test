use clap::{Parser, Subcommand};
use colored::Colorize;
use reqwest::Client;
use serde_json::Value;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "Rust Mastery CLI")]
#[command(about = "Learn Rust, APIs, and Algorithmic Trading interactively", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Learn Rust from Basic to Advanced
    Rust,
    /// Learn what an API is and core concepts
    ApiConcepts,
    /// Learn how to build and integrate APIs in Rust
    ApiBuild,
    /// Learn Algorithmic Trading concepts and architecture in Rust
    AlgoTrading,
    /// Run a LIVE demo: Fetch real Bitcoin price via API
    LiveDemo,
    /// Start the interactive menu-driven learning mode
    Interactive,
}

// ============================================================================
// LESSON CONTENT
// ============================================================================

fn print_header(title: &str) {
    println!("\n{}", "=".repeat(60).bright_cyan());
    println!(" {}", title.bold().bright_yellow());
    println!("{}", "=".repeat(60).bright_cyan());
}

fn print_section(title: &str, content: &str) {
    println!("\n{}", title.bold().bright_green());
    println!("{}", content);
}

fn print_code(title: &str, code: &str) {
    println!("\n{}", format!("📝 {}:", title).bright_blue());
    println!("{}", code.bright_black());
}

fn lesson_rust() {
    print_header("RUST: BASIC TO ADVANCED");
    
    print_section("1. The Basics (Variables & Types)", 
        "Rust is statically typed. Use `let` for immutable variables and `let mut` for mutable ones.\n\
        Types are inferred, but you can annotate: `let x: i32 = 5;`");

    print_section("2. Ownership & Borrowing (The Core Concept)", 
        "Rust's superpower: Memory safety without a garbage collector.\n\
        - Each value has a single 'owner'.\n\
        - When the owner goes out of scope, the value is dropped.\n\
        - Borrowing (`&` for immutable, `&mut` for mutable) lets you reference data without taking ownership.\n\
        - Rule: You can have either ONE mutable reference OR any number of immutable references, but not both.");

    print_code("Ownership Example", 
r#"fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED to s2. s1 is no longer valid!
    // println!("{}", s1); // This would cause a compile error!
    
    let s3 = String::from("world");
    let len = calculate_length(&s3); // We BORROW s3. s3 is still valid here.
}

fn calculate_length(s: &String) -> usize {
    s.len()
}"#);

    print_section("3. Error Handling (Result & Option)", 
        "Rust has no exceptions. It uses `Result<T, E>` for recoverable errors and `Option<T>` for absence of value.\n\
        Use the `?` operator to propagate errors cleanly up the call stack.");

    print_section("4. Advanced: Traits & Async Rust", 
        "- Traits: Define shared behavior (like interfaces in other languages). E.g., `std::fmt::Display`.\n\
        - Async Rust: Use `async`/`.await` with a runtime like `Tokio` for non-blocking I/O (crucial for APIs and trading!).");
}

fn lesson_api_concepts() {
    print_header("API CONCEPTS");
    
    print_section("What is an API?", 
        "API stands for Application Programming Interface. It is a contract that allows two software programs to communicate with each other.");

    print_section("Core Concepts", 
        "- Client & Server: The client (your Rust app) requests data; the server (e.g., Binance, GitHub) provides it.\n\
        - REST: The most common API architecture. It uses standard HTTP methods.\n\
        - HTTP Methods: GET (read), POST (create), PUT/PATCH (update), DELETE (remove).\n\
        - Endpoints: The URL where the API lives (e.g., `https://api.example.com/v1/prices`).\n\
        - Status Codes: 200 (OK), 400 (Bad Request), 401 (Unauthorized), 404 (Not Found), 500 (Server Error).");

    print_section("Data Formats", 
        "JSON is the universal language of APIs. Rust uses the `serde` and `serde_json` crates to seamlessly convert between JSON text and Rust structs (Serialization/Deserialization).");
}

fn lesson_api_build() {
    print_header("BUILDING & INTEGRATING APIS IN RUST");

    print_section("1. Consuming an API (Integration)", 
        "Use the `reqwest` crate. It is the standard HTTP client in Rust. Always use it with `tokio` for async operations.");

    print_code("API Integration Example", 
r#"use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    id: u32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let response = client.get("https://api.example.com/users/1")
        .header("Authorization", "Bearer YOUR_API_KEY")
        .send()
        .await?;
        
    let user: User = response.json().await?;
    println!("User: {}", user.name);
    Ok(())
}"#);

    print_section("2. Building an API (Server)", 
        "To build an API in Rust, the industry standard is `Axum` (by the Tokio team) or `Actix-Web`.\n\
        They allow you to define routes, extract JSON payloads, and return typed responses with high performance and concurrency.");
}

fn lesson_algo_trading() {
    print_header("ALGORITHMIC TRADING USING RUST");

    print_section("Why Rust for Algo Trading?", 
        "1. Performance: Near C++ speeds, crucial for High-Frequency Trading (HFT).\n\
        2. Reliability: The borrow checker prevents data races and memory leaks, which could cost millions in live trading.\n\
        3. Concurrency: Tokio allows handling thousands of WebSocket streams simultaneously.");

    // FIXED: Removed broken quote concatenation, using clean line continuations
    print_section("The 4 Pillars of an Algo Trading Bot", 
        "1. Data Ingestion: Fetching historical data (REST) and live ticks (WebSockets).\n\
        2. Strategy Engine: Calculating indicators (SMA, RSI, MACD) and generating signals.\n\
        3. Execution Engine: Sending buy/sell orders to the exchange via authenticated API calls.\n\
        4. Risk Management: Position sizing, stop-losses, and max drawdown limits.");

    print_code("Algo Trading Architecture (Conceptual)", 
r#"struct TradingBot {
    client: ExchangeClient,
    strategy: MovingAverageCrossover,
    risk_manager: RiskManager,
}

impl TradingBot {
    async fn run(&mut self) {
        loop {
            // 1. Get live data
            let ticker = self.client.get_ticker("BTC-USD").await.unwrap();
            
            // 2. Evaluate strategy
            if self.strategy.should_buy(&ticker) {
                // 3. Check risk limits
                if self.risk_manager.is_safe_to_trade() {
                    // 4. Execute order
                    self.client.place_order(Order::Buy { amount: 0.1 }).await;
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}"#);
}

// ============================================================================
// LIVE DEMO
// ============================================================================

async fn run_live_demo() {
    print_header("LIVE ALGO TRADING DATA DEMO");
    println!(" {}", "Fetching real-time Bitcoin data from CoinGecko API (No API Key required)...".bright_yellow());
    
    let client = Client::new();
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd&include_24hr_change=true";

    print!(" {}", "Requesting...".bright_blue());
    io::stdout().flush().unwrap();

    match client.get(url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Value>().await {
                    Ok(data) => {
                        println!("\r {}", "Success!".bright_green());
                        let price = data["bitcoin"]["usd"].as_f64().unwrap_or(0.0);
                        let change = data["bitcoin"]["usd_24h_change"].as_f64().unwrap_or(0.0);
                        
                        println!("\n{}", "📊 MARKET DATA RECEIVED:".bold().bright_cyan());
                        println!("   Asset : {}", "Bitcoin (BTC)".bold());
                        println!("   Price : ${:.2}", price);
                        
                        let change_str = format!("{:.2}%", change);
                        if change >= 0.0 {
                            println!("   24h Change : {}", change_str.bright_green());
                        } else {
                            println!("   24h Change : {}", change_str.bright_red());
                        }
                        
                        // FIXED: Replaced invalid .italic() with .bright_black()
                        println!("\n{}", "💡 In a real trading bot, this data would now be passed to your Strategy Engine to evaluate buy/sell conditions.".bright_black());
                    }
                    Err(e) => println!("\r {}", format!("Failed to parse JSON: {}", e).bright_red()),
                }
            } else {
                println!("\r {}", format!("API Error: {}", response.status()).bright_red());
            }
        }
        Err(e) => println!("\r {}", format!("Network Error: {}. Are you offline?", e).bright_red()),
    }
}

// ============================================================================
// INTERACTIVE MODE
// ============================================================================

fn interactive_mode() {
    loop {
        print_header("INTERACTIVE LEARNING MENU");
        println!(" 1. Learn Rust (Basic to Advanced)");
        println!(" 2. Learn API Concepts");
        println!(" 3. Learn Building & Integrating APIs");
        println!(" 4. Learn Algorithmic Trading in Rust");
        println!(" 5. Run Live API Data Demo");
        println!(" 6. Exit");
        
        print!("\n{} ", "Select an option (1-6):".bright_yellow());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => lesson_rust(),
            "2" => lesson_api_concepts(),
            "3" => lesson_api_build(),
            "4" => lesson_algo_trading(),
            "5" => {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(run_live_demo());
            }
            "6" => {
                println!("\n{}", "Happy coding and happy trading! 🚀".bright_green());
                break;
            }
            _ => println!("\n{}", "Invalid option. Please choose 1-6.".bright_red()),
        }
        
        println!("\n{}", "Press Enter to continue...".bright_black());
        let mut dummy = String::new();
        let _ = io::stdin().read_line(&mut dummy);
    }
}

// ============================================================================
// MAIN
// ============================================================================

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Rust) => lesson_rust(),
        Some(Commands::ApiConcepts) => lesson_api_concepts(),
        Some(Commands::ApiBuild) => lesson_api_build(),
        Some(Commands::AlgoTrading) => lesson_algo_trading(),
        Some(Commands::LiveDemo) => run_live_demo().await,
        Some(Commands::Interactive) => interactive_mode(),
        None => interactive_mode(),
    }
}