use actix_web::http::header::ETAG;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use css_minify::optimizations::{Level, Minifier};
use serde::Deserialize;
use std::str::FromStr;
use yarte::Template;

mod styles;

#[get("/")]
async fn index() -> impl Responder {
    match Template::call(&IndexTemplate::default()) {
        Ok(response) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(response),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body(e.to_string()),
    }
}

#[post("/")]
async fn minify_css(
    request: web::Form<MinifyRequest>,
    minifier: web::Data<Minifier>,
) -> impl Responder {
    let level = Level::from_str(&request.level).unwrap_or(Level::One);
    let output_css = minifier
        .minify(&request.input_css, level)
        .unwrap_or_else(|e| e.to_string());

    match Template::call(&IndexTemplate {
        input_css: Some(request.into_inner().input_css),
        output_css: Some(output_css),
        level,
    }) {
        Ok(response) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(response),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body(e.to_string()),
    }
}

#[get("/static/main.css")]
async fn main_css() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .append_header((ETAG, styles::STYLES_HASH))
        .body(styles::STYLES)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let minifier = web::Data::new(Minifier::default());

    HttpServer::new(move || {
        App::new()
            .app_data(web::FormConfig::default().limit(134_217_728))
            .app_data(minifier.clone())
            .service(index)
            .service(minify_css)
            .service(main_css)
    })
    .bind(std::env::var("HTTP_HOST").unwrap_or_else(|_| "0.0.0.0:8081".into()))?
    .run()
    .await
}

#[derive(Debug, Default, Clone, Template)]
#[template(path = "index")]
struct IndexTemplate {
    input_css: Option<String>,
    output_css: Option<String>,
    level: Level,
}

#[derive(Deserialize)]
struct MinifyRequest {
    input_css: String,
    level: String,
}
