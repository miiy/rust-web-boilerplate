// custom error handler
// https://github.com/actix/examples/blob/master/templating/tera/src/main.rs
use crate::AppState;
use actix_web::{
    body::BoxBody,
    dev::ServiceResponse,
    http::{header::ContentType, StatusCode},
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web, HttpResponse, Result,
};

// Custom error handlers, to return HTML responses when an error occurs.
pub fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new()
        .handler(StatusCode::NOT_FOUND, not_found)
        .handler(StatusCode::BAD_REQUEST, bad_request)
        .handler(StatusCode::UNAUTHORIZED, unauthorized)
        .handler(StatusCode::INTERNAL_SERVER_ERROR, internal_server_error)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Error handler for a 400 Bad request error.
fn bad_request<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Bad Request");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Error handler for a 401 Unauthorized error.
fn unauthorized<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Unauthorized");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Error handler for a 500 Internal Server Error.
fn internal_server_error<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Internal Server Error");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |err: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(err.to_string())
    };

    // let tera = request.app_data::<web::Data<Tera>>().map(|t| t.get_ref());
    let app_state = request
        .app_data::<web::Data<AppState>>()
        .map(|t| t.get_ref());
    let tera = match app_state {
        Some(app_state) => Some(&app_state.template.tera),
        None => return fallback(error),
    };

    match tera {
        Some(tera) => {
            let mut context = tera::Context::new();
            context.insert("error", error);
            context.insert("status_code", res.status().as_str());
            let template_name = get_template_name(res.status());
            let body = tera.render(template_name, &context);

            match body {
                Ok(body) => HttpResponse::build(res.status())
                    .content_type(ContentType::html())
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}


fn get_template_name(status_code: StatusCode) -> &'static str {
    match status_code {
        StatusCode::NOT_FOUND => "error/404.html",
        StatusCode::BAD_REQUEST => "error/400.html",
        StatusCode::UNAUTHORIZED => "error/401.html",
        StatusCode::INTERNAL_SERVER_ERROR => "error/error.html",
        _ => "error/error.html",
    }
}
