use crate::Result;
use crate::models::order::Order;
use crate::models::user::User;
use kova::Options;

async fn compile_check_update() -> Result<()> {
    let client = Options::from("").build()?;

    // 1. Single Column Update
    client
        .update::<User>()
        .set(User::name.set("new_name"))
        .set(User::id.increment(1))
        .set(User::balance.decrement(5.5))
        .filter(User::id.eq(1))
        .execute()
        .await?;

    // 2. Multi-column Update via Tuple
    client
        .update::<Order>()
        .set((Order::amount.set(99.9_f64), Order::is_active.set(true)))
        .filter(Order::user_id.in_list(vec![1, 2, 3]))
        .execute()
        .await?;

    // 3. Update Returning Tuple
    let _updated_records = client
        .update::<Order>()
        .set(Order::is_active.set(false))
        .filter(Order::amount.lt(10.0_f64))
        .returning((Order::id, Order::amount))
        .execute_in::<(i32, f64)>()
        .await?;

    Ok(())
}
