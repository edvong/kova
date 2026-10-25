use crate::runtime::codec::{AppCodec, ViewCodec};
use crate::runtime::runtime_client;
use chrono::{TimeZone, Utc};
use jiff::Timestamp;
use kova::Error;
use kova::codec::{Compatible, Encode, Static, Value};
use kova::dsl::func::{
    Abs, Avg, Json, JsonObject, JsonbGroupArray, JsonbObject, Sum,
};
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

// Custom type that intentionally fails during `Encode` to test `OperandNode::Error` propagation.
struct FailingParam;
impl Compatible<Static<String>, AppCodec> for Static<FailingParam> {}
impl<'a> Encode<'a, AppCodec> for FailingParam {
    fn encode(self) -> kova::Result<Value<'a>> {
        Err(Error::ValidationError("failing param rejected in encode()".into()))
    }
}

#[test]
fn primitive_codecs_zero_copy_slices_and_error_boundaries() {
    let client = runtime_client();

    let payload: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    let order = Order {
        id: 0,
        user_id: 42,
        amount: 199.95,
        discount: None,
        is_active: true,
        raw_data: payload.to_vec(),
    };
    client.insert(&order).execute().unwrap();

    // 1. Box<T>, Option<T>, and fixed-size byte array `[u8; 4]` extraction
    let row = client
        .select::<Order>()
        .columns((
            Order::user_id,
            Order::amount,
            Order::discount,
            Order::is_active,
            Order::raw_data,
        ))
        .load_in::<(Box<i32>, f64, Option<f64>, bool, [u8; 4])>()
        .unwrap();
    assert_eq!(row, vec![(Box::new(42), 199.95, None, true, payload)]);

    // 2. `[u8; N]` length mismatch returns `Error::ParseError`
    let len_err = client
        .select::<Order>()
        .columns(Order::raw_data)
        .load_in::<[u8; 8]>()
        .unwrap_err();
    assert!(matches!(len_err, Error::ParseError(_)));

    // 3. Reading SQL NULL into a non-nullable type returns `Error::UnexpectedNull`
    let null_err = client
        .select::<Order>()
        .columns(Order::discount)
        .load_in::<f64>()
        .unwrap_err();
    assert!(matches!(null_err, Error::UnexpectedNull(_)));

    // 4. Storage class mismatch returns `Error::TypeMismatch`
    let type_err = client
        .select::<Order>()
        .columns(Order::user_id)
        .load_in::<String>()
        .unwrap_err();
    assert!(matches!(type_err, Error::TypeMismatch(_)));

    // 5. Out-of-range i64 -> i32 extraction returns `Error::ParseError`
    let i32_overflow = client
        .statement("SELECT ?")
        .bind(i64::MAX)
        .load_in::<i32>()
        .unwrap_err();
    assert!(matches!(i32_overflow, Error::ParseError(_)));
}

#[test]
fn operand_encode_error_deferred_and_propagated_on_build() {
    let client = runtime_client().codec::<AppCodec>();

    // `FailingParam` fails inside `IntoOperand::into_operand`, storing `OperandNode::Error`.
    // Rendering the AST during `.load()` clones and returns the underlying `ValidationError`.
    let err = client
        .select::<User>()
        .filter(User::name.eq(FailingParam))
        .load()
        .unwrap_err();
    assert!(
        matches!(err, Error::ValidationError(ref msg) if msg.contains("failing param rejected")),
        "unexpected error: {err:?}"
    );
}

#[test]
fn custom_app_codec_and_domain_capabilities_roundtrip() {
    let client = runtime_client().codec::<AppCodec>();

    let id = Uuid::new_v4();
    let created = Utc.with_ymd_and_hms(2026, 7, 31, 12, 0, 0).unwrap();
    let updated = Timestamp::from_str("2026-07-31T12:30:00Z").unwrap();
    let url = Url::parse("https://rust-lang.org/learn").unwrap();
    let mut meta = HashMap::new();
    meta.insert("env".to_string(), "prod".to_string());
    let labels = TagList(vec!["sqlite".into(), "json".into()]);

    let log = EventLog {
        id,
        description: "bootstrapped".into(),
        created_at: created,
        updated_at: updated,
        website: url,
        metadata: meta.clone(),
        cost: Decimal(500),
        payload: AuditPayload { actor: "init".into(), level: 1 },
        labels,
    };

    client.insert(&log).execute().unwrap();

    let payload = AuditPayload { actor: "admin".into(), level: 9 };
    client
        .update::<EventLog>()
        .set((
            EventLog::cost.increment(250_i64),
            EventLog::payload.set(&payload),
            EventLog::website.set("https://crates.io"),
        ))
        .filter(EventLog::id.eq(&id))
        .execute()
        .unwrap();

    let loaded = client
        .select::<EventLog>()
        .filter(
            EventLog::id.eq(&id)
                & EventLog::cost.add(50_i64).eq(Decimal(800))
                & EventLog::payload.json_extract("$.actor").eq("admin"),
        )
        .load()
        .unwrap();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].id, id);
    assert_eq!(loaded[0].website.as_str(), "https://crates.io/");
    assert_eq!(loaded[0].metadata, meta);
    assert_eq!(loaded[0].cost, Decimal(750));
    assert_eq!(loaded[0].payload, payload);

    let math_res = client
        .select::<EventLog>()
        .columns((
            EventLog::cost.sub(100_i64),
            Sum(EventLog::cost),
            Avg(EventLog::cost),
            Abs(EventLog::cost.sub(1000_i64)),
        ))
        .load_in::<(Decimal, Decimal, Decimal, Decimal)>()
        .unwrap();
    assert_eq!(
        math_res,
        vec![(Decimal(650), Decimal(750), Decimal(750), Decimal(250))]
    );
}

#[test]
fn view_codec_nested_json_subquery_mapping() {
    let client = runtime_client().codec::<ViewCodec>();

    let u1 = client
        .insert(User { id: 0, name: "AuthorA".into(), bio: None, balance: 0.0, is_active: true })
        .returning(User::id)
        .execute_in::<i32>()
        .unwrap()[0];
    let u2 = client
        .insert(User { id: 0, name: "CommenterB".into(), bio: None, balance: 0.0, is_active: true })
        .returning(User::id)
        .execute_in::<i32>()
        .unwrap()[0];

    let p1 = client.clone().codec::<AppCodec>()
        .insert(Post {
            id: 0,
            title: "Kova ORM".into(),
            content: "SQLite JSONB test".into(),
            author_id: u1,
        })
        .returning(Post::id)
        .execute_in::<i32>()
        .unwrap()[0];

    client
        .insert(Comment {
            id: 0,
            post_id: p1,
            user_id: u2,
            content: "Great design!".into(),
        })
        .execute()
        .unwrap();

    let views = client.clone()
        .select::<Post>()
        .columns((
            Post::id,
            Post::title,
            Post::content,
            Json(JsonbObject((User::id, User::name))).alias(PostView::author),
            Select::from::<Comment>()
                .columns(Json(JsonbGroupArray(JsonbObject((
                    Comment::id,
                    Comment::content,
                    JsonObject((User::id, User::name)).alias(CommentDto::commenter),
                )))))
                .inner_join::<User>(Comment::user_id.eq(User::id))
                .filter(Comment::post_id.eq(Post::id))
                .alias(PostView::comments),
        ))
        .inner_join::<User>(Post::author_id.eq(User::id))
        .load_as::<PostView>()
        .unwrap();

    assert_eq!(views.len(), 1);
    assert_eq!(views[0].title, "Kova ORM");
    assert_eq!(views[0].author, Author { id: u1, name: "AuthorA".into() });
    assert_eq!(
        views[0].comments,
        vec![CommentDto {
            id: 1,
            content: "Great design!".into(),
            commenter: Author { id: u2, name: "CommenterB".into() },
        }]
    );
}

#[cfg(feature = "narrow-cast")]
#[test]
fn narrow_cast_feature_runtime_bounds() {
    let client = runtime_client();

    // 1. Valid narrowing roundtrip
    let row = client
        .statement("SELECT ?, ?, ?, ?")
        .bind((120_i8, 250_u8, 60000_u16, 1024_usize))
        .load_in::<(i8, u8, u16, usize)>()
        .unwrap();
    assert_eq!(row, vec![(120, 250, 60000, 1024)]);

    // 2. Encoding u64::MAX into SQLite i64 overflows and returns Error::ConversionError
    let enc_err = client
        .statement("SELECT ?")
        .bind(u64::MAX)
        .load_in::<i64>()
        .unwrap_err();
    assert!(matches!(enc_err, Error::ConversionError(_)));

    // 3. Decoding 200 into i8 overflows [-128, 127] and returns Error::ParseError
    let dec_err = client
        .statement("SELECT 200")
        .load_in::<i8>()
        .unwrap_err();
    assert!(matches!(dec_err, Error::ParseError(_)));
}