use kova::Options;
use kova::dsl::func::JsonEach;
use kova::query::Select;
use kova::types::{Cte, Relation};
use models::type_check::user::User;

struct DummyCte;
impl Relation for DummyCte {
    const NAME: &'static str = "dummy_cte";
}
impl Cte for DummyCte {}

fn main() {
    let client = Options::in_memory().build().unwrap();

    // 1. InnerQuery (Select::from) has no executor and cannot be executed (.load, .load_as, .load_in)
    let _ = Select::from::<User>().load();
    let _ = Select::from::<User>().load_as::<User>();
    let _ = Select::from::<User>().load_in::<i32>();

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
        .srf(JsonEach(client.select::<User>()));
    let _ = client.insert::<User>(client.select::<User>());
}