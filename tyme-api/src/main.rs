use std::env;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{Context, Result};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;

mod api;
mod repository;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };

    HttpResponse::Ok().json(response)
}

async fn not_found() -> actix_web::Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };

    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> Result<()> {
    let pool = PgPoolOptions::new()
        .connect("postgres://ericshen118:3iSWKAPVY9mG@ep-red-morning-327906.us-east-2.aws.neon.tech/reminders")
        .await?;

    let app_data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("0.0.0.0", 7057))?
    .run()
    .await?;

    Ok(())
}
