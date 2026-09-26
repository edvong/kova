#![allow(unreachable_code, dead_code, unused)]

use crate::Result;
use crate::codec::conversion::AppCodec;
use crate::codec::models::EventLog;
use crate::models::comment::Comment;
use crate::models::order::Order;
use crate::models::post::Post;
use crate::models::user::User;
use kova::Options;
use kova::dsl::func::{JsonEach, JsonTree};
use kova::query::Select;

/// Test 1: Polymorphic Joins (Seamlessly switching between ON and USING)
fn compile_check_advanced_joins() -> Result<()> {
    let client = Options::in_memory().build()?;

    let _ = client
        .select::<Order>()
        .inner_join::<User>(Order::user_id.eq(User::id) | Order::amount.gt(100.0_f64))
        .left_join::<Post>(Post::author_id)
        .right_join::<Comment>((Comment::user_id, Comment::post_id))
        .full_join::<EventLog>(EventLog::description.like("%warn%"))
        .cross_join::<User>()
        .load_as::<Order>()?;

    Ok(())
}

/// Test 2: Upsert Where (Boundary validation for secondary filtering on conflict)
fn compile_check_upsert_where() -> Result<()> {
    let client = Options::in_memory().build()?;
    let users = vec![User::default(), User::default()];

    client
        .insert(&users)
        .on_conflict((User::id, User::name))
        .update((User::name.set("conflict_resolved"), User::is_active.set(true)))
        .filter(User::is_active.eq(false))
        .execute()?;

    Ok(())
}

/// Test 3: Update From (Cross-table update support)
fn compile_check_update_from() -> Result<()> {
    let client = Options::in_memory().build()?;

    client
        .update::<Order>()
        .set(Order::amount.set(999.9_f64))
        .from::<User>()
        .from::<(User, Post)>()
        .filter(
            Order::user_id.eq(User::id) & User::id.eq(Post::author_id) & User::is_active.eq(true),
        )
        .execute()?;

    Ok(())
}

/// Test 4: Set Operation
fn compile_check_set_operations() -> Result<()> {
    let client = Options::from("").build()?;

    let _ = client
        .select::<User>()
        // Main query: extracting id and name
        .columns((User::id, User::name))
        .filter(User::is_active.eq(true))
        // Tacking on UNION ALL
        .union_all(
            Select::from::<User>()
                .columns((User::id, User::name))
                .filter(User::is_active.eq(false)),
        )
        // Chaining EXCEPT
        .except(
            Select::from::<User>()
                .columns((User::id, User::name))
                .filter(User::name.like("%test%")),
        )
        // Final safeguard during extraction: load_in tightly seals the type boundaries
        .load_in::<(i32, String)>()?;

    Ok(())
}

/// Test 5: INSERT ... SELECT
fn compile_check_insert_select() -> Result<()> {
    let client = Options::in_memory().build()?;

    client.insert(User::default()).execute()?;

    client
        .insert::<User>(
            Select::from::<Post>()
                .columns((Post::title, Post::content))
                .filter(Post::author_id.eq(100)),
        )
        .on_conflict(User::id)
        .do_nothing()
        .execute()?;

    Ok(())
}

/// Stress Test 6: Table-Valued JSON Functions (json_each & json_tree)
/// Memo: Validates the zero-abstraction-leak integration of SQLite's virtual tables.
/// Ensures that single/double arguments, tuple-based FROM clauses, static schema access,
/// and Deref-based instance access all compile seamlessly without requiring explicit generics (`::<_>`).
fn compile_check_sqlite_table_functions() -> Result<()> {
    let client = Options::in_memory().build()?.codec::<AppCodec>();

    // ==========================================
    // Scenario 1: Standard Single-Arg & Operator Inheritance
    // Memo: Tests implicit translation of single `Column` argument.
    // Also proves that the virtual column `JsonEach::value` inherits dialect-specific operators like `.json_extract()`.
    // ==========================================
    let _ = client
        .select_mix::<EventLog>(JsonEach(EventLog::metadata)) // 1 Arg
        .columns((EventLog::id, JsonEach::key, JsonEach::value, JsonEach::type_))
        .filter(
            // Standard operator (eq) on String virtual column
            JsonEach::type_.eq("object")
                // Dialect operator (json_extract -> eq) on String virtual column
                | JsonEach::value.json_extract("$.is_active").eq(true),
        )
        // Expected tuple perfectly aligns with the static types defined in JsonEachSchema
        .load_in::<(uuid::Uuid, String, String, String)>()?;

    // ==========================================
    // Scenario 2: Double-Arg & Tuple-based Multiple Append
    // Memo: Tests the `(Column, &'static str)` translation escaping the E0207/E0119 traps.
    // Also proves that `.from()` accepts tuples (IntoArgs) to perform implicit LATERAL JOINs.
    // ==========================================
    let _ = client
        .select_mix::<User>((
            // Tuple parsing: 2 Args
            JsonEach((User::name, "$.roles")),
            // Tuple parsing: 2 Args
            JsonTree((User::name, "$.permissions")),
        ))
        .columns((User::id, JsonEach::key, JsonTree::path, JsonTree::parent))
        .filter(
            // Cross-virtual-table comparisons are fully supported and type-safe
            JsonEach::atom.is_not_null() & JsonTree::parent.gt(0_i32),
        )
        .load_in::<(i32, String, String, i32)>()?;

    // ==========================================
    // Scenario 3: Subselect Penetration & Deref Instance Access
    // Memo: Proves that `Subselect` natively exposes `.from()`.
    // Also demonstrates the `Deref` magic where we instantiate the wrapper to access schema fields,
    // which paves the ultimate golden path for the upcoming `.alias("j1")` feature.
    // ==========================================

    // Instantiating the AST wrapper
    let virtual_tree = JsonTree((EventLog::metadata, "$.deep_nodes"));

    let _ = client
        .select::<EventLog>()
        .filter(
            EventLog::id.in_select(
                Select::from_mix::<EventLog>(JsonTree((EventLog::metadata, "$.deep_nodes")))
                    .columns(EventLog::id)
                    // Magic: `virtual_tree.atom` leverages `Deref<Target = JsonTreeSchema>`
                    // No static namespace used here! Pure instance field access.
                    .filter(virtual_tree.atom.eq("target_value")),
            ),
        )
        .load_as::<EventLog>()?;

    Ok(())
}
