use axum::{response::Redirect, routing::get, Router}; 
use tower_service::Service;
use worker::*;
use axum::response::Html;
use include_dir::{include_dir, Dir};
use askama::Template;
use comrak::{markdown_to_html, markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};

mod mdext; // set static options.extensions for ComrakOptions

static _TEMPLATES_DIR: Dir = include_dir!("templates");
const HOME_MD: &str = include_str!("../templates/md/home.md");
const ABOUT_MD: &str = include_str!("../templates/md/about.md");
const RESUME_MD: &str = include_str!("../templates/md/resume.md");


/// Set logical routes to content
fn router() -> Router {
    Router::new() 
        .route("/", get(home_page))
        .route("/about", get(about_page))
        .route("/resume", get(resume_page))
        .route("/home", get(content_home))
        .fallback(Redirect::permanent("/"))
}


// Main entry point for Cloudflare Worker
#[event(fetch)]
async fn fetch(req: HttpRequest, _env: Env, _ctx: Context) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}


/// Extends layout.html (navbar, buttons and footer)
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    content: String,
    content_box: bool,
} 

/// Does not extend layout.html
#[derive(Template)]
#[template(path = "content_template.html")]
struct ContentTemplate {
    content: String,
} 

/// used as '/' to init page OR fallback
/// include IndexTemplate with navbar, footer, meta tags.
/// use in router:
/// ```rust
/// Router::new().route("/", get(home_page))
/// ```
async fn home_page() -> Html<String> {
    let mut options = ComrakOptions::default();
    crate::mdext::enable_extensions(&mut options); 
    let html = markdown_to_html(&HOME_MD, &options);
    let template = IndexTemplate { content: html, content_box: true }; 
    Html(template.render().unwrap())
}


/// Loads home.md, content_template.html.
/// use in router:
/// ```rust
/// Router::new().route("/home", get(content_home))
/// ```
async fn content_home() -> Html<String> {
    let mut options = ComrakOptions::default();
    crate::mdext::enable_extensions(&mut options);
    let html = markdown_to_html(&HOME_MD, &options);
    let template = ContentTemplate { content: html };
    Html(template.render().unwrap())
}


/// Loads about.md with ContentTemplate
/// use in router:
/// ```rust
/// Router::new().route("/about", get(about_page))
/// ```
async fn about_page() -> Html<String> {
    let md = &ABOUT_MD;
    let mut options = ComrakOptions::default();
    crate::mdext::enable_extensions(&mut options);
    let plugins = ComrakPlugins::default();
    let html = markdown_to_html_with_plugins(md, &options, &plugins);
    // slight hack to run highlight.js when this content is loaded.
    let highlight_all = r#"<script>hljs.highlightAll();</script>"#;
    let highlight_html = format!("{}\n{}", highlight_all, &html);
    let template = ContentTemplate { content: highlight_html };
    Html(template.render().unwrap())
}



/// Loads about.md with ContentTemplate
/// use in router:
/// ```rust
/// Router::new().route("/resume", get(about_page))
/// ```
async fn resume_page() -> Html<String> {
    let md = &RESUME_MD;
    let mut options = ComrakOptions::default();
    crate::mdext::enable_extensions(&mut options);
    let plugins = ComrakPlugins::default();
    let html = markdown_to_html_with_plugins(md, &options, &plugins);
    let template = ContentTemplate { content: html };
    Html(template.render().unwrap())
}
