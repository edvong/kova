use kova::Options;
use models::type_check::order::Order;
use models::type_check::post::Post;
use models::type_check::user::User;

fn main() {
    let client = Options::from("").build().unwrap();

    // 1. Literal type mismatches in filter comparisons and between
    let _ = client.select::<User>().filter(User::id.eq("not_an_int"));
    let _ = client.select::<User>().filter(User::name.eq(123_i32));
    let _ = client.select::<User>().filter(User::is_active.eq(1_i32));
    let _ = client.select::<Order>().filter(Order::raw_data.eq("not_bytes"));
    let _ = client.select::<Post>().filter(Post::tags.eq(&[1_i32, 2_i32][..]));
    let _ = client.select::<User>().filter(User::id.between(1_i32, "ten"));

    // 2. Incompatible column-to-column comparison and arithmetic operand mismatch
    let _ = client.select::<User>().filter(User::id.eq(User::name));
    let _ = client.select::<User>().columns(User::id.add("not_a_number"));

    // 3. Literal and column type mismatch in UPDATE assignment
    let _ = client.update::<User>().set(User::id.set("wrong_value"));
    let _ = client.update::<User>().set(User::id.set(User::name));
}