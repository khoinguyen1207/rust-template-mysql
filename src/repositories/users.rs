use std::sync::Arc;
use sqlx::{Error, MySqlPool};

use crate::models::users::User;

#[derive(Clone, Debug)]
pub struct UserDao {
    db_pool: Arc<MySqlPool>,
}

impl UserDao {
    pub fn new(db_pool: Arc<MySqlPool>) -> Self {
        UserDao { db_pool: db_pool.clone() }
    }

    pub async fn get_users(&self) -> Result<Vec<User>, Error> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&*self.db_pool)
            .await?;

        Ok(result)
    }
}

