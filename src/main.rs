use actix_web::{HttpResponse, Responder, web, HttpServer, App};

mod db;
mod model;

type StdErr = Box<dyn std::error::Error>;

#[actix_web::get("/")]
async fn hello(db: web::Data<db::DB>) -> impl Responder {
    HttpResponse::Ok().body("Hola from Actix!")
}

async fn echo(db: web::Data<db::DB>, req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
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
            .route("/echo", web::post().to(echo))
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await?;

    Ok(())
}
