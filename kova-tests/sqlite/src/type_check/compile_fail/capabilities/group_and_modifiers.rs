use kova::Options;
use kova::dsl::func::{
    Abs, Count, GroupConcat, JsonGroupArray, JsonObject, JsonbGroupArray, JsonbObject, Lower, Sum,
    Wildcard, Window,
};
use models::type_check::event_log::EventLog;
use models::type_check::order::Order;
use models::type_check::user::User;
use sqlite::type_check::codec::AppCodec;

fn main() {
    let client = Options::in_memory().build().unwrap().codec::<AppCodec>();

    // 1. Aggregates and Scalars wrapping Aggregates cannot be in GROUP BY
    let _ = client.select::<Order>().group_by(Count(Wildcard));
    let _ = client.select::<Order>().group_by(Sum(Order::amount));
    let _ = client.select::<User>().group_by(GroupConcat(User::name));
    let _ = client.select::<User>().group_by(JsonGroupArray(User::id));
    let _ = client.select::<User>().group_by(JsonbGroupArray(User::id));
    let _ = client.select::<Order>().group_by(Abs(Sum(Order::amount)));

    // 2. JsonObject, JsonbObject, and FTS5 Bm25 cannot be in GROUP BY (and JsonObject cannot be in ORDER BY)
    let _ = client
        .select::<User>()
        .group_by(JsonObject((User::id, User::name)));
    let _ = client
        .select::<User>()
        .group_by(JsonbObject((User::id, User::name)));
    let _ = client
        .select::<EventLog>()
        .group_by(EventLog::description.bm25());
    let _ = client
        .select::<User>()
        .order_by(JsonObject((User::id, User::name)));

    // 3. Aggregates cannot be used inside Window::partition_by
    let _ = client.select::<Order>().columns(
        Count(Order::id).over(Window().partition_by(Sum(Order::amount))),
    );
    let _ = client.select::<User>().columns(
        Count(User::id).over(Window().partition_by(GroupConcat(User::name))),
    );

    // 4. Plain columns, scalar functions, and FTS5 functions do not support .over(), .distinct(), .filter(), .order_by()
    let _ = client.select::<User>().columns(User::id.over(Window()));
    let _ = client.select::<User>().columns(Lower(User::name).over(Window()));
    let _ = client
        .select::<EventLog>()
        .columns(EventLog::description.bm25().over(Window()));
    let _ = client.select::<User>().columns(User::id.distinct());
    let _ = client.select::<User>().columns(Lower(User::name).filter(User::is_active.eq(true)));
    let _ = client.select::<User>().columns(User::id.order_by(User::id.asc()));
}