use actix_web::{HttpResponse, Responder, web, http, error::InternalError, HttpServer, App};

mod db;
mod model;

type StdErr = Box<dyn std::error::Error>;

#[actix_web::get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hola from Actix!")
    // web::Json(model::Movie { id: Some(5), title: "Rush Hour 2".to_string(), poster_url: "bueno".to_string() })
}

#[actix_web::get("/watchlist")]
async fn get_watchlist(db: web::Data<db::DB>) -> Result<web::Json<Vec<model::Movie>>, InternalError<StdErr>> {
    db.movies()
        .await
        .map(web::Json)
        .map_err(to_internal_error)
}

async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// Private Functions
fn to_internal_error(e: StdErr) -> InternalError<StdErr> {
    println!("Received Error");
    InternalError::new(e, http::StatusCode::INTERNAL_SERVER_ERROR)
}

#[actix_web::main]
async fn main() -> Result<(), StdErr> {
    dotenv::dotenv()?;

    // Initialize the database connection
    let db = db::DB::connect().await?;

    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .service(hello)
            .service(get_watchlist)
            .route("/echo", web::post().to(echo))
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await?;

    Ok(())
}
