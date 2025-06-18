use std::fs;

use actix_web::{get, HttpResponse, Result};
use tracing::info;

#[get("/")]
pub async fn homepage() -> Result<HttpResponse> {
    info!("Homepage requested");
    
    let html_content = fs::read_to_string("static/index.html")
        .map_err(|e| {
            tracing::error!("Failed to read index.html: {}", e);
            actix_web::error::ErrorInternalServerError("Template not found")
        })?;
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content))
}