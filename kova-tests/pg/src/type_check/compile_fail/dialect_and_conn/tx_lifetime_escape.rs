use kova::Options;
use models::type_check::user::User;

async fn check() {
    let client = Options::from("").build().unwrap();

    // 1. Attempting to return `tx` out of the async transaction closure must violate HRTB lifetimes
    let _escaped_tx = client
        .transaction(async |tx| Ok(tx))
        .await;

    // 2. Attempting to return a query builder borrowing `tx` must also fail lifetime checks
    let _escaped_query = client
        .transaction(async |tx| Ok(tx.select::<User>()))
        .await;
}

fn main() {}