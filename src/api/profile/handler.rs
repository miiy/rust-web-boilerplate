use super::{error::ProfileError, service::Service};
use crate::api::error::APIError;
use crate::api::jwt::AuthenticatedUser;
use crate::AppState;
use actix_web::{web, Error, HttpResponse};

// GET /profile
pub async fn profile(
    user: Option<web::ReqData<AuthenticatedUser>>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    if let Some(user_data) = user {
        let user = user_data.into_inner();
        let resp = Service::profile(&user.name, &app_state.db)
            .await
            .map_err(APIError::from)?;
        Ok(HttpResponse::Ok().json(resp))
    } else {
        return Err(APIError::from(ProfileError::NotFound).into());
    }
}
