use std::future::{ready, Ready};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage, HttpResponse,
};
use futures_util::{future::LocalBoxFuture, FutureExt, TryFutureExt};

use crate::api::error::{ErrorEntity, ErrorResponse};
use crate::api::jwt::AuthenticatedUser;
use crate::AppState;
use actix_web::http::header::Header;
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Authentication;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let app_state = req.app_data::<web::Data<AppState>>().unwrap();

        // parse the authorization header
        let schema = Authorization::<Bearer>::parse(&req).unwrap();
        let token = schema.as_ref().token();
        let token_data = match app_state.jwt.decode(token) {
            Ok(token_data) => token_data,
            Err(e) => {
                let resp = ErrorResponse {
                    error: ErrorEntity {
                        code: 401,
                        message: e.to_string(),
                    },
                };
                return Box::pin(async move {
                    Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(resp)
                            .map_into_right_body(),
                    ))
                });
            }
        };

        // extract the user from the token
        let user = AuthenticatedUser {
            name: token_data.claims.sub,
        };
        // insert authenticated user into the request extensions
        req.extensions_mut().insert(user);

        // println!("Authorization from start. You requested: {}", req.path());
        self.service
            .call(req)
            .map_ok(ServiceResponse::map_into_left_body)
            .boxed_local()
    }
}
