#![allow(unreachable_code, dead_code, unused)]

use crate::codec::conversion::AppCodec;
use crate::codec::models::EventLog;
use crate::models::comment::Comment;
use crate::models::order::Order;
use crate::models::post::Post;
use crate::models::user::User;
use crate::view::post_view::{CommentDto, PostView};
use crate::Result;
use kova::Options;
use kova::dsl::func::{GroupConcat, Json, JsonGroupArray, JsonObject, JsonbGroupArray, JsonbObject};
use kova::query::Select;

/// Stress Test 1: SQLite-exclusive JSON extractors and extended operators (MATCH, REGEXP)
fn compile_check_sqlite_json_and_operators() -> Result<()> {
    // Hook up AppCodec here so we don't trip over advanced types like Uuid, Url, or HashMap sitting inside EventLog.
    let client = Options::in_memory().build()?.codec::<AppCodec>();

    let _ = client
        .select::<EventLog>()
        .filter(
            // 1. Check chained comparison inference for json_extract.
            // By passing "$.status", eq("active") automatically infers V = &str.
            EventLog::metadata.json_extract("$.status").eq("active")
                // Verifying operand inference for other types (like i32).
                | EventLog::metadata.json_extract("$.code").gt(200_i32)
                // Ensuring parameterless unary operators work smoothly.
                | EventLog::metadata.json_extract("$.deleted_at").is_null()
        )
        // 2. Firing off SQLite FTS5 full-text search (MATCH) alongside regular expressions (REGEXP).
        .filter(
            EventLog::description.matches("database OR framework")
                | EventLog::description.regexp("^[a-zA-Z0-9_]+$")
        )
        .load_as::<EventLog>()?;

    Ok(())
}

/// Stress Test 2: SQLite-exclusive aggregate functions
fn compile_check_sqlite_aggregates() -> Result<()> {
    let client = Options::in_memory().build()?;

    let _ = client
        .select::<User>()
        .columns((
            User::is_active,
            // 1. GroupConcat: The strict typing system demands that expr implements IntoArg<Projection>.
            GroupConcat {
                expr: User::name,
                separator: Some(" | "),
            }
                .alias(User::name),
            // 2. JsonGroupArray: Wrapping up a single column and slapping an alias on it.
            JsonGroupArray(User::id).alias(Order::raw_data),
        ))
        .group_by(User::is_active)
        // Strict tuple unpacking constraint: SQLite insists on returning TEXT for both json_group_array and group_concat.
        // Because of this, the last two types in load_in absolutely MUST be String (or a type capable of deserializing from text).
        .load_in::<(bool, String, String)>()?;

    Ok(())
}

/// Stress Test 3: SQLite Exclusive Nested JSON & JSONB Aggregations
/// Memo: Showcases the orthogonal design of SQLite's json/jsonb functions.
/// - `json_*` returns standard JSON text.
/// - `jsonb_*` returns SQLite's internal binary JSON format (faster for internal processing).
/// - `Json(...)` is the boundary wrapper used ONLY in the final projection to ensure Rust receives parseable Text.
fn compile_check_sqlite_complex_json_subquery() -> Result<()> {
    let client = Options::in_memory().build()?.codec::<AppCodec>();

    let _views = client
        .select::<Post>()
        .columns((
            Post::id,
            Post::title,
            Post::content,

            // Memo: Scenario 1 - High-performance object assembly.
            // JsonbObject creates binary JSON. We wrap it with Json() at the very edge 
            // so Rust's serde_json gets standard Text instead of a raw SQLite Blob.
            Json(JsonbObject((User::id, User::name))).alias(PostView::author),

            Select::from::<Comment>()
                .columns(
                // Memo: Scenario 2 - Deeply nested polymorphic aggregation.
                // 1. Inner: JsonObject generates Text (purely internal, no Json() wrapper needed here).
                // 2. Middle: JsonbObject generates Blob, embedding the inner Text seamlessly.
                // 3. Outer: JsonbGroupArray aggregates all Blobs into a single binary array.
                // 4. Boundary: The outermost Json() wrapper safely downcasts the entire binary tree to Text.
                Json(JsonbGroupArray(
                    JsonbObject((
                        Comment::id,
                        Comment::content,
                        JsonObject((User::id, User::name)).alias(CommentDto::commenter),
                    ))
                ))
            )
                .inner_join::<User>(Comment::user_id.eq(User::id))
                .filter(Comment::post_id.eq(Post::id))
                .alias(PostView::comments),
        ))
        .inner_join::<User>(Post::author_id.eq(User::id))
        .filter(
            // Memo: Scenario 3 - Using extractors independently in WHERE clauses.
            // No Json() wrapper needed because we are not projecting to Rust!

            // Fast binary tree traversal using jsonb_extract
            EventLog::metadata.jsonb_extract("$.source").eq("api")
                // Standard text traversal using json_extract
                | EventLog::metadata.json_extract("$.status").eq("active")
        )
        // Since the outermost boundaries are wrapped in Json(), `load_as` will safely 
        // deserialize into `Vec<CommentDto>` and `Author` using standard Text buffers.
        .load_as::<PostView>()?;

    Ok(())
}

/// Stress Test 4: Exhaustive permutations of json and jsonb variants
/// Memo: Proves that every JSON/JSONB function can exist independently, 
/// without being forced into a Json() wrapper, and can project raw Blobs or Texts to Rust if desired.
fn compile_check_sqlite_json_combinations() -> Result<()> {
    let client = Options::in_memory().build()?.codec::<AppCodec>();

    let _raw_tuples = client
        .select::<User>()
        .columns((
            // Memo: Raw Extractor bounds
            EventLog::metadata.json_extract("$.foo"),          // -> returns Text (String)
            EventLog::metadata.jsonb_extract("$.bar"),         // -> returns Blob (Vec<u8>)

            // Memo: Raw Object bounds
            JsonObject((User::id, User::name)),                // -> returns Text (String)
            JsonbObject((User::id, User::name)),               // -> returns Blob (Vec<u8>)

            // Memo: Raw Array bounds
            JsonGroupArray(User::id),                          // -> returns Text (String)
            JsonbGroupArray(User::name),                       // -> returns Blob (Vec<u8>)

            // Memo: Safe boundary wrappers explicitly forcing Text output
            Json(EventLog::metadata.jsonb_extract("$.baz")),   // -> forced to Text (String)
            Json(JsonbObject((User::id, User::name))),         // -> forced to Text (String)
            Json(JsonbGroupArray(User::id)),                   // -> forced to Text (String)
        ))
        // Memo: The target tuple explicitly asserts the return types generated by the AST.
        // If kova's type mapping works correctly, Jsonb variants MUST map to Blob (Vec<u8>), 
        // while Json and Json(...) variants MUST map to Text (String).
        .load_in::<(
            String, Vec<u8>,
            String, Vec<u8>,
            String, Vec<u8>,
            String, String, String
        )>()?;

    Ok(())
}