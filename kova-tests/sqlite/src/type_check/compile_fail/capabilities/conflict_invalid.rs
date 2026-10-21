use kova::Options;
use kova::dsl::Column;
use kova::dsl::func::{Count, GroupConcat, JsonEach, JsonObject, Lower, Wildcard};
use kova::types::IdentifierMeta;
use models::type_check::event_log::EventLog;
use models::type_check::user::User;

const VIRTUAL_NAME: Column<String, IdentifierMeta> = Column::identifier("cte", "name");

fn main() {
    let client = Options::in_memory().build().unwrap();
    let user = User::default();

    // 1. Aggregate functions cannot be used as ON CONFLICT targets
    let _ = client.insert(&user).on_conflict(Count(Wildcard));
    let _ = client.insert(&user).on_conflict(GroupConcat(User::name));

    // 2. Virtual columns (IdentifierMeta) and SRF columns cannot be used as ON CONFLICT targets
    let _ = client.insert(&user).on_conflict(VIRTUAL_NAME);
    let _ = client.insert(&user).on_conflict(JsonEach::key);
    let _ = client.insert(&user).on_conflict(JsonEach::value);

    // 3. Scalar function wrapping a non-physical virtual column must also fail recursively
    let _ = client.insert(&user).on_conflict(Lower(VIRTUAL_NAME));

    // 4. Whole table identifier (`Table<User>`), `JsonObject`, and FTS5 `bm25()` cannot be ON CONFLICT targets
    let _ = client.insert(&user).on_conflict(User);
    let _ = client
        .insert(&user)
        .on_conflict(JsonObject((User::id, User::name)));
    let _ = client
        .insert(&user)
        .on_conflict(EventLog::description.bm25());
}