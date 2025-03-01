use axum::{routing::get, 
    Router, 
    response::Redirect}; 
use tower_service::Service;
use worker::*;
use axum::response::Html;
use include_dir::{include_dir, Dir};
use askama_axum::Template;

// Template logic for Damage Sims Page (dps_sims.rs)
// mod about;
// mod home;


static _TEMPLATES_DIR: Dir = include_dir!("templates");

fn router() -> Router {
    Router::new() 
        .route("/", get(home_page))
        .fallback(Redirect::permanent("/"))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate; 


async fn home_page() -> Html<String> {
    let template = IndexTemplate; 
    let rendered = template.render().unwrap();
    
    Html(rendered)
}


