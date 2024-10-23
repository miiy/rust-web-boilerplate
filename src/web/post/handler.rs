use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "post/index.html")]
struct IndexTemplate<'a> {
    app_name: &'a str,
    page_title: &'a str,
    keywords: &'a str,
    description: &'a str,
}

// /posts
pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let t = IndexTemplate {
        app_name: app_name,
        page_title: "Home",
        keywords: "keywords",
        description: "description",
    };
    HttpResponse::Ok().body(t.render().unwrap())
}

#[derive(Template)]
#[template(path = "post/detail.html")]
struct DetailTemplate<'a> {
    app_name: &'a str,
    page_title: &'a str,
    keywords: &'a str,
    description: &'a str,
}

// /posts/{id}
pub async fn detail(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let t = DetailTemplate {
        app_name: app_name,
        page_title: "Home",
        keywords: "keywords",
        description: "description",
    };
    println!("111");
    HttpResponse::Ok().body(t.render().unwrap())
}

#[derive(Template)]
#[template(path = "post/create.html")]
struct CreateTemplate<'a> {
    app_name: &'a str,
    page_title: &'a str,
    keywords: &'a str,
    description: &'a str,
}

pub async fn create(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let t = CreateTemplate {
        app_name: app_name,
        page_title: "Home",
        keywords: "keywords",
        description: "description",
    };
    HttpResponse::Ok().body(t.render().unwrap())
}

#[derive(Template)]
#[template(path = "post/edit.html")]
struct EditTemplate<'a> {
    app_name: &'a str,
    page_title: &'a str,
    keywords: &'a str,
    description: &'a str,
}

// /posts/{id}/edit
pub async fn edit(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let t = EditTemplate {
        app_name: app_name,
        page_title: "Home",
        keywords: "keywords",
        description: "description",
    };
    HttpResponse::Ok().body(t.render().unwrap())
}
