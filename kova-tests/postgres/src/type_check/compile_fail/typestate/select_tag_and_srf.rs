use postgres::type_check::codec::AppCodec;
use kova::Options;
use kova::dsl::func::{GenerateSeries, Unnest};
use kova::query::Select;
use models::type_check::order::Order;
use models::type_check::user::User;

struct T1;
struct T2;

fn main() {
    let client = Options::from("").build().unwrap().codec::<AppCodec>();

    // 1. Cannot tag main table twice (`TaggedFrom` has no `.tag()`)
    let _ = client.select::<User>().tag::<T1>().tag::<T2>();
    let _ = Select::from::<User>().tag::<T1>().tag::<T2>();

    // 2. Cannot tag on `Select` after transitioning to `Normal` state (.filter, .columns, .distinct, .srf)
    let _ = client.select::<User>().filter(User::id.eq(1)).tag::<T1>();
    let _ = client.select::<User>().columns(User::id).tag::<T1>();
    let _ = client.select::<User>().distinct().tag::<T1>();
    let _ = client.select::<User>().srf(Unnest(vec![1_i32])).tag::<T1>();

    // 3. Cannot call `.srf()` after transitioning to `Normal` or `JustAddJoin`
    let _ = client.select::<User>().filter(User::id.eq(1)).srf(Unnest(vec![1_i32]));
    let _ = client
        .select::<User>()
        .inner_join::<Order>(Order::user_id.eq(User::id))
        .srf(Unnest(vec![1_i32]));

    // 4. Cannot call `.srf()` twice (`Normal` has no `.srf()`), cannot tag an SRF node twice,
    //    and cannot call `.step()` after `.tag()` on GenerateSeries
    let _ = client
        .select::<User>()
        .srf(Unnest(vec![1_i32]))
        .srf(Unnest(vec![2_i32]));
    let _ = Unnest(vec![1_i32]).tag::<T1>().tag::<T2>();
    let _ = GenerateSeries(1_i32, 10_i32).step(1_i32).tag::<T1>().tag::<T2>();
    let _ = GenerateSeries(1_i32, 10_i32).tag::<T1>().step(1_i32);

    // 5. Cannot tag a joined table twice (`JustAddJoin` -> `Normal`)
    let _ = client
        .select::<User>()
        .inner_join::<Order>(Order::user_id.eq(User::id))
        .tag::<T1>()
        .tag::<T2>();
}