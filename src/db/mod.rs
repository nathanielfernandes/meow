use moka::future::Cache;

use serde::Deserialize;
use sqlx::postgres::PgListener;

use crate::prelude::{leak, Pool};

use self::user::User;

pub mod avatar;
pub mod sentiment;
pub mod user;

pub struct Database {
    cache: Cache<u64, User>,
    pub pool: Pool,
}

impl Database {
    pub async fn new(pool: Pool, listener: PgListener) -> &'static Self {
        let cache = Cache::builder().build();
        let db = leak(Self { cache, pool });

        tokio::spawn(db.start_listening(listener));

        db
    }

    async fn start_listening(&'static self, mut listener: PgListener) {
        listener
            .listen("schema-db-changes")
            .await
            .expect("Failed to listen to database");

        while let Ok(notification) = listener.recv().await {
            let Ok(payload): Result<UserUpdate, _> = serde_json::from_str(&notification.payload())
            else {
                continue;
            };

            match payload.action {
                Action::INSERT | Action::UPDATE => {
                    self.cache.insert(payload.user.id, payload.user).await;
                }
                Action::DELETE => {
                    self.cache.remove(&payload.user.id).await;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct UserUpdate {
    #[serde(flatten)]
    pub user: User,
    #[allow(dead_code)]
    pub table: String,
    pub action: Action,
}

#[derive(Debug, Clone, Deserialize)]
enum Action {
    INSERT,
    UPDATE,
    DELETE,
}
