use super::{error::UserError, service::Service};
use crate::api::error::APIError;
use crate::AppState;
use actix_web::{web, Error, HttpResponse};
use crate::api::jwt::AuthenticatedUser;

// GET /users/me
pub async fn me(user: Option<web::ReqData<AuthenticatedUser>>, app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    if let Some(user_data) = user {
        let user = user_data.into_inner();
        let resp = Service::detail(&user.name, &app_state.db)
        .await
        .map_err(APIError::from)?;
        Ok(HttpResponse::Ok().json(resp))
    } else {
        return Err(APIError::from(UserError::NotFound).into());
    }

}
