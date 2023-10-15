use reqwest::Client;

use crate::prelude::leak;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    // pub pool: Pool,
}

impl AppState {
    pub async fn new() -> &'static Self {
        // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        // let pool = sqlx::postgres::PgPool::connect(&database_url)
        //     .await
        //     .expect("Failed to connect to database");

        let client = Client::new();

        leak(Self { client, /* pool */ })
    }
}
