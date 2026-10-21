use kova::Options;
use models::type_check::user::User;

fn main() {
    let client = Options::in_memory().build().unwrap();

    // 1. Attempting to return `tx` out of the synchronous transaction closure must violate HRTB lifetimes
    let _escaped_tx = client.transaction(|tx| Ok(tx));

    // 2. Attempting to return a query builder borrowing `tx` must also fail lifetime checks
    let _escaped_query = client.transaction(|tx| Ok(tx.select::<User>()));
}