use kova::dsl::VirtualColumn;
use kova::types::{Database, Decode, Engine, FromRow, Value};
use kova::{Error, bind_type};
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
    pub const commenter: VirtualColumn<Engine, Author> =
        kova::private::create_virtual_column("comment_dto", "commenter");
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
    pub const author: VirtualColumn<Engine, Author> =
        kova::private::create_virtual_column("post_view", "author");
    pub const comments: VirtualColumn<Engine, Vec<CommentDto>> =
        kova::private::create_virtual_column("post_view", "comments");
}

pub struct ViewCodec;

impl<'a> Decode<'a, Engine, ViewCodec> for Author {
    fn decode(value: Value<'a>) -> kova::Result<Self> {
        match value {
            Value::Text(t) => {
                serde_json::from_str(&t).map_err(|e| Error::ParseError(e.to_string()))
            }
            _ => Err(Error::TypeMismatch("Expected Text(JSON) for Author".into())),
        }
    }
}
bind_type!(ViewCodec, Author);

impl<'a> Decode<'a, Engine, ViewCodec> for Vec<CommentDto> {
    fn decode(value: Value<'a>) -> kova::Result<Self> {
        match value {
            Value::Text(t) => {
                serde_json::from_str(&t).map_err(|e| Error::ParseError(e.to_string()))
            }
            Value::Null => Ok(Vec::new()), // autowrap COALESCE(..., '[]'::jsonb)
            _ => Err(Error::TypeMismatch("Expected Text(JSON) for Vec<CommentDto>".into())),
        }
    }
}
bind_type!(ViewCodec, Vec<CommentDto>);
