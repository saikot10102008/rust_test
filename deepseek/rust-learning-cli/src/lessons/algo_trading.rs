use colored::*;
use crate::utils;

pub fn run() {
    utils::clear_screen();
    println!("{}", "=== Algorithmic Trading with Rust ===".bold().underline());

    println!("{}", "1. What is Algorithmic Trading?".bold());
    println!("Using computer programs to follow a defined set of instructions (an algorithm) to place trades.");
    println!("Goals: speed, consistency, backtesting, emotion‑free execution.");
    utils::pause();

    println!("\n{}", "2. Market Data & APIs".bold());
    println!("Data sources: Yahoo Finance, Alpha Vantage, IEX Cloud, broker APIs.");
    println!("Typical data: OHLCV (Open, High, Low, Close, Volume) candles.");
    println!("You'll fetch this data via REST or WebSocket.");
    utils::pause();

    println!("\n{}", "3. Building a simple strategy: Moving Average Crossover".bold());
    println!("When fast MA crosses above slow MA → BUY signal.");
    println!("When fast MA crosses below slow MA → SELL signal.");
    println!("Example implementation in Rust:");
    println!("  // Assume we have Vec<f64> closing prices");
    println!("  fn moving_average(data: &[f64], period: usize) -> Vec<f64> {{ ... }}");
    println!("  let fast_ma = moving_average(&closes, 10);");
    println!("  let slow_ma = moving_average(&closes, 30);");
    println!("  // Generate signals by comparing the two vectors");
    utils::pause();

    println!("\n{}", "4. Backtesting".bold());
    println!("Test your strategy on historical data before risking real money.");
    println!("Simulate trades and calculate P&L, win rate, Sharpe ratio, drawdown.");
    println!("Rust’s performance is ideal for large‑scale backtesting.");
    utils::pause();

    println!("\n{}", "5. Connecting to a Broker API".bold());
    println!("Brokers like Alpaca, Interactive Brokers, Binance provide REST/WebSocket APIs.");
    println!("You'll need API keys to place real orders.");
    println!("Use reqwest to send authenticated requests.");
    println!("Example (Alpaca paper trading):");
    println!("  let client = reqwest::Client::new();");
    println!("  let resp = client.post(\"https://paper-api.alpaca.markets/v2/orders\")");
    println!("      .json(&order)");
    println!("      .header(\"APCA-API-KEY-ID\", key)");
    println!("      .header(\"APCA-API-SECRET-KEY\", secret)");
    println!("      .send().await?;");
    utils::pause();

    println!("\n{}", "6. Risk Management".bold());
    println!("Always code safeguards: position sizing, stop‑loss orders, maximum drawdown limits.");
    println!("Never trade with money you can’t afford to lose, and always test thoroughly.");
    utils::pause();

    println!("\n{}", "Algorithmic trading journey begins now!".green().bold());
    println!("(Disclaimer: This is for educational purposes only.)");
    utils::pause();
}