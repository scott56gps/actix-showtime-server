use actix_web::{HttpResponse, Responder, web, HttpServer, App};
use web::{post};
use std::env;

mod db;
mod model;
mod routes;
use crate::db::DB;

type StdErr = Box<dyn std::error::Error>;

#[actix_web::get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hola from Actix!")
}

async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> Result<(), StdErr> {
    // dotenv::dotenv()?; // Uncomment for local dev environment vars

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("PORT must be a number");

    // Initialize the database connection
    let db = DB::connect().await?;

    println!("Now listening on port {}...", port);

    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .service(hello)
            .route("/echo", post().to(echo))
            .service(routes::api())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
