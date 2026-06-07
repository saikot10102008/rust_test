use colored::*;
use crate::utils;

pub fn run() {
    utils::clear_screen();
    println!("{}", "=== API Integration in Rust ===".bold().underline());

    println!("{}", "1. The reqwest crate".bold());
    println!("reqwest is the go-to HTTP client for Rust.");
    println!("Add to Cargo.toml: reqwest = {{ version = \"0.11\", features = [\"json\"] }}");
    println!("tokio for async runtime.");
    utils::pause();

    println!("\n{}", "2. Making a GET request".bold());
    println!("  let resp = reqwest::get(\"https://api.example.com/data\").await?;");
    println!("  let body = resp.text().await?;");
    println!("  // or parse JSON directly:");
    println!("  let data: MyStruct = resp.json().await?;");
    utils::pause();

    println!("\n{}", "3. POST request with JSON body".bold());
    println!("  let client = reqwest::Client::new();");
    println!("  let new_user = serde_json::json!( {{\"name\": \"Bob\"}} );");
    println!("  let resp = client.post(\"https://api.example.com/users\")");
    println!("      .json(&new_user)");
    println!("      .send().await?;");
    utils::pause();

    println!("\n{}", "4. Handling authentication".bold());
    println!("Add headers:");
    println!("  let resp = client.get(url)");
    println!("      .header(\"Authorization\", \"Bearer YOUR_TOKEN\")");
    println!("      .send().await?;");
    utils::pause();

    println!("\n{}", "5. Error handling & deserialization".bold());
    println!("Use serde to deserialize responses. Handle network errors, non‑200 status, invalid JSON.");
    println!("  match resp.status() {{");
    println!("      reqwest::StatusCode::OK => {{ ... }},");
    println!("      _ => {{ ... }},");
    println!("  }}");
    utils::pause();

    println!("\n{}", "6. Polling & WebSockets".bold());
    println!("For real‑time data consider WebSocket connections (tokio‑tungstenite) or long‑polling.");
    utils::pause();

    println!("\n{}", "You're now an API integration pro!".green().bold());
    utils::pause();
}