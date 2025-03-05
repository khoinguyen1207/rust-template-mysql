use actix_web::dev::Payload;
use actix_web::ResponseError;
use actix_web::{error::InternalError, http, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_derive::{Deserialize, Serialize};
use std::future::{ready, Ready};
use crate::configs::configs;
use crate::serializes::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    pub sub: String,
    pub aud: bool,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct JwtMiddleware {
    pub address: String,
}

impl FromRequest for JwtMiddleware {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let jwt_secret: String = configs::get("jwt_secret");

        let token = req
            .cookie("authorization-cookie")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            let error = AppError::new(401).message("You are not logged in, please provide token").code("UNAUTHORIZED");
            let error = InternalError::from_response("", error.error_response()).into();
            return ready(Err(error));
        }

        let payload = match decode::<TokenPayload>(
            &token.unwrap(),
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                let error = AppError::new(401).message("Invalid token").code("UNAUTHORIZED");
                return ready(Err(InternalError::from_response("", error.error_response()).into()));
            }
        };

        let address = payload.sub;
        req.extensions_mut().insert(address.clone());

        ready(Ok(JwtMiddleware {
            address: address.clone(),
        }))
    }
}
