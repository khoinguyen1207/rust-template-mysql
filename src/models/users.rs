use chrono::{NaiveDateTime, Utc};
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
#[serde(rename_all = "snake_case")]
pub struct User {
    // #[sqlx(rename = "userId")]
    pub user_id: Option<i32>,
    pub username: String,
    pub amount: Option<Decimal>,

    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub fn new(username: String) -> User {
        User {
            user_id: None,
            username: username.to_string(),
            amount: None,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        }
    }
}
