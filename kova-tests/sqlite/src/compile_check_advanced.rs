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

/// Test 1: Polymorphic Joins (ON vs USING)
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

/// Test 2: Upsert Where
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

/// Test 3: Update From
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

/// Test 4: Set Operations (UNION, UNION ALL, INTERSECT, EXCEPT)
fn compile_check_set_operations() -> Result<()> {
    let client = Options::in_memory().build()?;

    let _ = client
        .select::<User>()
        .columns((User::id, User::name))
        .filter(User::is_active.eq(true))
        .union(Select::from::<User>().columns((User::id, User::name)))
        .union_all(
            Select::from::<User>()
                .columns((User::id, User::name))
                .filter(User::is_active.eq(false)),
        )
        .intersect(Select::from::<User>().columns((User::id, User::name)))
        .except(
            Select::from::<User>()
                .columns((User::id, User::name))
                .filter(User::name.like("%test%")),
        )
        .load_in::<(i32, String)>()?;

    Ok(())
}

/// Test 5: INSERT ... SELECT & Raw Statement Binding
fn compile_check_insert_select_and_raw_statement() -> Result<()> {
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

    let bytes: &[u8] = b"raw";
    let fixed: [u8; 4] = [1, 2, 3, 4];
    let _rows = client
        .statement("SELECT id, name FROM user WHERE id = ? AND name = ? AND raw = ? AND hash = ? AND opt = ?")
        .bind((100_i32, "alice", bytes, fixed, Some(true)))
        .load_in::<(i32, String)>()?;

    Ok(())
}

/// Stress Test 6: Table-Valued JSON Functions (json_each & json_tree)
fn compile_check_sqlite_table_functions() -> Result<()> {
    let client = Options::in_memory().build()?.codec::<AppCodec>();

    // Scenario 1: Single-Arg & Operator Inheritance on Dynamic Virtual Column
    let _ = client
        .select::<EventLog>()
        .srf(JsonEach(EventLog::metadata))
        .columns((EventLog::id, JsonEach::key, JsonEach::value, JsonEach::type_))
        .filter(
            JsonEach::type_.eq("object")
                | JsonEach::value.json_extract("$.is_active").eq(true),
        )
        .load_in::<(uuid::Uuid, String, String, String)>()?;

    // Scenario 2: Double-Arg (Node, Path) & Tuple-based Multiple Append
    let _ = client
        .select::<User>()
        .srf((
            JsonEach((User::name, "$.roles")),
            JsonTree((User::name, "$.permissions")),
        ))
        .columns((User::id, JsonEach::key, JsonTree::path, JsonTree::parent))
        .filter(JsonEach::atom.is_not_null() & JsonTree::parent.gt(0_i64))
        .load_in::<(i32, String, String, i32)>()?;

    // Scenario 3: Subselect Penetration & Copyable Deref Instance Access
    let virtual_tree = JsonTree((EventLog::metadata, "$.deep_nodes"));

    let _ = client
        .select::<EventLog>()
        .filter(
            EventLog::id.in_select(
                Select::from::<EventLog>()
                    .srf(virtual_tree)
                    .columns(EventLog::id)
                    .filter(virtual_tree.atom.eq("target_value")),
            ),
        )
        .load_as::<EventLog>()?;

    // Scenario 4: Feeding an entire Subquery into JsonEach
    let _ = client
        .select::<User>()
        .filter(
            User::id.in_select(
                Select::from::<Order>()
                    .srf(JsonEach(
                        Select::from::<EventLog>()
                            .columns(EventLog::metadata)
                            .filter(EventLog::id.is_not_null()),
                ))
                    .columns(JsonEach::value),
            ),
        )
        .load_as::<User>()?;

    Ok(())
}
