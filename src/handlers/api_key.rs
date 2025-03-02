use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{error::InternalError, FromRequest, HttpRequest, HttpResponse};
use serde_json::json;
use std::future::{ready, Ready};

use crate::configs::configs;

#[derive(Debug, Clone)]
pub struct ApiKeyMiddleware;

impl FromRequest for ApiKeyMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let api_key_secret: String = configs::get("x_api_key");

        let api_key = req
            .headers()
            .get("x-api-key")
            .and_then(|header_value| header_value.to_str().ok());

        if api_key.is_none() || api_key.unwrap() != api_key_secret {
            let json_response = HttpResponse::Unauthorized().json(json!({
                "message": "Unauthorized",
            }));
            let error = InternalError::from_response("", json_response).into();
            return ready(Err(error));
        }

        ready(Ok(ApiKeyMiddleware))
    }
}
