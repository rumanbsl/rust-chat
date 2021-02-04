use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct Db {
  db: Pool<Postgres>,
}

impl Db {
  pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
    let db = PgPoolOptions::new()
      .max_connections(5)
      .connect("postgres://localhost/chat_app")
      .await?;
    Ok(Db { db })
  }
}
