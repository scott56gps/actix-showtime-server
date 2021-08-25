use actix_web::{HttpResponse, Responder, web, http, error::InternalError, HttpServer, App};
use web::{Json, Data, post};
use std::env;

mod db;
mod model;
use crate::db::DB;
use crate::model::Movie;

type StdErr = Box<dyn std::error::Error>;

#[actix_web::get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hola from Actix!")
}

async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::get("/watchlist")]
async fn get_watchlist(db: Data<DB>) -> Result<Json<Vec<Movie>>, InternalError<StdErr>> {
    db.movies()
        .await
        .map(Json)
        .map_err(to_internal_error)
}

#[actix_web::post("/watchlist")]
async fn post_watchlist(db: Data<DB>, movie: Json<Movie>) -> Result<Json<Movie>, InternalError<StdErr>> {
    println!("movie: {:#?}", movie);
    db.create_movie(movie)
        .await
        .map(Json)
        .map_err(to_internal_error)
}

// Private Functions
fn to_internal_error(e: StdErr) -> InternalError<StdErr> {
    println!("Received Error");
    InternalError::new(e, http::StatusCode::INTERNAL_SERVER_ERROR)
}

#[actix_web::main]
async fn main() -> Result<(), StdErr> {
    // dotenv::dotenv()?;

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    // Initialize the database connection
    let db = DB::connect().await?;

    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .service(hello)
            .service(get_watchlist)
            .service(post_watchlist)
            .route("/echo", post().to(echo))
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await?;

    Ok(())
}
