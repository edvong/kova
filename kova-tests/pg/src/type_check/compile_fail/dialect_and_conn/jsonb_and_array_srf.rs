use kova::Options;
use kova::dsl::func::{
    All, Any, JsonbArrayElements, JsonbEach, JsonbEachText, JsonbObject, JsonbObjectSingle, Lag,
    Lead, Ntile, Unnest,
};
use models::type_check::event_log::EventLog;
use models::type_check::order::Order;
use models::type_check::post::Post;
use models::type_check::user::User;
use pg::type_check::codec::AppCodec;

fn main() {
    let client = Options::from("").build().unwrap().codec::<AppCodec>();

    // 1. In PG, plain `Column<String>` does NOT implement `CanPgJsonb`
    let _ = client.select::<User>().filter(User::name.json_path("key").eq("val"));
    let _ = client.select::<User>().srf(JsonbEach(User::name));
    let _ = client.select::<User>().srf(JsonbEachText(User::name));

    // 2. `->>` (`json_path_text`) outputs `Static<String>`, terminating JSONB capability (`CanPgJsonb`)
    let _ = client.select::<EventLog>().filter(
        EventLog::metadata
            .json_path_text("user")
            .json_path("profile")
            .eq("admin"),
    );
    let _ = client.select::<EventLog>().filter(
        EventLog::metadata
            .json_path_text("user")
            .json_contains("admin"),
    );
    let _ = client.select::<EventLog>().srf(JsonbArrayElements(
        EventLog::metadata.json_path_text("tags"),
    ));

    // 3. JSON key/path arguments must be compatible with `Static<String>`
    let _ = client
        .select::<EventLog>()
        .filter(EventLog::metadata.json_path(123_i32).eq("val"));
    let _ = client
        .select::<EventLog>()
        .filter(EventLog::metadata.json_has_key(123_i32));

    // 4. Array operators, `Unnest`, `JsonbObject`, and `JsonbObjectSingle` require `CanPgArray`
    let _ = client.select::<User>().filter(User::name.array_contains("a"));
    let _ = client.select::<User>().srf(Unnest(User::id));
    let _ = client.select::<User>().srf(Unnest(123_i32));
    let _ = client
        .select::<Post>()
        .columns(JsonbObject(Post::title, Post::tags));
    let _ = client
        .select::<Post>()
        .columns(JsonbObjectSingle(Post::title));

    // 5. `Any` / `All` element type must be compatible with LHS column
    let _ = client
        .select::<User>()
        .filter(User::id.eq(Any(vec!["not_an_int"])));
    let _ = client
        .select::<User>()
        .filter(User::id.gt(All(["bad_element"])));

    // 6. Window function argument bounds: Ntile(i32), Lag/Lead offset(i32) and default(E::Output)
    let _ = client.select::<Order>().columns(Ntile("not_an_i32"));
    let _ = client
        .select::<Order>()
        .columns(Lag(Order::amount).offset("bad_offset"));
    let _ = client
        .select::<Order>()
        .columns(Lead(Order::amount).default("bad_default_for_f64"));
}