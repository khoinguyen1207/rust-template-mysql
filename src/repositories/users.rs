use std::sync::Arc;
use sqlx::{Error, MySqlPool};

use crate::models::users::User;

#[derive(Clone, Debug)]
pub struct UserDao {
    db: Arc<MySqlPool>,
}

impl UserDao {
    pub fn new(db: Arc<MySqlPool>) -> Self {
        UserDao { db: db.clone() }
    }

    pub async fn get_users(&self) -> Result<Vec<User>, Error> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&*self.db)
            .await?;

        Ok(result)
    }
}
