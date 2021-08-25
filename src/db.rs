use crate::web::Json;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use crate::model::Movie;
use crate::StdErr;

#[derive(Clone)]
pub struct DB {
    pool: Pool<Postgres>,
}

impl DB {
    pub async fn connect() -> Result<Self, StdErr> {
        let db_url = std::env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new().connect(&db_url).await?;
        Ok(DB { pool })
    }

    pub async fn movies(&self) -> Result<Vec<Movie>, StdErr> {
        let movies = sqlx::query_as("SELECT * FROM movies")
            .fetch_all(&self.pool)
            .await?;
        Ok(movies)
    }

    pub async fn create_movie(&self, movie: Json<Movie>) -> Result<Movie, StdErr> {
        let created_movie = sqlx::query_as("INSERT INTO movies (title, poster_url) VALUES ($1, $2) RETURNING *")
            .bind(&movie.title)
            .bind(&movie.poster_url)
            .fetch_one(&self.pool)
            .await?;
        Ok(created_movie)
    }
}
