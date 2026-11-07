#![allow(unused)]

use crate::render::{capture_one, render_client};
use crate::runtime::codec::AppCodec;
use kova::{Error, Model};
use kova::dsl::Column;
use kova::dsl::func::{
    All, Any, ArrayAgg, GenerateSeries, JsonbAgg, JsonbArrayElements, JsonbBuildObject, JsonbEach,
    JsonbEachText, JsonbObject, JsonbObjectSingle, Lower, RowToJson, Unnest,
};
use kova::query::Select;
use kova::types::IdentifierMeta;
use models::runtime::event_log::EventLog;
use models::runtime::order::Order;
use models::runtime::user::User;

struct TagSrf;
struct TagSrf2;
struct TagTbl;

#[derive(Model)]
struct Post {
    #[kova(auto)]
    id: i32,
    title: String,
    content: String,
    author_id: i32,
    tags: Vec<String>,
}

#[tokio::test]
async fn pg_jsonb_delete_operator_precedence_and_chains() {
    let client = render_client().await.codec::<AppCodec>();

    // Verifies `ExtFunction::JsonChain` precedence rules in PostgreSQL:
    // 1. Plain column `.json_delete("temp")` is not parenthesized: `"event_log"."metadata" - $1`
    // 2. Composite base `.json_path("user").json_delete("secret")` IS parenthesized: `("event_log"."metadata" -> $3) - $4`
    let cap = capture_one(
        client
            .select::<EventLog>()
            .filter(
                EventLog::metadata.json_delete("temp").json_has_key("k1")
                    & EventLog::metadata
                    .json_path("user")
                    .json_delete("secret")
                    .json_has_any_keys(["a", "b"])
                    & EventLog::metadata
                    .json_path_array(["deep", "node"])
                    .json_delete_path(["old", "field"])
                    .json_has_all_keys(["x", "y"])
                    & EventLog::metadata.json_path_exists("$.user ? (@.age > 18)")
                    & EventLog::metadata.json_path_predicate("$.active == true"),
            )
            .load(),
    )
        .await;

    assert_eq!(
        cap.sql,
        r#"select * from "event_log" where "event_log"."metadata" - $1 ? $2 and ("event_log"."metadata" -> $3) - $4 ?| array[$5, $6] and "event_log"."metadata" #> array[$7, $8] #- array[$9, $10] ?& array[$11, $12] and "event_log"."metadata" @? $13 and "event_log"."metadata" @@ $14"#
    );
}

#[tokio::test]
async fn pg_jsonb_constructors_row_to_json_and_missing_alias_error() {
    let client = render_client().await.codec::<AppCodec>();
    const QUOTED_KEY: Column<String, IdentifierMeta> = Column::identifier("user", "k'ey");

    // 1. Single-quote escaping in `JsonbBuildObject`, `JsonbObject` (2-arg and 1-arg), and `RowToJson(Table)`.
    let cap = capture_one(
        client
            .select::<Post>()
            .inner_join::<User>(Post::author_id.eq(User::id.tag::<TagTbl>()))
            .tag::<TagTbl>()
            .columns((
                RowToJson(Post),
                RowToJson(User.tag::<TagTbl>()),
                JsonbBuildObject((
                    Post::id,
                    Lower(User::name.tag::<TagTbl>()).alias(QUOTED_KEY),
                )),
                JsonbObject(Post::tags, ArrayAgg(User::name.tag::<TagTbl>())),
                JsonbObjectSingle(Post::tags),
                // "DISTINCT colA ORDER BY colB" Not Allow in Postgres.
                // JsonbAgg(Post::title).distinct().order_by(Post::id.desc()),
                JsonbAgg(Post::title).distinct().order_by(Post::title.desc()),
            ))
            .group_by((Post::id, Post::tags, User::id.tag::<TagTbl>()))
            .load_in::<(String, String, String, String, String, String)>(),
    )
        .await;

    assert_eq!(
        cap.sql,
        r#"select row_to_json("post"), row_to_json("user_0"), jsonb_build_object('id', "post"."id", 'k''ey', lower("user_0"."name")), jsonb_object("post"."tags", array_agg("user_0"."name")), jsonb_object("post"."tags"), coalesce(jsonb_agg(distinct "post"."title" order by "post"."title" desc), '[]'::jsonb) from "post" inner join "user" as "user_0" on "post"."author_id" = "user_0"."id" group by "post"."id", "post"."tags", "user_0"."id""#
    );

    // 2. Passing an unaliased non-column expression to `JsonbBuildObject` returns `Error::QueryError`.
    let err = client
        .select::<User>()
        .columns(JsonbBuildObject(Lower(User::name)))
        .load_in::<String>()
        .await
        .unwrap_err();
    assert!(
        matches!(err, Error::QueryError(ref msg) if msg.contains("jsonb_build_object argument requires a column or .alias()")),
        "unexpected error: {err:?}"
    );
}

#[tokio::test]
async fn pg_any_all_array_and_regex_operators() {
    let client = render_client().await.codec::<AppCodec>();
    let tags = vec!["rust".to_string(), "sql".to_string()];

    let cap = capture_one(
        client
            .select::<Post>()
            .filter(
                Post::id.eq(Any([1_i32, 2, 3]))
                    & Post::id.ne(All(
                    Select::from::<Order>()
                        .columns(Order::user_id)
                        .filter(Order::is_active.eq(true)),
                ))
                    & Post::tags.array_contains(&tags)
                    & Post::tags.array_contained_by(&tags)
                    & Post::tags.array_overlap(&tags)
                    & Post::title.ilike("kova%")
                    & Post::title.not_ilike("%deprecated%")
                    & Post::title.matches("^Kova[0-9]+")
                    & Post::title.not_matches("bad")
                    & Post::title.imatches("orm")
                    & Post::title.not_imatches("legacy"),
            )
            .load(),
    )
        .await;

    // `All(Select::...)` uses `render_bare` to emit `all(select ...)` instead of `all((select ...))`.
    assert_eq!(
        cap.sql,
        r#"select * from "post" where "post"."id" = any(array[$1, $2, $3]) and "post"."id" != all(select "order"."user_id" from "order" where "order"."is_active" = $4) and "post"."tags" @> $5 and "post"."tags" <@ $6 and "post"."tags" && $7 and "post"."title" ilike $8 and "post"."title" not ilike $9 and "post"."title" ~ $10 and "post"."title" !~ $11 and "post"."title" ~* $12 and "post"."title" !~* $13"#
    );
}

#[tokio::test]
async fn pg_srf_functions_and_tagged_srf_aliases() {
    let client = render_client().await.codec::<AppCodec>();

    // Verifies `push_srf_alias`:
    // - `Unnest` and `GenerateSeries` have `explicit_col`, emitting `as "generate_series_0"("generate_series")`
    // - `JsonbEachText` has no `explicit_col`, emitting `as "jsonb_each_text_0"`
    let cap_unnest = capture_one(
        client
            .select::<Post>()
            .srf((
                Unnest(Post::tags),
                GenerateSeries(1_i32, 10_i32).step(2_i32).tag::<TagSrf>(),
            ))
            .columns((Unnest::unnest, GenerateSeries::generate_series.tag::<TagSrf>()))
            .load_in::<(String, i32)>(),
    )
        .await;

    assert_eq!(
        cap_unnest.sql,
        r#"select "unnest"."unnest", "generate_series_0"."generate_series" from "post", unnest("post"."tags"), generate_series($1, $2, $3) as "generate_series_0"("generate_series")"#
    );

    let cap_json_srf = capture_one(
        client
            .select::<EventLog>()
            .srf((
                JsonbArrayElements(EventLog::metadata.json_path("items")),
                JsonbEach(EventLog::metadata).tag::<TagSrf>(),
                JsonbEachText(EventLog::metadata).tag::<TagSrf2>(),
            ))
            .columns((
                JsonbArrayElements::value,
                JsonbEachText::value.tag::<TagSrf2>(),
                JsonbEach::key.tag::<TagSrf>(),
            ))
            .load_in::<(String, String, String)>(),
    )
        .await;

    assert_eq!(
        cap_json_srf.sql,
        r#"select "jsonb_array_elements"."value", "jsonb_each_text_1"."value", "jsonb_each_0"."key" from "event_log", jsonb_array_elements("event_log"."metadata" -> $1), jsonb_each("event_log"."metadata") as "jsonb_each_0", jsonb_each_text("event_log"."metadata") as "jsonb_each_text_1""#
    );
}