use kova::Options;
use kova::dsl::func::{Json, JsonEach, JsonObject, JsonTree, JsonbObject, Lower};
use models::type_check::event_log::EventLog;
use models::type_check::order::Order;
use models::type_check::user::User;
use sqlite::type_check::codec::AppCodec;

fn main() {
    let client = Options::in_memory().build().unwrap().codec::<AppCodec>();

    // 1. Numeric/boolean/UUID columns do NOT implement `CanSqliteJson`
    let _ = client.select::<User>().filter(User::id.json_extract("$.a").eq(1));
    let _ = client.select::<Order>().filter(Order::amount.jsonb_extract("$.a").eq(1));
    let _ = client.select::<EventLog>().filter(EventLog::id.json_extract("$.a").eq(1));
    let _ = client.select::<User>().columns(Json(User::id));
    let _ = client.select::<User>().srf(JsonEach(User::id));
    let _ = client.select::<Order>().srf(JsonTree(Order::is_active));

    // 2. json_extract / jsonb_extract and 2-arg JsonEach / JsonTree require `Path` compatible with `Static<String>`
    let _ = client
        .select::<EventLog>()
        .filter(EventLog::metadata.json_extract(123_i32).eq(1));
    let _ = client
        .select::<EventLog>()
        .filter(EventLog::metadata.jsonb_extract(true).eq(1));
    let _ = client.select::<EventLog>().srf(JsonEach((EventLog::metadata, 123_i32)));
    let _ = client.select::<EventLog>().srf(JsonTree((EventLog::metadata, true)));

    // 3. JsonObject / JsonbObject require valid Projection arguments (rejecting raw literals)
    let _ = client.select::<User>().columns(JsonObject(123_i32));
    let _ = client.select::<User>().columns(JsonbObject("raw_str"));

    // 4. FTS5 capabilities (`CanSqliteFts`) are only implemented for textual `Column`s,
    //    rejecting numeric/UUID columns and composite scalar expressions (`Lower(...)`)
    let _ = client.select::<User>().filter(User::id.matches("fts_query"));
    let _ = client.select::<EventLog>().filter(EventLog::id.matches("fts_query"));
    let _ = client.select::<User>().columns(User::id.bm25());
    let _ = client
        .select::<EventLog>()
        .columns(Lower(EventLog::description).bm25());
    let _ = client
        .select::<EventLog>()
        .columns(Lower(EventLog::description).highlight(0, "<b>", "</b>"));
    let _ = client
        .select::<EventLog>()
        .columns(Lower(EventLog::description).snippet(0, "<b>", "</b>", "...", 16));

    // 5. FTS5 highlight and snippet marker arguments must be compatible with `Static<String>`
    let _ = client
        .select::<EventLog>()
        .columns(EventLog::description.highlight(0, 123_i32, "</b>"));
    let _ = client
        .select::<EventLog>()
        .columns(EventLog::description.snippet(0, "<b>", "</b>", 999_i32, 16));
}