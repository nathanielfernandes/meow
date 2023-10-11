use serde::Deserialize;

// use crate::prelude::Transaction;

use super::{avatar::Avatar, sentiment::UserSentiment};

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub avatar: Avatar,
    pub sentiment: UserSentiment,
}

impl User {
    // pub async fn get_or_create(tx: &mut Transaction<'_>, id: u64) -> Result<Self, sqlx::Error> {
    //     let user = sqlx::query_as!(
    //         User,
    //         "
    //         INSERT INTO users (id)
    //         VALUES ($1)
    //         ON CONFLICT DO NOTHING
    //         ",
    //         id,
    //     )
    //     .execute(&mut *tx)
    //     .await?;

    //     Ok(user)
    // }
}
