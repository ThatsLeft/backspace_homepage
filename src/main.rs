use actix_cors::Cors;
use actix_files::Files;
use actix_web::{get, middleware::{self}, web::{self, ServiceConfig}, HttpResponse, Result};
use shuttle_actix_web::ShuttleActixWeb;
use tracing::info;

use crate::handlers::home_page::homepage;

#[get("/ping")]
/// Health check endpoint
async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status": "healthy", "service": "backspace-homepage"}"#))
}

mod handlers;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    info!("Starting Backspace AS Homepage server!");
    
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .service(Files::new("/static", "./static"))
                .service(health_check)
                .service(homepage)

                .wrap(Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "OPTIONS"])  // Add OPTIONS
                    .allowed_headers(vec!["Content-Type", "Authorization", "X-Requested-With", "Accept", "Cookie"])  // Add more headers
                    .expose_headers(vec!["content-type", "content-length", "set-cookie"])  // Add exposed headers
                    .max_age(3600)
                )
                .wrap(middleware::NormalizePath::trim())
                .wrap(middleware::Logger::default()),
        );
    };

    Ok(config.into())
}
