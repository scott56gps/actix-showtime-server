#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
    pub id: Option<i32>,
    pub title: String,
    pub poster_url: String,
}
