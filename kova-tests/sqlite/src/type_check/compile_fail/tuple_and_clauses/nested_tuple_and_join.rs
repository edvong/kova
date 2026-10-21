use kova::Options;
use kova::dsl::func::{Count, Lower, Window};
use models::type_check::order::Order;
use models::type_check::user::User;

fn main() {
    let client = Options::in_memory().build().unwrap();
    let user = User::default();

    // 1. SingleArg guard forbids nested tuples and empty unit `()` across all IntoArg clauses
    let _ = client
        .select::<User>()
        .columns(((User::id, User::name), User::balance));
    let _ = client.select::<User>().columns(());
    let _ = client
        .select::<User>()
        .group_by(((User::id, User::name), User::is_active));
    let _ = client.select::<User>().group_by(());
    let _ = client
        .select::<User>()
        .order_by(((User::id.asc(), User::name.desc()), User::balance.asc()));
    let _ = client.select::<User>().order_by(());
    let _ = client
        .update::<User>()
        .set(((User::name.set("a"), User::balance.set(1.0)), User::is_active.set(true)));
    let _ = client.update::<User>().set(());
    let _ = client
        .insert(&user)
        .on_conflict(((User::id, User::name), User::is_active));
    let _ = client.insert(&user).returning(((User::id, User::name), User::balance));
    let _ = client.select::<Order>().columns(
        Count(Order::id).over(Window().partition_by(((Order::id, Order::user_id), Order::is_active))),
    );
    let _ = client.statement("SELECT ?, ?, ?").bind(((1_i32, 2_i32), 3_i32));
    let _ = client.statement("SELECT 1").bind(());

    // 2. Raw literals and boolean Conditions cannot be used as projections or GROUP BY items
    let _ = client.select::<User>().columns(123_i32);
    let _ = client.select::<User>().columns(User::id.eq(1));
    let _ = client.select::<User>().group_by(User::id.eq(1));

    // 3. IntoJoinConstraint forbids tuple of Conditions, mixed (Condition, Column), and non-column AST
    let _ = client.select::<Order>().inner_join::<User>((
        Order::user_id.eq(User::id),
        User::is_active.eq(true),
    ));
    let _ = client
        .select::<Order>()
        .inner_join::<User>((Order::user_id.eq(User::id), User::id));
    let _ = client
        .select::<Order>()
        .inner_join::<User>(Lower(User::name));

    // 4. in_list element type mismatch & in_select rejecting non-query Vec
    let _ = client.select::<User>().filter(User::id.in_list(["a", "b"]));
    let _ = client.select::<User>().filter(User::id.in_select(vec![1_i32, 2_i32]));
}