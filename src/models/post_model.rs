#[derive(Debug, sqlx::FromRow)]
pub struct Post {
    ID: u32,
    post_title: String,
    post_content: String,
    post_type: Option<String>,
}