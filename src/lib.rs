use axum::{routing::get, 
    Router, 
    response::Redirect}; 
use tower_service::Service;
use worker::*;
use axum::response::Html;
use include_dir::{include_dir, Dir};
use askama_axum::Template;


static _TEMPLATES_DIR: Dir = include_dir!("templates");

fn router() -> Router {
    Router::new() 
        .route("/", get(home_page))
        .route("/1", get(click_add))
        .route("/2", get(click_undo))
        .fallback(Redirect::permanent("/"))
}
// Main entry point for Cloudflare Worker
#[event(fetch)]
async fn fetch(req: HttpRequest, _env: Env, _ctx: Context) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    content: String,
} 

const CONTENT: &str  = "We are the original text!";

async fn home_page() -> Html<String> {

    let text = &CONTENT;
    let template = IndexTemplate { content: text.to_string() }; 
    let rendered = template.render().unwrap();
    
    Html(rendered)
}

async fn click_add() -> Html<String> {
    let text = "We are not the same!"; 
    let rendered = format!("<h1>{}</h1>", text.to_string());
    Html(rendered)
}

async fn click_undo() -> Html<String> {
    let text = &CONTENT;
    let rendered = format!("<h1>{}</h1>", text.to_string());
    Html(rendered)

}


