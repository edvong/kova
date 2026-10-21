use crate::Result;
use crate::type_check::codec::AppCodec;
use crate::type_check::codec_post_view::ViewCodec;
use kova::Options;
use kova::dsl::func::{
    All, Any, ArrayAgg, JsonbAgg, JsonbBuildObject, JsonbObject, JsonbObjectSingle, Lower,
    RowToJson,
};
use kova::query::Select;
use models::type_check::comment::Comment;
use models::type_check::event_log::EventLog;
use models::type_check::order::Order;
use models::type_check::post::Post;
use models::type_check::post_view::{CommentDto, PostView};
use models::type_check::user::User;
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

/// Stress Test 1: Complex JSONB Aggregation, JsonbBuildObject, JsonbObject & RowToJson
async fn compile_check_complex_jsonb_subquery() -> Result<()> {
    let client = Options::from("").build()?.codec::<ViewCodec>();

    struct T1;

    let _views = client
        .select::<Post>()
        .columns((
            RowToJson(User),
            RowToJson(User.tag::<T1>()),
            // Memo: Testing 2-arg zip JsonbObject(keys, values) and 1-arg JsonbObjectSingle(kv_array)
            JsonbObject(Post::tags, ArrayAgg(User::name)),
            JsonbObjectSingle(Post::tags),
        ))
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
        .await?;

    Ok(())
}

/// Stress Test 2: JSONB Chain & Terminal JSON Operators
async fn compile_check_jsonb_operators() -> Result<()> {
    let client = Options::from("").build()?.codec::<AppCodec>();

    let _ = client
        .select::<Post>()
        .inner_join::<User>(Post::author_id.eq(User::id))
        .filter(
            JsonbBuildObject((User::id, User::name)).json_path_text("name").eq("alice")
                & JsonbObjectSingle(Post::tags).json_has_key("rust"),
        )
        .group_by(Post::id)
        .having(JsonbAgg(User::name).json_contains("[\"alice\"]"));

    let _ = client
        .select::<EventLog>()
        .filter(
            // 1. Deep -> chaining + @> (json_contains)
            EventLog::metadata.json_path("user").json_path("profile").json_contains(HashMap::new())
                // 2. #> (json_path_array) + ? (json_has_key)
                | EventLog::metadata
                .json_path_array(vec!["deep", "node"])
                .json_has_key("target_key")
                // 3. || (json_concat) + <@ (json_contained_by)
                | EventLog::metadata
                .json_concat(HashMap::new())
                .json_contained_by(HashMap::new())
                // 4. - (json_delete) + ?| (json_has_any_keys)
                | EventLog::metadata
                .json_delete("temp")
                .json_has_any_keys(vec!["k1".to_string(), "k2".to_string()])
                // 5. #- (json_delete_path) + ?& (json_has_all_keys)
                | EventLog::metadata
                .json_delete_path(vec!["nested".to_string(), "old".to_string()])
                .json_has_all_keys(vec!["k1".to_string()])
                // 6. @? (json_path_exists) & @@ (json_path_predicate)
                | EventLog::metadata.json_path_exists("$.user ? (@.age > 18)")
                | EventLog::metadata.json_path_predicate("$.active == true"),
        )
        .load_as::<EventLog>()
        .await?;

    Ok(())
}

/// Stress Test 3: JSONB Text Extraction, PG Regex/ILike & Array Operators
async fn compile_check_jsonb_text_and_pg_operators() -> Result<()> {
    let client = Options::from("").build()?.codec::<AppCodec>();

    let _ = client
        .select::<EventLog>()
        .filter(
            // 1. ->> (json_path_text) + standard eq / in_list / is_not_null
            EventLog::metadata.json_path_text("status").eq("active")
                | EventLog::metadata.json_path_text("role").in_list(vec!["admin", "user"])
                | EventLog::metadata.json_path_text("deleted_at").is_not_null()
                // 2. PG String operators (ilike, not_ilike, matches, not_matches, imatches, not_imatches)
                | EventLog::metadata.json_path_text("status").ilike("act%")
                | EventLog::description.not_ilike("%spam%")
                | Lower(EventLog::description).imatches("^event_[0-9]+")
                | EventLog::description.not_imatches("^ignore_")
                | EventLog::metadata
                .json_path_text_array(vec!["user".to_string(), "email".to_string()])
                .matches("^[a-z]+@")
                | EventLog::description.not_matches("[0-9]{10}"),
        )
        .load_as::<EventLog>()
        .await?;

    // 3. PG Array operators (@>, <@, &&) on Column<Vec<String>> and ArrayAgg
    let required_tags = vec!["rust".to_string(), "orm".to_string()];
    let _ = client
        .select::<Post>()
        .columns((Post::id, ArrayAgg(Post::title)))
        .filter(
            Post::tags.array_contains(&required_tags)
                & Post::tags.array_contained_by(&required_tags)
                & Post::tags.array_overlap(required_tags.clone()),
        )
        .group_by(Post::id)
        .having(ArrayAgg(Post::title).array_contains(&required_tags))
        .load_as::<Post>()
        .await?;

    Ok(())
}

/// Stress Test 4: ANY / ALL Predicates with Arrays, Slices, and Subqueries
async fn compile_check_any_all_and_subselect() -> Result<()> {
    let client = Options::from("").build()?.codec::<AppCodec>();

    let id_vec = vec![1, 2, 3];
    let id_slice: &[i32] = &[4, 5, 6];

    let _ = client
        .select::<User>()
        .filter(
            User::id.eq(Any(id_vec))
                | User::id.ne(All(vec![9, 10]))
                | User::id.gt(Any(vec![0_i16])) // Widened i16 inside Any!
                | User::id.le(All(id_slice)),
        )
        .load_as::<User>()
        .await?;

    let uuids = vec![Uuid::new_v4(), Uuid::new_v4()];
    let urls = vec![Url::parse("https://github.com").unwrap()];

    let _ = client
        .select::<EventLog>()
        .filter(
            EventLog::id.eq(Any(&uuids))
                | EventLog::website.eq(Any(urls))
                | EventLog::website.eq(Any(["https://rust-lang.org"])), // Compatible &str inside Any!
        )
        .load_as::<EventLog>()
        .await?;

    let _ = client
        .select::<Order>()
        .filter(
            Order::user_id.eq(Any(
                Select::from::<User>().columns(User::id).filter(User::is_active.eq(true)),
            )) | Order::amount.gt(All(
                Select::from::<Order>().columns(Order::amount).filter(Order::id.eq(1)),
            )),
        )
        .load_as::<Order>()
        .await?;

    Ok(())
}