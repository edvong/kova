use kova::Options;
use kova::dsl::func::{Abs, Avg, Count, Length, Lower, Sum, Upper};
use models::type_check::event_log::EventLog;
use models::type_check::order::Order;
use models::type_check::user::User;
use pg::type_check::codec::AppCodec;

fn main() {
    let client = Options::from("").build().unwrap().codec::<AppCodec>();

    // 1. Non-numeric columns (including Option<String> and custom Uuid) cannot perform math or increment/decrement
    let _ = client.select::<User>().columns(User::name.add("suffix"));
    let _ = client.select::<User>().columns(User::bio.sub("bio"));
    let _ = client.select::<EventLog>().columns(EventLog::id.add(EventLog::id));
    let _ = client.update::<User>().set(User::name.increment("a"));
    let _ = client.update::<User>().set(User::bio.increment("a"));
    let _ = client.update::<User>().set(User::is_active.decrement(true));

    // 2. Non-numeric columns cannot be passed to Sum, Avg, or Abs
    let _ = client.select::<User>().columns(Sum(User::name));
    let _ = client.select::<User>().columns(Sum(User::bio));
    let _ = client.select::<User>().columns(Avg(User::is_active));
    let _ = client.select::<User>().columns(Abs(User::name));
    let _ = client.select::<EventLog>().columns(Sum(EventLog::id));

    // 3. Numeric/boolean/UUID columns (including Option<f64>) cannot use textual operators or functions
    let _ = client.select::<User>().filter(User::id.like("%1%"));
    let _ = client.select::<User>().filter(User::id.ilike("%1%"));
    let _ = client.select::<Order>().filter(Order::discount.like("%1%"));
    let _ = client.select::<Order>().filter(Order::amount.matches("^[0-9]+"));
    let _ = client.select::<EventLog>().filter(EventLog::id.ilike("%a%"));
    let _ = client.select::<User>().columns(Lower(User::id));
    let _ = client.select::<Order>().columns(Lower(Order::discount));
    let _ = client.select::<User>().columns(Upper(User::is_active));
    let _ = client.select::<User>().columns(Length(Order::amount));

    // 4. Output type transition: Length/Count -> Static<i64> (loses CanString); Lower -> Static<String> (no CanMath)
    let _ = client.select::<User>().filter(Length(User::name).like("%1%"));
    let _ = client.select::<User>().columns(Lower(Length(User::name)));
    let _ = client.select::<User>().columns(Lower(Count(User::name)));
    let _ = client.select::<User>().columns(Lower(User::name).add("x"));
}