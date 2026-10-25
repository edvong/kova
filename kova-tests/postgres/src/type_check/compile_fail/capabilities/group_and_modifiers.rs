use kova::Options;
use kova::dsl::func::{
    Abs, ArrayAgg, Count, JsonbAgg, JsonbBuildObject, Lag, Lower, Ntile, RowNumber, RowToJson,
    Sum, Wildcard, Window,
};
use models::type_check::order::Order;
use models::type_check::user::User;

fn main() {
    let client = Options::from("").build().unwrap();

    // 1. Aggregates, Window functions, and Scalars wrapping Aggregates cannot be in GROUP BY
    let _ = client.select::<Order>().group_by(Count(Wildcard));
    let _ = client.select::<Order>().group_by(Sum(Order::amount));
    let _ = client.select::<Order>().group_by(ArrayAgg(Order::amount));
    let _ = client.select::<Order>().group_by(JsonbAgg(Order::amount));
    let _ = client.select::<Order>().group_by(RowNumber());
    let _ = client.select::<Order>().group_by(Lag(Order::amount));
    let _ = client.select::<Order>().group_by(Ntile(4));
    let _ = client.select::<Order>().group_by(Abs(Sum(Order::amount)));

    // 2. JSON object constructors and Table<T> cannot be used in GROUP BY or ORDER BY
    let _ = client
        .select::<User>()
        .group_by(JsonbBuildObject((User::id, User::name)));
    let _ = client.select::<User>().group_by(RowToJson(User));
    let _ = client
        .select::<User>()
        .order_by(JsonbBuildObject((User::id, User::name)));
    let _ = client.select::<User>().order_by(RowToJson(User));

    // 3. Aggregates and Window functions cannot be used inside Window::partition_by
    let _ = client.select::<Order>().columns(
        Count(Order::id).over(Window().partition_by(Sum(Order::amount))),
    );
    let _ = client.select::<Order>().columns(
        Count(Order::id).over(Window().partition_by(RowNumber())),
    );

    // 4. Plain columns and scalar functions do not support .over(), .distinct(), .filter(), .order_by()
    let _ = client.select::<User>().columns(User::id.over(Window()));
    let _ = client.select::<User>().columns(Lower(User::name).over(Window()));
    let _ = client.select::<User>().columns(User::id.distinct());
    let _ = client.select::<User>().columns(Lower(User::name).filter(User::is_active.eq(true)));
    let _ = client.select::<User>().columns(User::id.order_by(User::id.asc()));

    // 5. Window functions support .over(), but NOT aggregate modifiers (.distinct(), .filter(), .order_by())
    let _ = client.select::<Order>().columns(RowNumber().distinct());
    let _ = client.select::<Order>().columns(RowNumber().filter(Order::is_active.eq(true)));
    let _ = client.select::<Order>().columns(RowNumber().order_by(Order::id.asc()));
    let _ = client.select::<Order>().columns(Lag(Order::amount).distinct());
    let _ = client.select::<Order>().columns(Ntile(4).filter(Order::is_active.eq(true)));
}