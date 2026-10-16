#![allow(unreachable_code, dead_code, unused)]

use crate::Result;
use crate::models::order::Order;
use crate::models::user::User;
use kova::Options;
use kova::dsl::func::{
    Abs, Avg, Count, GroupConcat, Length, Lower, Max, Min, Sum, Upper, Wildcard,
};

/// Stress Test 1: Select Statements, Nested Functions & Aggregate Modifiers
fn compile_check_sqlite_select() {
    let client = Options::in_memory().build().unwrap();

    client
        .select::<User>()
        .distinct()
        // 1. Polymorphic projections: single items and large mixed tuples
        .columns(Count(Wildcard))
        .columns(User::name)
        .columns((
            User::id,
            Order::amount,
            Count(User::id).alias(User::id),
            Sum(Order::amount).alias(User::name),
            Min(Order::amount),
            Max(Order::amount),
            Avg(Order::amount),
            Lower(User::name),
            Upper(User::name),
            Abs(Order::amount),
            Length(User::name),
            Upper(Lower(Upper(User::name))),
            Sum(Abs(Order::amount)),
            // GroupConcat chaining with separator, distinct, order_by, filter, and alias
            GroupConcat(Lower(User::name))
                .separator(",")
                .distinct()
                .order_by(User::id.desc())
                .filter(User::is_active.eq(true))
                .alias(User::name),
        ))
        // 2. JOIN boundaries: Column-to-Column and Column-to-Value
        .inner_join::<Order>(Order::user_id.eq(User::id))
        .left_join::<Order>(Order::is_active.ne(false))
        // 3. Deep Boolean Logic Boundaries (&, |, !) and mixed AST/Value between
        .filter(
            (!User::name.like("%banned%") & Order::amount.is_not_null())
                | (Order::amount.between(10.5_f64, i64::MAX as f64)
                & Order::amount.not_between(0.0_f64, Order::amount)
                & Order::id.in_list(vec![1, 2, 3])
                & Order::id.not_in_list([4, 5, 6]))
                | (Order::amount.ge(0.0_f64)
                | Order::amount.le(f64::MAX)
                | Order::raw_data.eq(&b"zero_copy_bytes"[..])),
        )
        // 4. Group By Boundaries: columns and scalar functions
        .group_by(User::id)
        .group_by(Order::is_active)
        .group_by(Lower(User::name))
        .group_by((User::id, Order::is_active, Lower(User::name)))
        // 5. Having Boundaries
        .having(Sum(Order::amount).gt(1000.50_f64) & Count(Wildcard).ge(1))
        // 6. Order By Boundaries: explicit direction/nulls, implicit column, and Condition
        .order_by((
            Order::amount.desc().nulls_last(),
            User::name.asc().nulls_first(),
            User::id,
            User::is_active.eq(true),
        ))
        // 7. Pagination Bounds
        .limit(usize::MAX)
        .offset(0);
}

/// Stress Test 2: Math AST Chaining & Type Widening Compatibility
fn compile_check_sqlite_math_and_compatible() -> Result<()> {
    let client = Options::in_memory().build()?;

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
            // Memo: Chained arithmetic (add/sub/mul/div) preserving output type Static<f64>
            User::balance.add(Order::amount).sub(10.0).mul(float_32).div(2_i16).alias(User::balance),
            // Memo: Integer arithmetic with implicit widening (i8 -> i32)
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
        .load_in::<(f64, i32)>()?;

    Ok(())
}

/// Stress Test 3: Lifecycle of Load API (Synchronous)
fn compile_check_sqlite_load_endpoints() -> Result<()> {
    let client = Options::in_memory().build()?;

    let _users: Vec<User> = client.select::<User>().load()?;
    let _orders: Vec<Order> = client.select::<Order>().load()?;
    let _joined: Vec<(User, Order)> = client.select::<User>().load_as::<(User, Order)>()?;
    let _opt_model: Vec<Option<User>> = client.select::<User>().load_as::<Option<User>>()?;

    let _ids: Vec<Option<f64>> = client.select::<User>().load_in::<Option<f64>>()?;
    let _boxed: Vec<Box<String>> = client.select::<User>().load_in::<Box<String>>()?;
    let _fixed: Vec<[u8; 16]> = client.select::<Order>().load_in::<[u8; 16]>()?;

    let _complex_rows = client
        .select::<User>()
        .load_in::<(i64, String, Option<f64>, bool, Vec<u8>, [u8; 8])>()?;

    Ok(())
}
