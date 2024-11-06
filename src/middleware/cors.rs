use actix_cors::Cors;
use actix_web::http;

pub fn cors(origin: &str) -> Cors {
    let cors = Cors::default()
        .allowed_origin(origin)
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .supports_credentials()
        .max_age(3600);
    cors
}
