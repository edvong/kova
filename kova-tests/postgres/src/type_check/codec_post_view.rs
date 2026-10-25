use kova::{decode_column, Error};
use kova::codec::{Decode, Value};
use models::type_check::post_view::{Author, CommentDto};

pub struct ViewCodec;

impl Decode<ViewCodec> for Author {
    fn decode(value: Value<'_>) -> kova::Result<Self> {
        match value {
            Value::Text(t) => {
                serde_json::from_str(&t).map_err(|e| Error::ParseError(e.to_string()))
            }
            _ => Err(Error::TypeMismatch("Expected Text(JSON) for Author".into())),
        }
    }
}
decode_column!(ViewCodec, Author);

impl Decode<ViewCodec> for Vec<CommentDto> {
    fn decode(value: Value<'_>) -> kova::Result<Self> {
        match value {
            Value::Text(t) => {
                serde_json::from_str(&t).map_err(|e| Error::ParseError(e.to_string()))
            }
            _ => Err(Error::TypeMismatch("Expected Text(JSON) for Vec<CommentDto>".into())),
        }
    }
}
decode_column!(ViewCodec, Vec<CommentDto>);
