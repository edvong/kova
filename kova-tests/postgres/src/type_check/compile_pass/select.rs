use crate::Result;
use models::type_check::order::Order;
use models::type_check::user::User;
use kova::Options;
use kova::dsl::func::{
    Abs, ArrayAgg, Avg, Count, Length, Lower, Max, Min, Sum, Upper, Wildcard,
};
use kova::dsl::{LockMode, WaitPolicy};

/// Stress Test 1: Select Clauses, Nested Functions, and Aggregate Modifiers
fn compile_check_select() {
    let client = Options::from("").build().unwrap();

    client
        .select::<User>()
        .distinct()
        // 1. Polymorphic projections: single items and massive heterogeneous tuples
        .columns(Count(Wildcard))
        .columns(User::name)
        .columns(Sum(Order::amount))
        .columns((User::id, Sum(Order::amount), Count(User::id).alias(User::id)))
        .columns(ArrayAgg(Order::amount))
        .columns((
            User::id,
            Order::amount,
            Count(User::id).alias(User::id),
            Sum(Order::amount).alias(User::name),
            Min(Order::amount),
            Max(Order::amount),
            Lower(User::name),
            Upper(User::name),
            Abs(Order::amount),
            Length(User::name),
            // Nested scalar chains (String -> String) and (f64 -> f64)
            Lower(Upper(User::name)),
            Max(Abs(Order::amount)),
            ArrayAgg(Order::amount),
            // Aggregate modifiers chaining: distinct -> order_by -> filter -> alias
            ArrayAgg(Lower(User::name))
                .distinct()
                .order_by(Order::id.desc().nulls_last())
                .alias(User::name),
            ArrayAgg(Order::amount)
                .distinct()
                .order_by(Order::id.desc().nulls_first())
                .filter(Order::is_active.eq(true) & Order::amount.gt(100.0))
                .alias(User::name),
        ))
        // 2. JOIN boundaries: Column-to-Column and Column-to-Value
        .inner_join::<Order>(Order::user_id.eq(User::id))
        .left_join::<Order>(Order::is_active.ne(false))
        // 3. Boolean condition tree (&, |, !) and comparison operators
        .filter(
            !User::name.like("%banned%")
                & User::name.not_like("%bot%")
                & (Order::amount.between(10.5_f64, 99.9_f64)
                | Order::amount.not_between(0.0_f64, Order::amount) // Mixed Value + AST Node in between!
                | Order::id.in_list(vec![1, 2, 3])
                | Order::id.not_in_list([4, 5, 6])
                | Order::amount.ge(0.0_f64)
                | Order::amount.le(1000.0_f64)
                | Order::raw_data.is_not_null()),
        )
        // 4. Group By: columns, scalar functions, and math expressions
        .group_by(User::id)
        .group_by((User::id, Order::is_active, Lower(User::name)))
        // 5. Having: aggregates as LHS operands
        .having(Avg(User::id).eq(100) & Max(Order::id).gt(100_i32) & Count(Wildcard).ge(1_i64))
        // 6. Order By: explicit direction/nulls, implicit column ordering, and boolean condition ordering
        .order_by(User::name.asc())
        .order_by((
            Order::amount.desc().nulls_last(),
            User::name.asc().nulls_first(),
            User::id,
            User::is_active.eq(true),
        ))
        // 7. Pagination & Locking
        .limit(50)
        .offset(100)
        .lock::<User>(LockMode::NoKeyUpdate, Some(WaitPolicy::NoWait))
        .lock_all(LockMode::Share, Some(WaitPolicy::SkipLocked));
}

/// Stress Test 2: Math AST Chaining & Type Widening Compatibility
async fn compile_check_math_and_compatible() -> Result<()> {
    let client = Options::from("").build()?;

    let small_i8: i8 = 5;
    let small_u16: u16 = 100;
    let float_32: f32 = 1.5;
    let owned_name = String::from("alice");
    let raw_slice: &[u8] = b"binary_payload";
    let fixed_bytes: [u8; 4] = [1, 2, 3, 4];

    let _ = client
        .select::<User>()
        .inner_join::<Order>(Order::user_id.eq(User::id))
        .columns((
            // Chained arithmetic (add/sub/mul/div) preserving output type Static<f64>
            User::balance.add(Order::amount).sub(10.0).mul(float_32).div(2_i16).alias(User::balance),
            // Integer arithmetic with implicit widening (i8 / i16 -> i32)
            User::id.add(small_i8).mul(2).alias(User::id),
        ))
        .filter(
            // 1. Numeric widening: i8, &i8, u16, f32 into i32 / f64 columns
            User::id.gt(small_i8)
                & User::id.le(&small_i8)
                & User::balance.ge(small_u16)
                & User::balance.lt(&float_32)
                // 2. Math expression as LHS in comparison and between
                & User::balance.add(Order::amount).gt(1000_i32)
                & User::id.sub(1_i8).between(0_i8, 500_i16)
                // 3. Option<T> column compatibility: T, &T, &str, Option<T> -> Column<Option<T>>
                & User::bio.eq("concise bio")
                & User::bio.eq(&owned_name)
                & User::bio.eq(Some(String::from("opt bio")))
                & Order::discount.gt(0.5_f32)
                & Order::discount.le(10_i16)
                // 4. Slice & fixed-array compatibility with Column<Vec<u8>>
                & Order::raw_data.eq(raw_slice)
                & Order::raw_data.eq(fixed_bytes)
                & Order::raw_data.eq(&fixed_bytes),
        )
        .group_by((User::id.add(1), User::balance.add(Order::amount)))
        .order_by(User::balance.sub(Order::amount).desc())
        .load_in::<(f64, i32)>()
        .await?;

    Ok(())
}

/// Stress Test 3: Load Endpoints (load, load_as, load_in)
async fn compile_check_load_endpoints() -> Result<()> {
    let client = Options::from("").build()?;

    let _users: Vec<User> = client.select::<User>().load().await?;
    let _orders: Vec<Order> = client.select::<Order>().load().await?;
    let _joined: Vec<(User, Order)> = client.select::<User>().load_as::<(User, Order)>().await?;
    let _opt_model: Vec<Option<User>> = client.select::<User>().load_as::<Option<User>>().await?;

    let _scalar: Vec<i32> = client.select::<User>().load_in::<i32>().await?;
    let _opt_scalar: Vec<Option<f64>> = client.select::<User>().load_in::<Option<f64>>().await?;
    let _box_scalar: Vec<Box<String>> = client.select::<User>().load_in::<Box<String>>().await?;
    let _fixed_blob: Vec<[u8; 16]> = client.select::<Order>().load_in::<[u8; 16]>().await?;

    let _complex_rows = client
        .select::<User>()
        .load_in::<(i32, String, Option<f64>, bool, Vec<u8>, [u8; 8])>()
        .await?;

    Ok(())
}

/// Stress Test 4: Nullable Column (`Option<T>`) Capability Penetration & Deep Reference/Container Operands
fn compile_check_nullable_capabilities_and_deep_refs() {
    let client = Options::from("").build().unwrap();

    let boxed_id: Box<i32> = Box::new(42);
    let opt_id: Option<i32> = Some(7);
    let opt_str: Option<&str> = Some("borrowed_opt");
    let double_ref_str: &&str = &"double_borrowed";
    let double_ref_bytes: &&[u8] = &b"bytes".as_slice();

    let _ = client
        .select::<User>()
        .inner_join::<Order>(Order::user_id.eq(User::id))
        .columns((
            // 1. Nullable numeric column (`Column<Option<f64>>`) math & aggregate penetration
            Order::discount.add(0.1_f64).mul(Order::amount).sub(Order::discount),
            Sum(Order::discount),
            Avg(Order::discount),
            Abs(Order::discount),
            // 2. Nullable string column (`Column<Option<String>>`) string function penetration
            Lower(User::bio),
            Upper(User::bio),
            Length(User::bio),
        ))
        .filter(
            // 3. Nullable string column with .like() / .not_like()
            User::bio.like("%developer%")
                & User::bio.not_like("%bot%")
                // 4. Container & multi-layer reference penetration (`Box<T>`, `&Box<T>`, `&Option<T>`, `Option<&str>`, `&&str`, `&&[u8]`)
                & User::id.eq(&boxed_id)
                & User::id.eq(boxed_id.clone())
                & Order::discount.eq(&Some(0.25_f64))
                & User::bio.eq(&opt_id.map(|_| String::from("owned")))
                & User::bio.eq(opt_str)
                & User::bio.eq(&opt_str)
                & User::name.eq(double_ref_str)
                & Order::raw_data.eq(double_ref_bytes),
        );
}