use kova::Options;
use models::type_check::order::Order;
use models::type_check::user::User;

fn main() {
    let client = Options::in_memory().build().unwrap();

    // 1. Non-nullable Column<String> cannot accept Option<&str> or Option<String> in filter/between/in_list
    let _ = client.select::<User>().filter(User::name.eq(Some("alice")));
    let _ = client.select::<User>().filter(User::name.eq(None::<String>));
    let _ = client
        .select::<User>()
        .filter(User::id.in_list([Some(1_i32), None]));
    let _ = client
        .select::<User>()
        .filter(User::id.between(Some(1_i32), Some(10_i32)));

    // 2. Non-nullable Column<T> cannot accept nullable Column<Option<T>> in eq or math
    let _ = client.select::<User>().filter(User::name.eq(User::bio));
    let _ = client
        .select::<Order>()
        .filter(Order::amount.eq(Order::discount));
    let _ = client
        .select::<Order>()
        .columns(Order::amount.add(Order::discount));

    // 3. Non-nullable Column<T> cannot be assigned or incremented by Option<T> or Column<Option<T>>
    let _ = client.update::<User>().set(User::id.set(Some(1_i32)));
    let _ = client.update::<User>().set(User::id.increment(Some(1_i32)));
    let _ = client
        .update::<Order>()
        .set(Order::amount.set(Order::discount));
}