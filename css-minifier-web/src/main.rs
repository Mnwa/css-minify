use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use css_minify::optimizations::{Level, Minifier};
use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use yarte::Template;

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
    let output_css = minifier.minify(&request.input_css, level).unwrap_or_else(|e| e.to_string());

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
async fn main_css(css: web::Data<MinifiedCss>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .append_header(("Etag", &*css.hash))
        .body(css.css.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut hasher = DefaultHasher::default();
    let minifier = web::Data::new(Minifier::default());

    let minified_css = minifier
        .minify(include_str!("../static/main.css"), Level::Three)
        .expect("invalid css");

    minified_css.hash(&mut hasher);

    let output_css = web::Data::new(MinifiedCss {
        css: minified_css,
        hash: hasher.finish().to_string(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::FormConfig::default().limit(134_217_728))
            .app_data(minifier.clone())
            .app_data(output_css.clone())
            .service(index)
            .service(minify_css)
            .service(main_css)
    })
        .bind(std::env::var("HTTP_HOST").unwrap_or_else(|_| "0.0.0.0:8081".into()))?
        .run()
        .await
}

struct MinifiedCss {
    css: String,
    hash: String,
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
