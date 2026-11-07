use crate::render::{capture_one, render_client};
use crate::runtime::codec::AppCodec;
use kova::Error;
use kova::dsl::Identifier;
use kova::dsl::func::{
    Json, JsonEach, JsonGroupArray, JsonObject, JsonTree, JsonbGroupArray, JsonbObject, Lower,
};
use models::runtime::event_log::EventLog;
use models::runtime::user::User;

struct J1;
struct J2;

#[test]
fn sqlite_json_and_jsonb_functions_and_single_quote_escaping() {
    let client = render_client().codec::<AppCodec>();
    let quoted_alias: Identifier<String> = Identifier::new("a'b");

    let cap = capture_one(|| {
        client
            .select::<User>()
            .columns((
                JsonObject((User::id, Lower(User::name).alias(quoted_alias))),
                Json(JsonbObject((User::id, User::name))),
                JsonGroupArray(User::id).distinct().filter(User::is_active.eq(true)),
                Json(JsonbGroupArray(User::name)),
            ))
            .filter(
                User::name.json_extract("$.role").eq("admin")
                    & User::name.jsonb_extract("$.level").gt(1_i32),
            )
            .load_in::<(String, String, String, String)>()
    });

    assert_eq!(
        cap.sql,
        r#"select json_object('id', "user"."id", 'a''b', lower("user"."name")), json(jsonb_object('id', "user"."id", 'name', "user"."name")), json_group_array(distinct "user"."id") filter (where "user"."is_active" = ?), json(jsonb_group_array("user"."name")) from "user" where json_extract("user"."name", ?) = ? and jsonb_extract("user"."name", ?) > ?"#
    );
    assert_eq!(
        cap.params,
        r#"[Int8(1), Text("$.role"), Text("admin"), Text("$.level"), Int8(1)]"#
    );
}

#[test]
fn sqlite_json_object_without_column_or_alias_returns_query_error() {
    let client = render_client();

    let err = client
        .select::<User>()
        .columns(JsonObject(Lower(User::name)))
        .load_in::<String>()
        .unwrap_err();

    assert!(
        matches!(err, Error::QueryError(_)),
        "expected Error::QueryError when passing unaliased expression to JsonObject, got: {err:?}"
    );
}

#[test]
fn sqlite_fts5_and_srf_rendering() {
    let client = render_client().codec::<AppCodec>();

    // Note: FTS5 virtual tables in SQLite require matching the unaliased table name in `bm25("event_log")`.
    // We tag the trailing SRF (`JsonTree`) with `J2` to verify SRF aliasing alongside FTS5 functions.
    let cap = capture_one(|| {
        client
            .select::<EventLog>()
            .srf((
                JsonEach(EventLog::metadata).tag::<J1>(),
                JsonTree((EventLog::metadata, "$.nested")).tag::<J2>(),
            ))
            .columns((
                EventLog::description.bm25().weights([10.0, 2.5]),
                EventLog::description.highlight(1, "<b>", "</b>"),
                EventLog::description.snippet(1, "[", "]", "...", 32),
                JsonEach::key.tag::<J1>(),
                JsonTree::value.tag::<J2>(),
            ))
            .filter(
                EventLog::description.matches("kova AND sqlite")
                    & EventLog::description.regexp("^kova"),
            )
            .load_in::<(f64, String, String, String, String)>()
    });

    assert_eq!(
        cap.sql,
        r#"select bm25("event_log", 10, 2.5), highlight("event_log", 1, ?, ?), snippet("event_log", 1, ?, ?, ?, 32), "json_each_0"."key", "json_tree_1"."value" from "event_log", json_each("event_log"."metadata") as "json_each_0", json_tree("event_log"."metadata", ?) as "json_tree_1" where "event_log"."description" match ? and "event_log"."description" regexp ?"#
    );
    assert_eq!(
        cap.params,
        r#"[Text("<b>"), Text("</b>"), Text("["), Text("]"), Text("..."), Text("$.nested"), Text("kova AND sqlite"), Text("^kova")]"#
    );
}