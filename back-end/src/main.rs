#[macro_use]
extern crate bson;

#[macro_use]
extern crate derive_more;

#[macro_use]
extern crate serde;

use std::env;

use actix_web::{web as aweb, App, HttpServer};
use anyhow::Result;

use tracing::{Instrument, Level};
use tracing_actix_web::TracingLogger;

mod db;
mod error;
mod logging;
mod util;
mod web;

use crate::db::MongoDB;
use crate::logging::{init_cpu_logging, init_tracing, send_honeycomb_deploy_marker};
use crate::util::required_env_var;
use crate::web::services::endpoints::clean_up_devices;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// We will define a custom root span builder to capture additional fields, specific
/// to our application, on top of the ones provided by `DefaultRootSpanBuilder` out of the box.
pub struct CustomRootSpanBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let honeycomb_api_key = env::var("HONEYCOMB_API_KEY").ok();
    match init_tracing(honeycomb_api_key.clone()) {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    }
    if let Some(honeycomb_api_key) = honeycomb_api_key {
        send_honeycomb_deploy_marker(&honeycomb_api_key).await;
    }

    let span = tracing::span!(Level::DEBUG, "main");
    let _enter = span.enter();

    if env::var("LOG_CPU_USAGE") == Ok("1".to_string()) {
        init_cpu_logging().await;
    }

    let database_url = match required_env_var("DATABASE_URL") {
        Ok(url) => url,
        Err(e) => return Err(e),
    };

    let span = tracing::span!(Level::DEBUG, "main");
    let _enter = span.enter();

    let db_data = aweb::Data::new(MongoDB::new(&database_url).await?);
    let db_clone = db_data.clone(); // Clone is for the clean_up_devices task.
                                    // Create the server.
    let web_server = HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .wrap(TracingLogger::default())
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(web::services::endpoints::fetch)
            .service(web::services::endpoints::fetch_user)
            .service(web::services::endpoints::pill_data)
            .service(web::services::endpoints::update)
            .service(web::services::endpoints::update_pills)
            .service(web::services::endpoints::post_devices)
            .service(web::services::endpoints::get_devices)
            .default_service(aweb::to(web::services::not_found::not_found))
    })
    .bind("0.0.0.0:5000")
    .unwrap_or_else(|_| panic!("Could not bind to http://{}", "0.0.0.0:5000"))
    // Start the server running.
    .run();

    // Spawn a task to clean up devices every 30s if no heartbeat is received.
    tokio::spawn(
        async move {
            clean_up_devices(db_clone).await;
        }
        .instrument(tracing::info_span!("clean_up_devices")),
    );

    tracing::info!("Server running at http://0.0.0.0:5000");

    // Wait on server to produce an error.
    return Ok(web_server.await?);
}
