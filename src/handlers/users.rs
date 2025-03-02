use actix_web::{web, HttpResponse};
use std::result::Result::Ok;
use crate::{
    serializes::{common::ApiResponse, error::AppError}, services::users::UserSrv,
};

async fn get_users(user_srv: web::Data<UserSrv>) -> Result<HttpResponse, AppError> {
    match user_srv.get_users().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse {
            message: "Get users successfully".to_string(),
            data: Some(data),
        })),
        Err(e) => Err(e),
    }
}

pub fn route(conf: &mut web::ServiceConfig) {
    conf.service(web::scope("/users").route("", web::get().to(get_users)));
}
