use crate::{models::users::User, repositories::users::UserDao, serializes::error::AppError};

#[derive(Clone, Debug)]
pub struct UserSrv {
    user_dao: UserDao,
}

impl UserSrv {
    pub fn new(user_dao: UserDao) -> Self {
        UserSrv {
            user_dao: user_dao.clone(),
        } 
    }

    pub async fn get_users(&self) -> Result<Vec<User>, AppError> {
        match self.user_dao.get_users().await {
            Ok(users) => Ok(users),
            Err(error) => Err(AppError::new(500).message(&error.to_string())),
        }
            
    }
}
