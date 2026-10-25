use kova::FromRow;
use serde::{Deserialize};

#[derive(FromRow)]
pub struct PostView {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author: Author,
    pub comments: Vec<CommentDto>,
}

#[derive(Debug, PartialEq, Deserialize, FromRow)]
pub struct CommentDto {
    pub id: i32,
    pub content: String,
    pub commenter: Author,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Author {
    pub id: i32,
    pub name: String,
}
