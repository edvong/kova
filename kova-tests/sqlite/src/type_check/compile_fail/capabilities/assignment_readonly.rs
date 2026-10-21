use kova::Options;
use kova::dsl::Column;
use kova::dsl::func::{Count, JsonEach, Lower, Wildcard};
use kova::types::IdentifierMeta;
use models::type_check::event_log::EventLog;
use models::type_check::post_view::PostView;
use models::type_check::user::User;
use sqlite::type_check::codec::AppCodec;

const VIRTUAL_COL: Column<i32, IdentifierMeta> = Column::identifier("cte", "id");

fn main() {
    let client = Options::in_memory().build().unwrap().codec::<AppCodec>();

    // 1. Virtual / CTE / View columns (IdentifierMeta) cannot be assigned or excluded
    let _ = client.update::<User>().set(VIRTUAL_COL.set(1));
    let _ = client.update::<User>().set(VIRTUAL_COL.increment(1));
    let _ = client.update::<User>().set(VIRTUAL_COL.decrement(1));
    let _ = client.update::<User>().set(PostView::author.excluded());

    // 2. SRF virtual columns (JsonEach::key / JsonEach::value) cannot be assigned
    let _ = client.update::<User>().set(JsonEach::key.set("k"));
    let _ = client.update::<User>().set(JsonEach::value.set(1));

    // 3. Scalar functions, aggregates, arithmetic, JSON extract, and FTS5 nodes are read-only
    let _ = client.update::<User>().set(Lower(User::name).set("alice"));
    let _ = client.update::<User>().set(User::id.add(1).set(10));
    let _ = client.update::<User>().set(Count(Wildcard).set(1_i64));
    let _ = client
        .update::<EventLog>()
        .set(EventLog::metadata.json_extract("$.k").set("v"));
    let _ = client
        .update::<EventLog>()
        .set(EventLog::description.bm25().set(1.0_f64));
}