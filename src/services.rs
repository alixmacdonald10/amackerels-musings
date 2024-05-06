
use std::fmt;

use askama::Template;
use axum::{
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    extract::Query,
};
use serde::Deserialize;
use uuid::Uuid;


struct BlogPost {
    id: Uuid,
    title: String,
    content: String,
}



#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate;

pub(crate) async fn handler_404() -> impl IntoResponse {
    tracing::warn!("Path not found.");
    (StatusCode::NOT_FOUND, Html(NotFoundTemplate.render().unwrap()))
}


#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    blog_post: BlogPost
}


pub(crate) async fn index() -> Html<String> {

    let content = "
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Et ultrices neque ornare aenean. Sit amet volutpat consequat mauris nunc congue. Nunc scelerisque viverra mauris in. Tempus imperdiet nulla malesuada pellentesque elit eget gravida cum. Eget nulla facilisi etiam dignissim diam quis enim lobortis scelerisque. Viverra accumsan in nisl nisi scelerisque. Feugiat sed lectus vestibulum mattis. Mi tempus imperdiet nulla malesuada. Leo vel fringilla est ullamcorper eget. Cursus sit amet dictum sit amet justo. Magna eget est lorem ipsum dolor sit amet. Fermentum et sollicitudin ac orci phasellus egestas tellus. Cursus metus aliquam eleifend mi in. Luctus venenatis lectus magna fringilla urna porttitor rhoncus dolor purus.

Tempor orci dapibus ultrices in iaculis nunc sed augue. Mi sit amet mauris commodo quis imperdiet massa tincidunt nunc. Pharetra convallis posuere morbi leo urna molestie. Erat nam at lectus urna duis convallis convallis tellus. Odio eu feugiat pretium nibh. Sapien pellentesque habitant morbi tristique senectus et. Urna cursus eget nunc scelerisque viverra mauris in aliquam. Commodo quis imperdiet massa tincidunt nunc pulvinar sapien et ligula. Tellus orci ac auctor augue mauris augue. Cras ornare arcu dui vivamus arcu felis bibendum. Faucibus interdum posuere lorem ipsum dolor. Et malesuada fames ac turpis egestas sed tempus. Ipsum dolor sit amet consectetur adipiscing. Mi bibendum neque egestas congue quisque. Semper auctor neque vitae tempus. Sit amet est placerat in. Habitant morbi tristique senectus et netus.

Nisl rhoncus mattis rhoncus urna neque viverra justo nec. Ultrices sagittis orci a scelerisque purus. Proin libero nunc consequat interdum varius. Ac feugiat sed lectus vestibulum. Id venenatis a condimentum vitae sapien pellentesque habitant. Tortor at auctor urna nunc id cursus metus aliquam eleifend. Vel orci porta non pulvinar neque laoreet suspendisse interdum consectetur. In dictum non consectetur a erat nam at lectus. Vitae aliquet nec ullamcorper sit amet risus nullam eget. Urna molestie at elementum eu facilisis sed odio morbi. Cursus eget nunc scelerisque viverra mauris in aliquam. Ut faucibus pulvinar elementum integer enim neque. Augue mauris augue neque gravida. Faucibus nisl tincidunt eget nullam non nisi. Interdum consectetur libero id faucibus nisl tincidunt eget nullam non. In tellus integer feugiat scelerisque varius morbi.

Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Et ultrices neque ornare aenean. Sit amet volutpat consequat mauris nunc congue. Nunc scelerisque viverra mauris in. Tempus imperdiet nulla malesuada pellentesque elit eget gravida cum. Eget nulla facilisi etiam dignissim diam quis enim lobortis scelerisque. Viverra accumsan in nisl nisi scelerisque. Feugiat sed lectus vestibulum mattis. Mi tempus imperdiet nulla malesuada. Leo vel fringilla est ullamcorper eget. Cursus sit amet dictum sit amet justo. Magna eget est lorem ipsum dolor sit amet. Fermentum et sollicitudin ac orci phasellus egestas tellus. Cursus metus aliquam eleifend mi in. Luctus venenatis lectus magna fringilla urna porttitor rhoncus dolor purus.

Tempor orci dapibus ultrices in iaculis nunc sed augue. Mi sit amet mauris commodo quis imperdiet massa tincidunt nunc. Pharetra convallis posuere morbi leo urna molestie. Erat nam at lectus urna duis convallis convallis tellus. Odio eu feugiat pretium nibh. Sapien pellentesque habitant morbi tristique senectus et. Urna cursus eget nunc scelerisque viverra mauris in aliquam. Commodo quis imperdiet massa tincidunt nunc pulvinar sapien et ligula. Tellus orci ac auctor augue mauris augue. Cras ornare arcu dui vivamus arcu felis bibendum. Faucibus interdum posuere lorem ipsum dolor. Et malesuada fames ac turpis egestas sed tempus. Ipsum dolor sit amet consectetur adipiscing. Mi bibendum neque egestas congue quisque. Semper auctor neque vitae tempus. Sit amet est placerat in. Habitant morbi tristique senectus et netus.

Nisl rhoncus mattis rhoncus urna neque viverra justo nec. Ultrices sagittis orci a scelerisque purus. Proin libero nunc consequat interdum varius. Ac feugiat sed lectus vestibulum. Id venenatis a condimentum vitae sapien pellentesque habitant. Tortor at auctor urna nunc id cursus metus aliquam eleifend. Vel orci porta non pulvinar neque laoreet suspendisse interdum consectetur. In dictum non consectetur a erat nam at lectus. Vitae aliquet nec ullamcorper sit amet risus nullam eget. Urna molestie at elementum eu facilisis sed odio morbi. Cursus eget nunc scelerisque viverra mauris in aliquam. Ut faucibus pulvinar elementum integer enim neque. Augue mauris augue neque gravida. Faucibus nisl tincidunt eget nullam non nisi. Interdum consectetur libero id faucibus nisl tincidunt eget nullam non. In tellus integer feugiat scelerisque varius morbi.
";
    
    let index = IndexTemplate {
        blog_post: BlogPost {
            id: Uuid::now_v7(),
            title: "Blog Post 1".to_string(),
            content: content.to_owned()
        },
    };
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



#[derive(Template)]
#[template(path = "blog_post.html")]
struct BlogPostTemplate {
    blog_post: BlogPost
}

#[derive(Deserialize)]
pub(crate) struct GetBlogPostParams {
    id: Uuid,
}


pub(crate) async fn get_blog_post(Query(params): Query<GetBlogPostParams>) -> Html<String> {
    let id = params.id; 
    tracing::info!("got id {:#?}", id);

    // TODO: get blog post from db? or 
    let content = format!("
POST RECEIVED ID: {:#?}
POST LOADED AT: {:#?}


Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Et ultrices neque ornare aenean. Sit amet volutpat consequat mauris nunc congue. Nunc scelerisque viverra mauris in. Tempus imperdiet nulla malesuada pellentesque elit eget gravida cum. Eget nulla facilisi etiam dignissim diam quis enim lobortis scelerisque. Viverra accumsan in nisl nisi scelerisque. Feugiat sed lectus vestibulum mattis. Mi tempus imperdiet nulla malesuada. Leo vel fringilla est ullamcorper eget. Cursus sit amet dictum sit amet justo. Magna eget est lorem ipsum dolor sit amet. Fermentum et sollicitudin ac orci phasellus egestas tellus. Cursus metus aliquam eleifend mi in. Luctus venenatis lectus magna fringilla urna porttitor rhoncus dolor purus.

Tempor orci dapibus ultrices in iaculis nunc sed augue. Mi sit amet mauris commodo quis imperdiet massa tincidunt nunc. Pharetra convallis posuere morbi leo urna molestie. Erat nam at lectus urna duis convallis convallis tellus. Odio eu feugiat pretium nibh. Sapien pellentesque habitant morbi tristique senectus et. Urna cursus eget nunc scelerisque viverra mauris in aliquam. Commodo quis imperdiet massa tincidunt nunc pulvinar sapien et ligula. Tellus orci ac auctor augue mauris augue. Cras ornare arcu dui vivamus arcu felis bibendum. Faucibus interdum posuere lorem ipsum dolor. Et malesuada fames ac turpis egestas sed tempus. Ipsum dolor sit amet consectetur adipiscing. Mi bibendum neque egestas congue quisque. Semper auctor neque vitae tempus. Sit amet est placerat in. Habitant morbi tristique senectus et netus.

Nisl rhoncus mattis rhoncus urna neque viverra justo nec. Ultrices sagittis orci a scelerisque purus. Proin libero nunc consequat interdum varius. Ac feugiat sed lectus vestibulum. Id venenatis a condimentum vitae sapien pellentesque habitant. Tortor at auctor urna nunc id cursus metus aliquam eleifend. Vel orci porta non pulvinar neque laoreet suspendisse interdum consectetur. In dictum non consectetur a erat nam at lectus. Vitae aliquet nec ullamcorper sit amet risus nullam eget. Urna molestie at elementum eu facilisis sed odio morbi. Cursus eget nunc scelerisque viverra mauris in aliquam. Ut faucibus pulvinar elementum integer enim neque. Augue mauris augue neque gravida. Faucibus nisl tincidunt eget nullam non nisi. Interdum consectetur libero id faucibus nisl tincidunt eget nullam non. In tellus integer feugiat scelerisque varius morbi.

Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Et ultrices neque ornare aenean. Sit amet volutpat consequat mauris nunc congue. Nunc scelerisque viverra mauris in. Tempus imperdiet nulla malesuada pellentesque elit eget gravida cum. Eget nulla facilisi etiam dignissim diam quis enim lobortis scelerisque. Viverra accumsan in nisl nisi scelerisque. Feugiat sed lectus vestibulum mattis. Mi tempus imperdiet nulla malesuada. Leo vel fringilla est ullamcorper eget. Cursus sit amet dictum sit amet justo. Magna eget est lorem ipsum dolor sit amet. Fermentum et sollicitudin ac orci phasellus egestas tellus. Cursus metus aliquam eleifend mi in. Luctus venenatis lectus magna fringilla urna porttitor rhoncus dolor purus.

Tempor orci dapibus ultrices in iaculis nunc sed augue. Mi sit amet mauris commodo quis imperdiet massa tincidunt nunc. Pharetra convallis posuere morbi leo urna molestie. Erat nam at lectus urna duis convallis convallis tellus. Odio eu feugiat pretium nibh. Sapien pellentesque habitant morbi tristique senectus et. Urna cursus eget nunc scelerisque viverra mauris in aliquam. Commodo quis imperdiet massa tincidunt nunc pulvinar sapien et ligula. Tellus orci ac auctor augue mauris augue. Cras ornare arcu dui vivamus arcu felis bibendum. Faucibus interdum posuere lorem ipsum dolor. Et malesuada fames ac turpis egestas sed tempus. Ipsum dolor sit amet consectetur adipiscing. Mi bibendum neque egestas congue quisque. Semper auctor neque vitae tempus. Sit amet est placerat in. Habitant morbi tristique senectus et netus.

Nisl rhoncus mattis rhoncus urna neque viverra justo nec. Ultrices sagittis orci a scelerisque purus. Proin libero nunc consequat interdum varius. Ac feugiat sed lectus vestibulum. Id venenatis a condimentum vitae sapien pellentesque habitant. Tortor at auctor urna nunc id cursus metus aliquam eleifend. Vel orci porta non pulvinar neque laoreet suspendisse interdum consectetur. In dictum non consectetur a erat nam at lectus. Vitae aliquet nec ullamcorper sit amet risus nullam eget. Urna molestie at elementum eu facilisis sed odio morbi. Cursus eget nunc scelerisque viverra mauris in aliquam. Ut faucibus pulvinar elementum integer enim neque. Augue mauris augue neque gravida. Faucibus nisl tincidunt eget nullam non nisi. Interdum consectetur libero id faucibus nisl tincidunt eget nullam non. In tellus integer feugiat scelerisque varius morbi.
", id, std::time::Instant::now());
    
    let blog_post_template = BlogPostTemplate {
        blog_post: BlogPost {
            id: Uuid::now_v7(),
            title: "Blog Post".to_string(),
            content: content.to_owned()
        },
    };

    Html(blog_post_template.render().unwrap())
}
