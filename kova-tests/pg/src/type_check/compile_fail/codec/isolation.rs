use kova::Options;
use kova::codec::{Decode, Value};
use models::type_check::event_log::EventLog;
use models::type_check::post_view::PostView;
use models::type_check::user::User;
use pg::type_check::codec::AppCodec;
use uuid::Uuid;

pub struct OtherCodec;
pub struct UnregisteredScalar(pub i32);

impl Decode<AppCodec> for UnregisteredScalar {
    fn decode(_: Value<'_>) -> kova::Result<Self> {
        Ok(UnregisteredScalar(0))
    }
}
// Note: intentionally omitting `decode_column!(AppCodec, UnregisteredScalar);`

async fn check() {
    let default_client = Options::from("").build().unwrap();
    let other_client = Options::from("").build().unwrap().codec::<OtherCodec>();
    let app_client = Options::from("").build().unwrap().codec::<AppCodec>();

    let id = Uuid::nil();

    // 1. Model & FromRow with custom fields rejected when their codec bounds are not met
    let _ = default_client.select::<EventLog>();
    let _ = default_client.select::<User>().load_as::<PostView>().await;
    let _ = app_client.select::<User>().load_as::<PostView>().await;

    // 2. Custom types (Uuid) rejected in filter, bind, and load_in under default codec `()`
    let _ = default_client.select::<User>().filter(EventLog::id.eq(id));
    let _ = default_client.statement("SELECT $1").bind(id);
    let _ = default_client.select::<User>().load_in::<Uuid>().await;

    // 3. Types, custom Compatible rules, and custom Can*<AppCodec> capabilities rejected under `OtherCodec`
    let _ = other_client.select::<User>().filter(EventLog::id.eq(id));
    let _ = other_client
        .update::<User>()
        .set(EventLog::website.set("https://rust-lang.org"));
    let _ = other_client
        .select::<User>()
        .columns(EventLog::cost.add(10_i64));
    let _ = other_client
        .select::<User>()
        .filter(EventLog::website.ilike("https://%"));
    let _ = other_client
        .select::<User>()
        .filter(EventLog::payload.json_has_key("actor"));
    let _ = other_client
        .select::<User>()
        .filter(EventLog::labels.array_contains(&["core"][..]));

    // 4. Type implementing `Decode<AppCodec>` without `decode_column!` rejected by `load_in` (`DecodeColumn`)
    let _ = app_client
        .select::<EventLog>()
        .load_in::<UnregisteredScalar>()
        .await;
}

fn main() {}