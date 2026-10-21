use crate::Result;
use crate::type_check::codec::AppCodec;
use kova::Options;
use kova::dsl::func::{
    GroupConcat, Json, JsonGroupArray, JsonObject, JsonbGroupArray, JsonbObject, Lower,
};
use kova::query::Select;
use models::type_check::comment::Comment;
use models::type_check::event_log::EventLog;
use models::type_check::order::Order;
use models::type_check::post::Post;
use models::type_check::post_view::{CommentDto, PostView};
use models::type_check::user::User;
use crate::type_check::codec_post_view::ViewCodec;

/// Stress Test 1: SQLite JSON Extractors, Chained Extraction & Extended Operators (MATCH, REGEXP)
fn compile_check_sqlite_json_and_operators() -> Result<()> {
    let client = Options::in_memory().build()?.codec::<AppCodec>();

    let _ = client
        .select::<EventLog>()
        .filter(
            // 1. Dynamic comparison inference for json_extract (&str, i32, bool, null)
            EventLog::metadata.json_extract("$.status").eq("active")
                | EventLog::metadata.json_extract("$.code").gt(200_i32)
                | EventLog::metadata.json_extract("$.deleted_at").is_null()
                // 2. Multi-step chained json_extract + arithmetic + comparison
                | EventLog::metadata
                .json_extract("$.user")
                .json_extract("$.score")
                .add(10)
                .ge(100)
                // 3. Using regexp on json_extract and Lower() scalar node
                | EventLog::metadata.json_extract("$.email").regexp("^[a-z]+@")
                | Lower(EventLog::description).regexp("^[a-z0-9_]+$"),
        )
        .filter(
            EventLog::description.matches("database OR framework")
                | EventLog::description.regexp("^[a-zA-Z0-9_]+$"),
        )
        .load_as::<EventLog>()?;

    Ok(())
}

/// Stress Test 2: SQLite-Exclusive Aggregate Functions & Modifiers
fn compile_check_sqlite_aggregates() -> Result<()> {
    let client = Options::in_memory().build()?;

    let _ = client
        .select::<User>()
        .columns((
            User::is_active,
            GroupConcat(User::name).separator(" | ").order_by(User::name.asc()).alias(User::name),
            JsonGroupArray(User::id).distinct().filter(User::id.gt(0)).alias(Order::raw_data),
        ))
        .group_by(User::is_active)
        .load_in::<(bool, String, String)>()?;

    Ok(())
}

/// Stress Test 3: Nested JSON & JSONB Subquery Aggregations
fn compile_check_sqlite_complex_json_subquery() -> Result<()> {
    let client = Options::in_memory().build()?.codec::<ViewCodec>();

    let _views = client
        .select::<Post>()
        .columns((
            Post::id,
            Post::title,
            Post::content,
            // Scenario 1: High-performance binary JSONB assembly converted to Text at projection edge
            Json(JsonbObject((User::id, User::name))).alias(PostView::author),
            // Scenario 2: Deeply nested polymorphic aggregation inside a correlated subquery
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
        .filter(
            EventLog::metadata.jsonb_extract("$.source").eq("api")
                | EventLog::metadata.json_extract("$.status").eq("active"),
        )
        .load_as::<PostView>()?;

    Ok(())
}

/// Stress Test 4: Exhaustive Permutations of JSON and JSONB Variants
fn compile_check_sqlite_json_combinations() -> Result<()> {
    let client = Options::in_memory().build()?.codec::<AppCodec>();

    let _raw_tuples = client
        .select::<User>()
        .columns(EventLog::metadata.json_extract("$.foo"))
        .columns(JsonObject((User::id, User::name)))
        .columns(Json(JsonbObject((User::id, User::name))))
        .columns(JsonGroupArray(User::id))
        .columns((
            EventLog::metadata.json_extract("$.foo"),
            EventLog::metadata.jsonb_extract("$.bar"),
            JsonObject((User::id, User::name)),
            JsonbObject((User::id, User::name)),
            JsonGroupArray(User::id),
            JsonbGroupArray(User::name),
            Json(EventLog::metadata.jsonb_extract("$.baz")),
            Json(JsonbObject((User::id, User::name))),
            Json(JsonbGroupArray(User::id)),
        ))
        .load_in::<(String, Vec<u8>, String, Vec<u8>, String, Vec<u8>, String, String, String)>()?;

    Ok(())
}
