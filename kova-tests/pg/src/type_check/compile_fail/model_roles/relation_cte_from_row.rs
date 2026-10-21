use kova::Options;
use kova::dsl::LockMode;
use kova::query::Select;
use kova::types::{Cte, Relation};
use models::type_check::order::Order;
use models::type_check::post::Post;
use models::type_check::post_view::PostView;
use models::type_check::user::User;

pub struct OnlyRelation;
impl Relation for OnlyRelation {
    const NAME: &'static str = "only_rel";
}

pub struct OnlyCte;
impl Relation for OnlyCte {
    const NAME: &'static str = "only_cte";
}
impl Cte for OnlyCte {}

async fn check() {
    let client = Options::from("").build().unwrap();

    // 1. Non-Model types (OnlyRelation, OnlyCte, PostView) cannot initiate CRUD or lock::<M>()
    let _ = client.select::<OnlyRelation>();
    let _ = client.select::<OnlyCte>();
    let _ = client.select::<PostView>();
    let _ = Select::from::<OnlyCte>();
    let _ = client.update::<OnlyRelation>();
    let _ = client.delete::<OnlyRelation>();
    let _ = client.delete::<PostView>();
    let _ = client.select::<User>().lock::<OnlyRelation>(LockMode::Update, None);

    // 2. `.with::<C>()` requires `C: Cte`; plain `Model` or `Relation` is rejected
    let _ = client.select::<User>().with::<Order>(Select::from::<Order>());
    let _ = client.select::<User>().with::<OnlyRelation>(Select::from::<Order>());

    // 3. `.inner_join::<R>()` and `Update::from::<R>()` require `R: Relation`; `PostView` (FromRow only) is rejected
    let _ = client
        .select::<User>()
        .inner_join::<PostView>(User::id.eq(1));
    let _ = client.update::<User>().from::<PostView>();
    let _ = client.update::<User>().from::<(Order, PostView)>();

    // 4. `.load_as::<U>()` requires `U: FromRow`; `OnlyCte` / `OnlyRelation` are rejected
    let _ = client.select::<User>().load_as::<OnlyCte>().await;
    let _ = client.select::<User>().load_as::<OnlyRelation>().await;

    // 5. `client.insert::<T>(data)` requires data source to match target Model `T`
    let _ = client.insert::<User>(Post);
    let _ = client.insert::<User>(vec![Post]);
}

fn main() {}