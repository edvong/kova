use kova::Error;
use kova::codec::{Decode, Value, decode_column};
use kova::dsl::Column;
use kova::types::{Database, Engine, FromRow, IdentifierMeta, Row};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CommentDto {
    pub id: i32,
    pub content: String,
    pub commenter: Author,
}

#[allow(non_upper_case_globals)]
impl CommentDto {
    pub const commenter: Column<Author, IdentifierMeta> = Column::identifier("comment_dto", "commenter");
}

pub struct PostView {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author: Author,
    pub comments: Vec<CommentDto>,
}

impl<X> FromRow<Engine, X> for PostView
    where
        i32: Decode<X>,
        String: Decode<X>,
        Author: Decode<X>,
        Vec<CommentDto>: Decode<X> {
    fn from_row_at(row: &<Engine as Database>::Row<'_>, idx: &mut usize) -> Result<Self, Error> {
        Ok(Self {
            id: row.get_column::<_, X>(idx, "id")?,
            title: row.get_column::<_, X>(idx, "title")?,
            content: row.get_column::<_, X>(idx, "content")?,
            author: row.get_column::<_, X>(idx, "author")?,
            comments: row.get_column::<_, X>(idx, "comments")?,
        })
    }
}

#[allow(non_upper_case_globals)]
impl PostView {
    pub const author: Column<Author, IdentifierMeta> = Column::identifier("post_view", "author");
    pub const comments: Column<Vec<CommentDto>, IdentifierMeta> =
        Column::identifier("post_view", "comments");
}