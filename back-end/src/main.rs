#[macro_use]
extern crate bson;

#[macro_use]
extern crate derive_more;

#[macro_use]
extern crate serde;

use std::env;

use actix_cors::Cors;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::{web as aweb, App, Error, HttpServer};
use anyhow::{anyhow, Context, Result};

use tracing::Span;
use tracing_actix_web::{DefaultRootSpanBuilder, RootSpanBuilder, TracingLogger};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use crate::web::services::endpoints::clean_up_devices;

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

/// We will define a custom root span builder to capture additional fields, specific
/// to our application, on top of the ones provided by `DefaultRootSpanBuilder` out of the box.
pub struct CustomRootSpanBuilder;

impl RootSpanBuilder for CustomRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        // Not sure why you'd be keen to capture this, but it's an example and we try to keep it simple
        let n_headers = request.headers().len();
        // We set `cloud_provider` to a constant value.
        //
        // `name` is not known at this point - we delegate the responsibility to populate it
        // to the `personal_hello` handler. We MUST declare the field though, otherwise
        // `span.record("caller_name", XXX)` will just be silently ignored by `tracing`.
        tracing_actix_web::root_span!(
            request,
            n_headers,
            cloud_provider = "localhost",
            caller_name = tracing::field::Empty
        )
    }

    fn on_request_end<B: MessageBody>(span: Span, outcome: &Result<ServiceResponse<B>, Error>) {
        // Capture the standard fields when the request finishes.
        DefaultRootSpanBuilder::on_request_end(span, outcome);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let database_url = match required_env_var("DATABASE_URL") {
        Ok(url) => url,
        Err(e) => return Err(e),
    };

    let db_data = aweb::Data::new(MongoDB::new(&database_url).await?);

    let db_clone = db_data.clone();

    std::env::set_var("RUST_LOG", "info,actix_web=info");

    // info
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::try_new("trace").unwrap())
        .add_directive("mongodb::connection=trace".parse().unwrap())
        .add_directive("actix_web=trace".parse().unwrap())
        .add_directive("actix_server=trace".parse().unwrap());
    let formatting_layer = BunyanFormattingLayer::new("rpillpal-backend".into(), std::io::stdout);

    let subscriber = Registry::default()
        .with(filter)
        .with(tracing_subscriber::fmt::Layer::default())
        .with(JsonStorageLayer)
        .with(formatting_layer);

    LogTracer::init().expect("Failed to set logger");

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let web_server = HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .wrap(TracingLogger::<CustomRootSpanBuilder>::new())
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin() // Allows requests from any origin
                    .allow_any_method() // Allows any method (GET, POST, etc.)
                    .allow_any_header() // Allows any header
                    .supports_credentials(), // Supports credentials (cookies, authorization headers, etc.)
            )
            // Logger middleware
            .service(web::services::endpoints::fetch)
            .service(web::services::endpoints::fetch_user)
            .service(web::services::endpoints::pill_data)
            .service(web::services::endpoints::update)
            .service(web::services::endpoints::update_pills)
            .service(web::services::endpoints::post_devices)
            .service(web::services::endpoints::get_devices)
            // .service(web::services::fetch::fetch_user)
            .default_service(aweb::to(web::services::not_found::not_found))
    })
    .bind("0.0.0.0:5000")
    .unwrap_or_else(|_| panic!("Could not bind to http://{}", "0.0.0.0:5000"))
    // Start the server running.
    .run();

    // spawn a task to clean up devices
    // tokio::spawn(clean_up_devices(db_clone));
    tokio::spawn(async move {
        clean_up_devices(db_clone).await;
    });

    tracing::info!("Server running at http://0.0.0.0:5000");

    // Wait on server to produce an error.
    return Ok(web_server.await?);
}
