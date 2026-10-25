use crate::Result;
use models::type_check::comment::Comment;
use models::type_check::order::Order;
use models::type_check::post::Post;
use models::type_check::user::User;
use kova::Options;
use kova::dsl::Column;
use kova::query::Select;
use kova::types::{IdentifierMeta, Relation};

pub struct ActiveUserStats;
impl Relation for ActiveUserStats {
    const NAME: &'static str = "active_user_stats";
}
#[allow(non_upper_case_globals)]
impl ActiveUserStats {
    pub const user_id: Column<i32, IdentifierMeta> =
        Column::identifier("active_user_stats", "user_id");
    pub const total_spent: Column<f64, IdentifierMeta> =
        Column::identifier("active_user_stats", "total_spent");
}

/// Stress Test 1: Polymorphic Joins (ON vs USING)
async fn compile_check_advanced_joins() -> Result<()> {
    let client = Options::from("").build()?;

    let _ = client
        .select::<Order>()
        .inner_join::<User>(Order::user_id.eq(User::id) & User::is_active.eq(true))
        .left_join::<Post>(Post::author_id)
        .right_join::<Comment>((Comment::user_id, Comment::post_id))
        .full_join::<User>(Order::user_id)
        .cross_join::<User>()
        .load_as::<Order>()
        .await?;

    Ok(())
}

/// Stress Test 2: Upsert WHERE
async fn compile_check_upsert_where() -> Result<()> {
    let client = Options::from("").build()?;
    let users = vec![User::default(), User::default()];

    client
        .insert(&users)
        .on_conflict((User::id, User::name))
        .update((User::name.set("conflict_resolved"), User::is_active.set(true)))
        .filter(User::balance.gt(0.0_f64) | User::is_active.eq(false))
        .execute()
        .await?;

    Ok(())
}

/// Stress Test 3: UPDATE ... FROM
async fn compile_check_update_from() -> Result<()> {
    let client = Options::from("").build()?;

    client
        .update::<User>()
        .set(User::is_active.set(true))
        .from::<(Order, ActiveUserStats)>()
        .filter(
            User::id.eq(Order::user_id)
                & User::id.eq(ActiveUserStats::user_id)
                & ActiveUserStats::total_spent.gt(5000.0_f64),
        )
        .execute()
        .await?;

    Ok(())
}

/// Stress Test 4: Set Operations (UNION, UNION ALL, INTERSECT, EXCEPT)
async fn compile_check_set_operations() -> Result<()> {
    let client = Options::from("").build()?;

    let _ = client
        .select::<User>()
        .columns((User::id, User::name))
        .filter(User::is_active.eq(true))
        .union(Select::from::<User>().columns((User::id, User::name)))
        .union_all(
            Select::from::<User>()
                .columns((User::id, User::name))
                .filter(User::balance.gt(1000.0_f64)),
        )
        .intersect(Select::from::<User>().columns((User::id, User::name)))
        .except(
            Select::from::<User>()
                .columns((User::id, User::name))
                .filter(User::name.like("%test%")),
        )
        .load_in::<(i32, String)>()
        .await?;

    Ok(())
}

/// Stress Test 5: INSERT ... SELECT & Raw SQL Statement Binding
async fn compile_check_insert_select_and_raw_statement() -> Result<()> {
    let client = Options::from("").build()?;

    let user = User::default();
    client.insert(user).execute().await?;

    client
        .insert::<User>(
            Select::from::<User>()
                .columns((User::name, User::is_active))
                .filter(User::is_active.eq(true)),
        )
        .execute()
        .await?;

    client
        .insert::<User>(
            Select::from::<Post>()
                .columns((Post::title, Post::content))
                .filter(Post::author_id.is_not_null()),
        )
        .on_conflict((User::id, User::name))
        .do_nothing()
        .execute()
        .await?;

    // Validates RawStatement polymorphic .bind() with primitives, slices, Options, and fixed arrays
    let bytes: &[u8] = b"raw";
    let fixed: [u8; 4] = [1, 2, 3, 4];
    let _rows = client
        .statement("SELECT id, name FROM \"user\" WHERE id = $1 AND name = $2 AND raw = $3 AND hash = $4 AND opt = $5")
        .bind((100_i32, "alice", bytes, fixed, Some(true)))
        .load_in::<(i32, String)>()
        .await?;

    Ok(())
}