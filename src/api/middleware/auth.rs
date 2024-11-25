use actix_web::{
    dev::ServiceRequest, error, web, Error, HttpMessage
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::{api::jwt::AuthenticatedUser, AppState};

/// Validator that:
/// - accepts Bearer auth;
/// - returns a custom response for requests without a valid Bearer Authorization header;
/// - rejects tokens containing an "x" (for quick testing using command line HTTP clients).
pub async fn validator(
    req: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let Some(credentials) = credentials else {
        return Err((error::ErrorBadRequest("no bearer header"), req));
    };
    let app_state = req.app_data::<web::Data<AppState>>().unwrap();

    let claims = app_state.jwt.decode(credentials.token()).unwrap().claims;
    let user = AuthenticatedUser{
        name: claims.sub,
    };
    req.extensions_mut().insert(user);

    // if credentials.token().contains('x') {
    //     return Err((error::ErrorBadRequest("token contains x"), req));
    // }

    Ok(req)
}

// Use this to create the middleware
// HttpAuthentication::with_fn(validator)

