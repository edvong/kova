use kova::Options;
use kova::dsl::func::{All, Any, JsonbEach};
use kova::query::Select;
use kova::types::{Cte, Relation};
use models::type_check::user::User;

struct DummyCte;
impl Relation for DummyCte {
    const NAME: &'static str = "dummy_cte";
}
impl Cte for DummyCte {}

async fn check() {
    let client = Options::from("").build().unwrap();

    // 1. InnerQuery (Select::from) has no executor and cannot be executed (.load, .load_as, .load_in)
    let _ = Select::from::<User>().load().await;
    let _ = Select::from::<User>().load_as::<User>().await;
    let _ = Select::from::<User>().load_in::<i32>().await;

    // 2. OuterQuery (client.select) cannot be used as a subquery or aliased projection
    let _ = client.select::<User>().alias(User::id);
    let _ = client
        .select::<User>()
        .columns(client.select::<User>().columns(User::id));
    let _ = client
        .select::<User>()
        .filter(User::id.in_select(client.select::<User>().columns(User::id)));
    let _ = client.select::<User>().union(client.select::<User>());
    let _ = client.select::<User>().intersect(client.select::<User>());
    let _ = client.select::<User>().except(client.select::<User>());
    let _ = client
        .select::<User>()
        .with::<DummyCte>(client.select::<User>());
    let _ = client
        .select::<User>()
        .filter(User::id.eq(Any(client.select::<User>().columns(User::id))));
    let _ = client
        .select::<User>()
        .filter(User::id.gt(All(client.select::<User>().columns(User::id))));
    let _ = client
        .select::<User>()
        .srf(JsonbEach(client.select::<User>()));
    let _ = client.insert::<User>(client.select::<User>());
}

fn main() {}