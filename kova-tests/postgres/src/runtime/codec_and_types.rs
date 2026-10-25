use crate::runtime::codec::{AppCodec, ViewCodec};
use crate::runtime::runtime_client;
use chrono::{TimeZone, Utc};
use jiff::Timestamp;
use kova::Error;
use kova::codec::{Compatible, Encode, Static, Value};
use kova::dsl::func::{JsonbAgg, JsonbBuildObject};
use kova::query::Select;
use models::runtime::comment::Comment;
use models::runtime::event_log::{AuditPayload, Decimal, EventLog, TagList};
use models::runtime::order::Order;
use models::runtime::post::Post;
use models::runtime::post_view::{Author, CommentDto, PostView};
use models::runtime::user::User;
use std::collections::HashMap;
use std::str::FromStr;
use url::Url;
use uuid::Uuid;

// Custom type that intentionally fails in `Encode` to test `OperandNode::Error` deferred propagation.
struct FailingPgParam;
impl Compatible<Static<String>, AppCodec> for Static<FailingPgParam> {}
impl<'a> Encode<'a, AppCodec> for FailingPgParam {
    fn encode(self) -> kova::Result<Value<'a>> {
        Err(Error::ValidationError("Postgres encode rejected".into()))
    }
}

#[tokio::test]
async fn pg_wire_protocol_primitives_and_error_variants() {
    let client = runtime_client().await;

    let raw: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];
    client
        .insert(Order {
            id: 0,
            user_id: 7,
            amount: 88.5,
            discount: None,
            is_active: true,
            raw_data: raw.to_vec(),
        })
        .execute()
        .await
        .unwrap();

    // 1. Lossless integer upcast encoding (`i8` -> `Int2`, `u16` -> `Int4`, `u32` -> `Int8`) and `[u8; 4]` extraction
    let small_i8: i8 = 12;
    let small_u16: u16 = 1000;
    let small_u32: u32 = 400_000;
    let tup = client
        .statement("SELECT $1::int2, $2::int4, $3::int8, $4::bytea")
        .bind((small_i8, small_u16, small_u32, &raw))
        .load_in::<(i16, i32, i64, [u8; 4])>()
        .await
        .unwrap();
    assert_eq!(tup, vec![(12_i16, 1000_i32, 400_000_i64, raw)]);

    // 2. `[u8; N]` length mismatch returns `Error::ParseError`
    let len_err = client
        .select::<Order>()
        .columns(Order::raw_data)
        .load_in::<[u8; 16]>()
        .await
        .unwrap_err();
    assert!(matches!(len_err, Error::ParseError(_)));

    // 3. Non-nullable type reading SQL NULL returns `Error::UnexpectedNull`
    let null_err = client
        .select::<Order>()
        .columns(Order::discount)
        .load_in::<f64>()
        .await
        .unwrap_err();
    assert!(matches!(null_err, Error::UnexpectedNull(_)));

    // 4. Storage type mismatch (`String` decoding `Int4`) returns `Error::TypeMismatch`
    let mismatch_err = client
        .select::<Order>()
        .columns(Order::user_id)
        .load_in::<String>()
        .await
        .unwrap_err();
    assert!(matches!(mismatch_err, Error::TypeMismatch(_)));
}

#[tokio::test]
async fn pg_operand_encode_error_deferred_and_propagated_on_build() {
    let client = runtime_client().await.codec::<AppCodec>();

    let err = client
        .select::<User>()
        .filter(User::name.eq(FailingPgParam))
        .load()
        .await
        .unwrap_err();
    assert!(
        matches!(err, Error::ValidationError(ref msg) if msg.contains("Postgres encode rejected")),
        "unexpected error: {err:?}"
    );
}

#[tokio::test]
async fn pg_custom_app_codec_jsonb_and_array_wire_roundtrip() {
    let client = runtime_client().await.codec::<AppCodec>();

    let id = Uuid::new_v4();
    let created = Utc.with_ymd_and_hms(2026, 7, 31, 8, 0, 0).unwrap();
    let updated = Timestamp::from_str("2026-07-31T08:30:00Z").unwrap();
    let url = Url::parse("https://rust-lang.org").unwrap();
    let meta = HashMap::from([
        ("role".to_string(), "admin".to_string()),
        ("region".to_string(), "us".to_string()),
    ]);

    let log = EventLog {
        id,
        description: "pg_jsonb_wire_test".into(),
        created_at: created,
        updated_at: updated,
        website: url,
        metadata: meta.clone(),
        cost: Decimal(1000),
        payload: AuditPayload { actor: "init".into(), level: 1 },
        labels: TagList(vec!["init".into()]),
    };

    client.insert(&log).execute().await.unwrap();

    // Update Decimal, AuditPayload (JSONB), TagList (TEXT[]), and Compatible<&str -> Url>
    let payload = AuditPayload { actor: "root".into(), level: 5 };
    let labels = TagList(vec!["rust".into(), "postgres".into()]);
    let slice_check: &[&str] = &["rust"];

    client
        .update::<EventLog>()
        .set((
            EventLog::cost.increment(500_i64),
            EventLog::payload.set(&payload),
            EventLog::labels.set(&labels),
            EventLog::website.set("https://docs.rs"),
        ))
        .filter(
            EventLog::id.eq(&id)
                & EventLog::metadata.json_path_text("role").eq("admin")
                & EventLog::metadata.json_has_all_keys(["role", "region"]),
        )
        .execute()
        .await
        .unwrap();

    // Full roundtrip extraction including `TagList` decoded from `Value::RawRead` (`TEXT[]`)
    let fetched = client
        .select::<EventLog>()
        .filter(
            EventLog::id.eq(&id)
                & EventLog::labels.array_contains(slice_check)
                & EventLog::payload.json_path_text("actor").eq("root"),
        )
        .columns((
            EventLog::id,
            EventLog::website,
            EventLog::metadata,
            EventLog::cost.add(100_i64),
            EventLog::payload,
            EventLog::labels,
        ))
        .load_in::<(Uuid, Url, HashMap<String, String>, Decimal, AuditPayload, TagList)>()
        .await
        .unwrap();

    assert_eq!(fetched.len(), 1);
    assert_eq!(fetched[0].0, id);
    assert_eq!(fetched[0].1.as_str(), "https://docs.rs/");
    assert_eq!(fetched[0].2, meta);
    assert_eq!(fetched[0].3, Decimal(1600));
    assert_eq!(fetched[0].4, payload);
    assert_eq!(fetched[0].5, labels);
}

#[tokio::test]
async fn pg_view_codec_nested_jsonb_subquery_mapping() {
    let client = runtime_client().await.codec::<ViewCodec>();

    let u1 = client
        .insert(User { id: 0, name: "Alice".into(), bio: None, balance: 0.0, is_active: true })
        .returning(User::id)
        .execute_in::<i32>()
        .await
        .unwrap()[0];
    let u2 = client
        .insert(User { id: 0, name: "Bob".into(), bio: None, balance: 0.0, is_active: true })
        .returning(User::id)
        .execute_in::<i32>()
        .await
        .unwrap()[0];

    let p1 = client
        .insert(Post {
            id: 0,
            title: "Postgres JSONB View".into(),
            content: "Zero-copy JSONB slicing".into(),
            author_id: u1,
        })
        .returning(Post::id)
        .execute_in::<i32>()
        .await
        .unwrap()[0];

    client
        .insert(Comment {
            id: 0,
            post_id: p1,
            user_id: u2,
            content: "Works seamlessly!".into(),
        })
        .execute()
        .await
        .unwrap();

    let views = client
        .select::<Post>()
        .columns((
            Post::id,
            Post::title,
            Post::content,
            JsonbBuildObject((User::id, User::name)).alias(PostView::author),
            Select::from::<Comment>()
                .columns(JsonbAgg(JsonbBuildObject((
                    Comment::id,
                    Comment::content,
                    JsonbBuildObject((User::id, User::name)).alias(CommentDto::commenter),
                ))))
                .inner_join::<User>(Comment::user_id.eq(User::id))
                .filter(Comment::post_id.eq(Post::id))
                .alias(PostView::comments),
        ))
        .inner_join::<User>(Post::author_id.eq(User::id))
        .load_as::<PostView>()
        .await
        .unwrap();

    assert_eq!(views.len(), 1);
    assert_eq!(views[0].author, Author { id: u1, name: "Alice".into() });
    assert_eq!(
        views[0].comments,
        vec![CommentDto {
            id: 1,
            content: "Works seamlessly!".into(),
            commenter: Author { id: u2, name: "Bob".into() },
        }]
    );
}

#[cfg(feature = "narrow-cast")]
#[tokio::test]
async fn pg_narrow_cast_runtime_bounds() {
    let client = runtime_client().await;

    // 1. Valid narrowing roundtrip
    let ok_row = client
        .statement("SELECT $1::int2, $2::int2, $3::int4, $4::int8")
        .bind((100_i8, 200_u8, 50000_u16, 1024_usize))
        .load_in::<(i8, u8, u16, usize)>()
        .await
        .unwrap();
    assert_eq!(ok_row, vec![(100, 200, 50000, 1024)]);

    // 2. Encoding u64::MAX into Int8 overflows and returns Error::ConversionError
    let enc_err = client
        .statement("SELECT $1::int8")
        .bind(u64::MAX)
        .load_in::<i64>()
        .await
        .unwrap_err();
    assert!(matches!(enc_err, Error::ConversionError(_)));

    // 3. Decoding Int2(300) into i8 overflows [-128, 127] and returns Error::ParseError
    let dec_err = client
        .statement("SELECT 300::int2")
        .load_in::<i8>()
        .await
        .unwrap_err();
    assert!(matches!(dec_err, Error::ParseError(_)));
}