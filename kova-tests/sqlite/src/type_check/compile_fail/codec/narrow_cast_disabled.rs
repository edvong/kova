use kova::Options;
use models::type_check::user::User;

fn main() {
    let client = Options::in_memory().build().unwrap();

    // 1. 64-bit unsigned, pointer-sized, and 128-bit integers cannot be encoded without `narrow-cast`
    let _ = client.statement("SELECT ?").bind(1_u64);
    let _ = client.statement("SELECT ?").bind(1_usize);
    let _ = client.statement("SELECT ?").bind(1_isize);
    let _ = client.statement("SELECT ?").bind(1_i128);
    let _ = client.statement("SELECT ?").bind(1_u128);

    // 2. 64-bit unsigned, pointer-sized, and 128-bit integers cannot be decoded without `narrow-cast`
    let _ = client.select::<User>().load_in::<u64>();
    let _ = client.select::<User>().load_in::<usize>();
    let _ = client.select::<User>().load_in::<isize>();
    let _ = client.select::<User>().load_in::<i128>();

    // 3. Narrowing extractions (i8, u8, i16, u16, u32, f32) are disabled on SQLite without `narrow-cast`
    let _ = client.select::<User>().load_in::<i8>();
    let _ = client.select::<User>().load_in::<u8>();
    let _ = client.select::<User>().load_in::<i16>();
    let _ = client.select::<User>().load_in::<u16>();
    let _ = client.select::<User>().load_in::<u32>();
    let _ = client.select::<User>().load_in::<f32>();
    let _ = client.select::<User>().load_in::<Option<f32>>();
    let _ = client.select::<User>().load_in::<(i64, i16)>();
}