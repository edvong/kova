use kova::Options;
use models::type_check::user::User;

fn check() {
    let client = Options::in_memory().build().unwrap();

    let _ = client.statement("SELECT $1").bind(1_i128);
    let _ = client.statement("SELECT $1").bind(1_u128);

    let _ = client.select::<User>().load_in::<i128>();
    let _ = client.select::<User>().load_in::<u128>();
}

fn main() {}