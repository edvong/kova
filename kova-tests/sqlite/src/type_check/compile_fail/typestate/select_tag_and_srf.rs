use kova::Options;
use kova::dsl::func::{JsonEach, JsonTree};
use kova::query::Select;
use models::type_check::order::Order;
use models::type_check::user::User;

struct T1;
struct T2;

fn main() {
    let client = Options::in_memory().build().unwrap();

    // 1. Cannot tag main table twice (`TaggedFrom` has no `.tag()`)
    let _ = client.select::<User>().tag::<T1>().tag::<T2>();
    let _ = Select::from::<User>().tag::<T1>().tag::<T2>();

    // 2. Cannot tag on `Select` after transitioning to `Normal` state (.filter, .columns, .distinct, .srf)
    let _ = client.select::<User>().filter(User::id.eq(1)).tag::<T1>();
    let _ = client.select::<User>().columns(User::id).tag::<T1>();
    let _ = client.select::<User>().distinct().tag::<T1>();
    let _ = client.select::<User>().srf(JsonEach(User::name)).tag::<T1>();

    // 3. Cannot call `.srf()` after transitioning to `Normal` or `JustAddJoin`
    let _ = client.select::<User>().filter(User::id.eq(1)).srf(JsonEach(User::name));
    let _ = client
        .select::<User>()
        .inner_join::<Order>(Order::user_id.eq(User::id))
        .srf(JsonEach(User::name));

    // 4. Cannot call `.srf()` twice (`Normal` has no `.srf()`) or tag an SRF node twice
    let _ = client
        .select::<User>()
        .srf(JsonEach(User::name))
        .srf(JsonEach(User::name));
    let _ = JsonEach(User::name).tag::<T1>().tag::<T2>();
    let _ = JsonTree((User::name, "$.a")).tag::<T1>().tag::<T2>();

    // 5. Cannot tag a joined table twice (`JustAddJoin` -> `Normal`)
    let _ = client
        .select::<User>()
        .inner_join::<Order>(Order::user_id.eq(User::id))
        .tag::<T1>()
        .tag::<T2>();
}