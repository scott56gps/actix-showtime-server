use actix_web::web::{Data, Json, Path};
use actix_web::http::StatusCode;
use actix_web::error::InternalError;
use actix_web::dev::HttpServiceFactory;
use actix_web::{HttpResponse};

use crate::StdErr;
use crate::db::DB;
use crate::model::*;

#[actix_web::get("/watchlist")]
async fn get_watchlist(db: Data<DB>) -> Result<Json<Vec<Movie>>, InternalError<StdErr>> {
    db.movies()
        .await
        .map(Json)
        .map_err(to_internal_error)
}

#[actix_web::post("/watchlist")]
async fn post_watchlist_movie(db: Data<DB>, movie: Json<Movie>) -> Result<Json<Movie>, InternalError<StdErr>> {
    println!("movie: {:#?}", movie);
    db.create_movie(movie)
        .await
        .map(Json)
        .map_err(to_internal_error)
}

#[actix_web::delete("/watchlist/{id}")]
async fn delete_watchlist_movie(db: Data<DB>, Path(id): Path<i32>) -> Result<HttpResponse, InternalError<StdErr>> {
    db.delete_movie(id)
	.await
	.map(to_ok)
	.map_err(to_internal_error)
}

// Private Functions
fn to_internal_error(e: StdErr) -> InternalError<StdErr> {
    println!("Received Error");
    InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
}

fn to_ok(_: ()) -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}

// Single public function to export all routes
pub fn api() -> impl HttpServiceFactory + 'static {
    actix_web::web::scope("/")
        .service(get_watchlist)
        .service(post_watchlist_movie)
        .service(delete_watchlist_movie)
}