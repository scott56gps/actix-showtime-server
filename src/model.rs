#[derive(serde::Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
    pub id: Option<i32>,
    pub title: String,
    pub poster_url: String,
}
