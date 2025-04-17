use axum::{response::Redirect, routing::get, Router}; 
use tower_service::Service;
use worker::*;
use axum::response::Html;
use include_dir::{include_dir, Dir};
use askama::Template;
use comrak::{markdown_to_html, markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};


static _TEMPLATES_DIR: Dir = include_dir!("templates");



fn router() -> Router {
    Router::new() 
        .route("/", get(home_page))
        .route("/about", get(about_page))
        .route("/1", get(fragment_handler))
        .route("/2", get(click_undo))
        .fallback(Redirect::permanent("/"))
}


// Main entry point for Cloudflare Worker
#[event(fetch)]
async fn fetch(req: HttpRequest, _env: Env, _ctx: Context) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}


/// Extends layout.html (navbar and footer)
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    content: String,
} 

/// Does not extend layout.html
#[derive(Template)]
#[template(path = "content_fragment.html")]
struct ContentFragmentTemplate {
    content: String,
} 

async fn about_page() -> Html<String> {
    let md = include_str!("../templates/md/original.md");
    let mut options = ComrakOptions::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.render.github_pre_lang = true;
    
    let plugins = ComrakPlugins::default();
   
    let html = markdown_to_html_with_plugins(md, &options, &plugins);
    let highlight_all = r#"<script>hljs.highlightAll();</script>"#;
    let highlight_html = format!("{}\n{}", highlight_all, &html);

    let template = ContentFragmentTemplate { content: highlight_html };
    Html(template.render().unwrap())
}


async fn fragment_handler() -> Html<String> {
    let md = "## Dynamic Markdown!";
    let html = markdown_to_html(md, &ComrakOptions::default());
    let template = ContentFragmentTemplate { content: html };
    Html(template.render().unwrap())
}



const CONTENT: &str  = include_str!("../templates/md/home.md");
async fn home_page() -> Html<String> {
    let mut options = ComrakOptions::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.shortcodes = true;
    options.extension.tasklist = true;

    let html = markdown_to_html(&CONTENT, &options);
    let template = IndexTemplate { content: html }; 
    Html(template.render().unwrap())
}



async fn click_undo() -> Html<String> {
    let options = ComrakOptions::default();
    let rendered = format!("{}", markdown_to_html(&CONTENT, &options)); 
    Html(rendered)

}


