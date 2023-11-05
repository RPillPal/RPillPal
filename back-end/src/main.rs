#[macro_use]
extern crate bson;

#[macro_use]
extern crate derive_more;

#[macro_use]
extern crate serde;

use std::env;

use actix_cors::Cors;
use anyhow::{anyhow, Context, Result};

use actix_web::{middleware, web as aweb, App, HttpServer};

mod db;
mod error;
mod web;

use db::MongoDB;

/// Get an environment variable, returning an Err with a
/// nice error message mentioning the missing variable in case the value is not found.
pub fn required_env_var(key: &str) -> Result<String> {
    env::var(key).with_context(|| format!("Missing environment variable: {key}"))
}

/// like [`required_env_var`], but also uses `FromStr` to parse the value.
pub fn parse_required_env_var<E: Into<anyhow::Error>, T: std::str::FromStr<Err = E>>(
    key: &str,
) -> Result<T> {
    required_env_var(key)?
        .parse()
        .map_err(|e: E| anyhow!(e))
        .with_context(|| format!("Failed to parse env-var {key}"))
}

#[tokio::main]
async fn main() -> Result<()> {
    let database_url = match required_env_var("DATABASE_URL") {
        Ok(url) => url,
        Err(e) => return Err(e),
    };
    let database_port = match parse_required_env_var("DATABASE_PORT") {
        Ok(port) => port,
        Err(e) => return Err(e),
    };

    let db_data = aweb::Data::new(MongoDB::new(&database_url, database_port).await?);

    let web_server = HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin() // Allows requests from any origin
                    .allow_any_method() // Allows any method (GET, POST, etc.)
                    .allow_any_header() // Allows any header
                    .supports_credentials(), // Supports credentials (cookies, authorization headers, etc.)
            )
            // Logger middleware
            .wrap(middleware::Logger::default())
            .route("/fetch", aweb::post().to(web::services::fetch::fetch))
            .default_service(aweb::to(web::services::not_found::not_found))
    })
    // Bind to 80 (this gets reversed proxied by Caddy later)
    .bind("127.0.0.1:5000")
    .unwrap_or_else(|_| panic!("Could not bind to http://{}", "127.0.0.1:5000"))
    // Start the server running.
    .run();

    println!("Server running at http://127.0.0.1:5000");

    // Wait on server to produce an error.
    return Ok(web_server.await?);
}