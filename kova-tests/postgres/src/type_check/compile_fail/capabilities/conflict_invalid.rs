use kova::Options;
use kova::dsl::{Column, Constraint};
use kova::dsl::func::{Count, JsonbBuildObject, Lower, RowNumber, Unnest, Wildcard};
use kova::types::IdentifierMeta;
use models::type_check::event_log::EventLog;
use models::type_check::user::User;
use postgres::type_check::codec::AppCodec;

const VIRTUAL_NAME: Column<String, IdentifierMeta> = Column::identifier("cte", "name");

fn main() {
    let client = Options::from("").build().unwrap().codec::<AppCodec>();
    let user = User::default();

    // 1. Aggregate and Window functions cannot be used as ON CONFLICT targets
    let _ = client.insert(&user).on_conflict(Count(Wildcard));
    let _ = client.insert(&user).on_conflict(RowNumber());

    // 2. Virtual columns (IdentifierMeta) and SRF dynamic columns cannot be ON CONFLICT targets
    let _ = client.insert(&user).on_conflict(VIRTUAL_NAME);
    let _ = client.insert(&user).on_conflict(Unnest::unnest);

    // 3. Scalar function wrapping a non-physical virtual column must fail recursively
    let _ = client.insert(&user).on_conflict(Lower(VIRTUAL_NAME));

    // 4. Whole table identifier (`Table<User>`) and JSONB constructors/chains (`->`) cannot be ON CONFLICT targets
    let _ = client.insert(&user).on_conflict(User);
    let _ = client
        .insert(&user)
        .on_conflict(JsonbBuildObject((User::id, User::name)));
    let _ = client
        .insert(&user)
        .on_conflict(EventLog::metadata.json_path("key"));

    // 5. `Constraint` cannot be nested in a tuple or mixed with columns
    let _ = client.insert(&user).on_conflict((Constraint("uk_1"), Constraint("uk_2")));
    let _ = client.insert(&user).on_conflict((User::id, Constraint("uk_1")));

    // 6. Parameterized arithmetic or JSONB text extraction expressions cannot be ON CONFLICT targets
    let _ = client.insert(&user).on_conflict(User::id.add(1));
    let _ = client.insert(&user).on_conflict(EventLog::metadata.json_path_text("key"));
}