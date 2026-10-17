use kova::dsl::Column;
use kova::types::{Database, Decode, Engine, FromRow, IdentifierMeta, Value};
use kova::{Error, decode_column};
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
    pub const commenter: Column<Author, IdentifierMeta> =
        Column::identifier("comment_dto", "commenter");
}

pub struct PostView {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author: Author,
    pub comments: Vec<CommentDto>,
}

impl FromRow for PostView {
    type DB = Engine;

    fn from_row(_: &<Self::DB as Database>::Row<'_>) -> Result<Self, Error> {
        unimplemented!()
    }
}

#[allow(non_upper_case_globals)]
impl PostView {
    pub const author: Column<Author, IdentifierMeta> =
        Column::identifier("post_view", "author");
    pub const comments: Column<Vec<CommentDto>, IdentifierMeta> =
        Column::identifier("post_view", "comments");
}

pub struct ViewCodec;

impl<'a> Decode<'a, ViewCodec> for Author {
    fn decode(value: Value<'a>) -> kova::Result<Self> {
        match value {
            Value::Text(t) => {
                serde_json::from_str(&t).map_err(|e| Error::ParseError(e.to_string()))
            }
            _ => Err(Error::TypeMismatch("Expected Text(JSON) for Author".into())),
        }
    }
}
decode_column!(ViewCodec, Author);

impl<'a> Decode<'a, ViewCodec> for Vec<CommentDto> {
    fn decode(value: Value<'a>) -> kova::Result<Self> {
        match value {
            Value::Text(t) => {
                serde_json::from_str(&t).map_err(|e| Error::ParseError(e.to_string()))
            }
            Value::Null => Ok(Vec::new()),
            _ => Err(Error::TypeMismatch("Expected Text(JSON) for Vec<CommentDto>".into())),
        }
    }
}
decode_column!(ViewCodec, Vec<CommentDto>);