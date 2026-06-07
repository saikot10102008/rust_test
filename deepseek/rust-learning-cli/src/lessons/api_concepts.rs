use colored::*;
use crate::utils;

pub fn run() {
    utils::clear_screen();
    println!("{}", "=== API Concepts ===".bold().underline());

    println!("{}", "1. What is an API?".bold());
    println!("API = Application Programming Interface.");
    println!("It allows two software components to communicate.");
    println!("Examples: a weather app fetching data from a server, your IDE using a code formatter, a payment gateway.");
    utils::pause();

    println!("\n{}", "2. RESTful APIs".bold());
    println!("REST (Representational State Transfer) is a common architectural style for web APIs.");
    println!("Key ideas:");
    println!("  - Resources identified by URLs (e.g., /users/123)");
    println!("  - Stateless communication");
    println!("  - Standard HTTP methods: GET, POST, PUT, DELETE");
    utils::pause();

    println!("\n{}", "3. HTTP Basics".bold());
    println!("GET    - Retrieve data (read)");
    println!("POST   - Create a new resource");
    println!("PUT    - Update/replace a resource");
    println!("DELETE - Remove a resource");
    println!("Responses include a status code (200 OK, 201 Created, 404 Not Found, 500 Internal Server Error).");
    utils::pause();

    println!("\n{}", "4. JSON (JavaScript Object Notation)".bold());
    println!("Most APIs exchange data in JSON format.");
    println!("Example:");
    println!("  {{\"id\": 1, \"name\": \"Alice\", \"email\": \"alice@example.com\"}}");
    println!("Rust uses serde to serialize/deserialize JSON.");
    utils::pause();

    println!("\n{}", "5. Authentication".bold());
    println!("APIs often require authentication to ensure security.");
    println!("Common methods:");
    println!("  - API keys (a simple token)");
    println!("  - Bearer tokens (OAuth 2.0)");
    println!("  - Basic Auth (base64 encoded username:password)");
    println!("Always use HTTPS in production.");
    utils::pause();

    println!("\n{}", "6. Rate Limiting & Pagination".bold());
    println!("APIs limit requests per time window to prevent abuse.");
    println!("Pagination splits large result sets (e.g., ?page=2&per_page=50).");
    utils::pause();

    println!("\n{}", "API concepts clear! Ready to build and integrate.".green().bold());
    utils::pause();
}