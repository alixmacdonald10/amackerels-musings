
use std::fmt;

use askama::Template;
use axum::{
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    extract::Query,
};
use serde::Deserialize;




#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate;

pub(crate) async fn handler_404() -> impl IntoResponse {
    tracing::warn!("Path not found.");
    (StatusCode::NOT_FOUND, Html(NotFoundTemplate.render().unwrap()))
}


#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;


pub(crate) async fn index() -> Html<String> {
    let index = IndexTemplate;

    // TODO: get top 5 blog posts on load and add to template with infinite scroll

    Html(index.render().unwrap())
}



#[derive(Deserialize)]
pub(crate) struct RedirectParams {
    pub(crate) target: RedirectTarget
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum RedirectTarget {
    Github,
    LinkedIn
}

impl fmt::Display for RedirectTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let url = match &self {
            RedirectTarget::Github => "https://github.com/alixmacdonald10",
            RedirectTarget::LinkedIn => "https://linkedin.com/in/alixmac"
        }; 
        write!(f, "{}", url)
    }
}

pub(crate) async fn redirect(params: Query<RedirectParams>) -> impl IntoResponse {
     
    let url = format!("{}", params.target);
    tracing::debug!("Redirecting to {}", &url);
    
    let mut headers = HeaderMap::new();
    headers.insert("HX-Redirect", url.parse().unwrap());
    (StatusCode::PERMANENT_REDIRECT, headers)
}


// pub(crate) async fn get_blog_post(params: Query) {
//
//     // TODO: get blog post from db? or memory
//
// }
