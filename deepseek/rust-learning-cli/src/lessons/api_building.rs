use colored::*;
use crate::utils;

pub fn run() {
    utils::clear_screen();
    println!("{}", "=== Building APIs in Rust ===".bold().underline());

    println!("{}", "1. Choosing a web framework".bold());
    println!("Popular choices: Actix-web, Axum, Rocket, Warp.");
    println!("We'll illustrate using Axum (lightweight, built on Tokio).");
    utils::pause();

    println!("\n{}", "2. Basic Axum server".bold());
    println!("Example code (Cargo.toml: axum, tokio, serde):");
    println!("  use axum::{{routing::get, Router, response::Json}};");
    println!("  use serde::Serialize;");
    println!("  ");
    println!("  #[derive(Serialize)]");
    println!("  struct User {{ id: u32, name: String }}");
    println!("  ");
    println!("  async fn get_user() -> Json<User> {{");
    println!("      Json(User {{ id: 1, name: \"Alice\".into() }})");
    println!("  }}");
    println!("  ");
    println!("  #[tokio::main]");
    println!("  async fn main() {{");
    println!("      let app = Router::new().route(\"/user\", get(get_user));");
    println!("      axum::Server::bind(&\"0.0.0.0:3000\".parse().unwrap())");
    println!("          .serve(app.into_make_service())");
    println!("          .await.unwrap();");
    println!("  }}");
    utils::pause();

    println!("\n{}", "3. Handling different methods".bold());
    println!("Add POST, PUT, DELETE routes.");
    println!("  .route(\"/user\", post(create_user))");
    println!("  .route(\"/user/:id\", put(update_user).delete(delete_user))");
    println!("Extract path parameters and JSON bodies with extractors.");
    utils::pause();

    println!("\n{}", "4. Error handling & status codes".bold());
    println!("Return appropriate HTTP status codes using Result and custom error types.");
    println!("  async fn get_item(id: Path<u32>) -> Result<Json<Item>, StatusCode> {{");
    println!("      // lookup item...");
    println!("  }}");
    utils::pause();

    println!("\n{}", "5. Authentication middleware".bold());
    println!("Axum supports middleware: check auth headers before processing request.");
    println!("Use tower::ServiceBuilder to layer middleware.");
    println!("  let app = Router::new().route(\"/secure\", get(handler))");
    println!("      .layer(middleware::from_fn(auth_middleware));");
    utils::pause();

    println!("\n{}", "6. Database integration".bold());
    println!("Use sqlx or diesel for PostgreSQL/MySQL.");
    println!("Connection pooling with sqlx::PgPool and async queries.");
    utils::pause();

    println!("\n{}", "API building fundamentals covered!".green().bold());
    utils::pause();
}