use clap::{Parser, Subcommand};
use colored::Colorize;
use reqwest::Client;
use serde_json::Value;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "Rust Mastery CLI")]
#[command(about = "Comprehensive interactive tutor for Rust, APIs, and Algo Trading", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Rust,
    ApiConcepts,
    ApiIntegration,
    ApiBuilding,
    AlgoTrading,
    LiveDemo,
    Interactive,
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

fn print_header(title: &str) {
    println!("\n{}", "═".repeat(70).bright_cyan());
    println!(" {}", title.bold().bright_yellow());
    println!("{}", "═".repeat(70).bright_cyan());
}

fn print_section(title: &str, content: &str) {
    println!("\n{}", format!("▶ {}", title).bold().bright_green());
    println!("{}", content);
}

fn print_code(title: &str, code: &str) {
    println!("\n{}", format!("💻 {}:", title).bright_blue());
    println!("{}", code.bright_black());
}

fn pause() {
    print!("\n{}", "[ Press Enter to continue... ]".bright_black());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}

// ============================================================================
// LESSON: RUST (BASIC TO ADVANCED)
// ============================================================================

fn lesson_rust() {
    print_header("RUST: FROM ABSOLUTE BEGINNER TO ADVANCED");
    
    print_section("1. The Absolute Basics", 
        "Rust is a statically typed, compiled language. \n\
        - `let x = 5;` creates an immutable variable.\n\
        - `let mut y = 10;` creates a mutable variable.\n\
        - Types are inferred, but can be explicit: `let z: f64 = 3.14;`\n\
        - Functions use `fn`, and the last expression without a `;` is the return value.");

    pause();

    print_section("2. Ownership & Borrowing (The Heart of Rust)", 
        "Rust guarantees memory safety without a Garbage Collector via three rules:\n\
        1. Each value in Rust has a variable that’s called its owner.\n\
        2. There can only be one owner at a time.\n\
        3. When the owner goes out of scope, the value will be dropped (freed).\n\n\
        Borrowing (`&`) lets you reference data without taking ownership. \n\
        Rule: You can have EITHER one mutable reference (`&mut`) OR any number of immutable references (`&`), but never both at the same time.");

    print_code("Ownership in Action", 
r#"fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED to s2. s1 is now invalid!
    // println!("{}", s1); // ❌ Compile Error: value borrowed here after move

    let s3 = String::from("world");
    let len = calculate_length(&s3); // ✅ We BORROW s3. s3 is still valid.
    println!("Length of {} is {}", s3, len);
}

fn calculate_length(s: &String) -> usize {
    s.len() // No semicolon = implicit return
}"#);

    pause();

    print_section("3. Enums & Pattern Matching (Algebraic Data Types)", 
        "Rust Enums can hold data, making them incredibly powerful. Combined with `match`, they replace complex `if/else` chains and prevent unhandled states.");

    print_code("Enums and Match", 
r#"enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Move to {}, {}", x, y),
        Message::Write(text) => println!("Text: {}", text),
    }
}"#);

    pause();

    print_section("4. Advanced: Lifetimes, Generics, and Traits", 
        "- Generics: Write code that works for multiple types (`fn largest<T: PartialOrd>(list: &[T]) -> &T`).\n\
        - Traits: Define shared behavior (like interfaces). E.g., `std::fmt::Display`.\n\
        - Lifetimes: Annotations (`'a`) that tell the compiler how long references are valid, preventing dangling pointers. The compiler usually infers these (Lifetime Elision), but you must specify them when returning references from functions.");

    pause();

    print_section("5. Advanced: Async Rust & Concurrency", 
        "Rust uses `async`/`.await` for non-blocking I/O. Unlike JS or Python, Rust's async code does nothing until it is executed by a runtime (like `Tokio`).\n\
        - `tokio::spawn` creates lightweight green threads (tasks).\n\
        - Channels (`mpsc`) are used for safe message passing between tasks without shared mutable state (fearless concurrency).");
}

// ============================================================================
// LESSON: API CONCEPTS
// ============================================================================

fn lesson_api_concepts() {
    print_header("API CONCEPTS: THE COMPLETE GUIDE");

    print_section("1. What is an API?", 
        "Application Programming Interface. It is a formal contract defining how software components interact. Think of it as a waiter in a restaurant: you (the Client) give an order to the waiter (the API), who takes it to the kitchen (the Server) and brings back your food (the Response).");

    pause();

    print_section("2. Core HTTP Mechanics", 
        "- Method: GET (retrieve), POST (create), PUT (replace), PATCH (update), DELETE (remove).\n\
        - Endpoint: The URL path (e.g., `/api/v1/orders`).\n\
        - Headers: Metadata (e.g., `Content-Type: application/json`, `Authorization: Bearer <token>`).\n\
        - Status Codes: 2xx (Success), 3xx (Redirect), 4xx (Client Error, e.g., 401 Unauthorized, 429 Rate Limited), 5xx (Server Error).");

    pause();

    print_section("3. REST vs. WebSockets", 
        "- REST: Request-Response model. Good for fetching historical data or placing a single order. Stateless.\n\
        - WebSockets: Persistent, full-duplex connection. The server can push data to the client instantly. CRITICAL for algorithmic trading to receive live order book updates and trades without polling.");

    pause();

    print_section("4. Professional API Concerns", 
        "- Authentication: API Keys (simple), OAuth 2.0 (user delegation), HMAC-SHA256 (signing requests with a secret key, standard for crypto exchanges).\n\
        - Rate Limiting: Servers restrict requests per second (e.g., 10 req/sec). Exceeding this results in a 429 status code.\n\
        - Idempotency: Ensuring that making the same request multiple times has the same effect as making it once (crucial for preventing duplicate trades). Achieved via `Idempotency-Key` headers.");
}

// ============================================================================
// LESSON: API INTEGRATION & BUILDING
// ============================================================================

fn lesson_api_integration() {
    print_header("API INTEGRATION & BUILDING IN RUST");

    print_section("1. Consuming APIs (The Right Way)", 
        "Never use `unwrap()` in production API code. Use proper error handling. The `reqwest` crate paired with `serde` is the industry standard.");

    print_code("Production-Grade API Client", 
r#"use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Ticker {
    symbol: String,
    price: f64,
}

async fn fetch_price(client: &Client, symbol: &str) -> Result<Ticker> {
    // 1. Build request with timeout and headers
    let response = client.get(format!("https://api.example.com/ticker/{}", symbol))
        .header("Accept", "application/json")
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await?; // ? propagates network errors

    // 2. Check for HTTP errors (e.g., 404, 500)
    response.error_for_status_ref()?;

    // 3. Parse JSON
    let ticker: Ticker = response.json().await?;
    Ok(ticker)
}"#);

    pause();

    print_section("2. Building APIs in Rust (Server-side)", 
        "To build an API, `Axum` (backed by Tokio and Hyper) is the modern standard. It is highly ergonomic and type-safe.");

    print_code("Minimal Axum Server", 
r#"use axum::{routing::get, Router, Json};
use serde::Serialize;

#[derive(Serialize)]
struct HealthCheck {
    status: String,
}

async fn health() -> Json<HealthCheck> {
    Json(HealthCheck { status: "OK".into() })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}"#);
}

// ============================================================================
// LESSON: ALGORITHMIC TRADING
// ============================================================================

fn lesson_algo_trading() {
    print_header("ALGORITHMIC TRADING WITH RUST");

    print_section("1. Why Rust Dominates High-Frequency & Algo Trading", 
        "1. Predictable Latency: No Garbage Collector pauses. Execution time is deterministic.\n\
        2. Memory Safety: A segfault in a C++ trading bot can halt trading. Rust prevents this at compile time.\n\
        3. Concurrency: Tokio handles thousands of concurrent WebSocket streams (e.g., listening to 500 crypto pairs simultaneously) with minimal overhead.");

    pause();

    print_section("2. Market Microstructure (You Must Know This)", 
        "- Order Book: A list of buy orders (Bids) and sell orders (Asks).\n\
        - Spread: The difference between the highest Bid and lowest Ask. Tighter spread = more liquidity.\n\
        - Market Order: Executes immediately at the best available price (guaranteed execution, not guaranteed price).\n\
        - Limit Order: Executes only at a specific price or better (guaranteed price, not guaranteed execution).\n\
        - Slippage: The difference between the expected price of a trade and the price at which the trade is actually executed.");

    pause();

    print_section("3. The 4 Pillars of a Trading Bot Architecture", 
        "1. Data Layer: Ingests REST (historical) and WebSocket (live) data. Normalizes it into a standard format.\n\
        2. Strategy Engine: Stateful logic. Calculates indicators (RSI, MACD, Order Flow) and emits `Signal` events.\n\
        3. Risk Management: The most important part. Checks position limits, max drawdown, and portfolio exposure BEFORE any order is sent.\n\
        4. Execution Gateway: Manages API keys, signs requests (HMAC), handles rate limits, and manages order state (Pending, Filled, Cancelled).");

    print_code("Event-Driven Bot Architecture (Rust)", 
r#"// Using an event-driven model prevents blocking and allows clean separation of concerns.

enum MarketEvent {
    Tick { symbol: String, price: f64, volume: f64 },
    OrderFilled { order_id: String, price: f64 },
}

enum Command {
    Buy { symbol: String, amount: f64 },
    Sell { symbol: String, amount: f64 },
}

struct TradingBot {
    risk_manager: RiskManager,
    // Channels to communicate between async tasks
    event_rx: tokio::sync::mpsc::Receiver<MarketEvent>,
    command_tx: tokio::sync::mpsc::Sender<Command>,
}

impl TradingBot {
    async fn run(&mut self) {
        while let Some(event) = self.event_rx.recv().await {
            match event {
                MarketEvent::Tick { symbol, price, .. } => {
                    if self.strategy_is_bullish(&symbol, price) {
                        if self.risk_manager.can_buy(&symbol) {
                            let _ = self.command_tx.send(Command::Buy { 
                                symbol, amount: 0.1 
                            }).await;
                        }
                    }
                }
                _ => {}
            }
        }
    }
}"#);
}

// ============================================================================
// LIVE DEMO
// ============================================================================

async fn run_live_demo() {
    print_header("LIVE ALGO TRADING DATA INGESTION DEMO");
    println!(" {}", "Fetching real-time Bitcoin data via REST API (CoinGecko)...".bright_yellow());
    
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap();
        
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd&include_24hr_change=true";

    print!(" {}", "Sending HTTP GET request...".bright_blue());
    io::stdout().flush().unwrap();

    match client.get(url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Value>().await {
                    Ok(data) => {
                        println!("\r {}", "✅ Success! JSON parsed.".bright_green());
                        let price = data["bitcoin"]["usd"].as_f64().unwrap_or(0.0);
                        let change = data["bitcoin"]["usd_24h_change"].as_f64().unwrap_or(0.0);
                        
                        println!("\n{}", "📊 MARKET DATA RECEIVED:".bold().bright_cyan());
                        println!("   Asset       : {}", "Bitcoin (BTC)".bold());
                        println!("   Price       : ${:.2}", price);
                        
                        let change_str = format!("{:.2}%", change);
                        if change >= 0.0 {
                            println!("   24h Change  : {}", change_str.bright_green());
                        } else {
                            println!("   24h Change  : {}", change_str.bright_red());
                        }
                        
                        println!("\n{}", "💡 STRATEGY EVALUATION:".bright_yellow());
                        if change < -2.0 {
                            println!("   Signal: {}", "Potential Mean Reversion Buy Opportunity".bright_green());
                        } else if change > 2.0 {
                            println!("   Signal: {}", "Strong Momentum / Caution for FOMO".bright_red());
                        } else {
                            println!("   Signal: {}", "Consolidation / No Clear Signal".bright_white());
                        }
                    }
                    Err(e) => println!("\r {}", format!("❌ Failed to parse JSON: {}", e).bright_red()),
                }
            } else {
                println!("\r {}", format!("❌ API HTTP Error: {}", response.status()).bright_red());
            }
        }
        Err(e) => println!("\r {}", format!("❌ Network Error: {}", e).bright_red()),
    }
}

// ============================================================================
// INTERACTIVE MODE
// ============================================================================

fn interactive_mode() {
    loop {
        print_header("INTERACTIVE LEARNING MENU");
        println!(" 1. Rust: Basic to Advanced Concepts");
        println!(" 2. API: Core Concepts & Architecture");
        println!(" 3. API: Integration & Building in Rust");
        println!(" 4. Algorithmic Trading: Concepts & Rust Architecture");
        println!(" 5. Live Demo: Real-time Market Data Ingestion");
        println!(" 6. Exit");
        
        print!("\n{} ", "Select an option (1-6):".bright_yellow());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => lesson_rust(),
            "2" => lesson_api_concepts(),
            "3" => lesson_api_integration(),
            "4" => lesson_algo_trading(),
            "5" => {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(run_live_demo());
            }
            "6" => {
                println!("\n{}", "🚀 Happy coding and happy trading!".bright_green());
                break;
            }
            _ => println!("\n{}", "Invalid option. Please choose 1-6.".bright_red()),
        }
    }
}

// ============================================================================
// MAIN
// ============================================================================

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // FIXED: Added the missing Some(Commands::ApiBuilding) arm
    match cli.command {
        Some(Commands::Rust) => lesson_rust(),
        Some(Commands::ApiConcepts) => lesson_api_concepts(),
        Some(Commands::ApiIntegration) => lesson_api_integration(),
        Some(Commands::ApiBuilding) => lesson_api_integration(), // Maps to the same comprehensive lesson
        Some(Commands::AlgoTrading) => lesson_algo_trading(),
        Some(Commands::LiveDemo) => run_live_demo().await,
        Some(Commands::Interactive) => interactive_mode(),
        None => interactive_mode(),
    }
}