use kova::Options;
use models::type_check::user::User;

async fn check() {
    let client = Options::from("").build().unwrap();

    // 1. 64-bit unsigned, pointer-sized, and 128-bit integers cannot be encoded without `narrow-cast`
    let _ = client.statement("SELECT $1").bind(1_u64);
    let _ = client.statement("SELECT $1").bind(1_usize);
    let _ = client.statement("SELECT $1").bind(1_isize);

    // 2. 64-bit unsigned, pointer-sized, and 128-bit integers cannot be decoded without `narrow-cast`
    let _ = client.select::<User>().load_in::<u64>();
    let _ = client.select::<User>().load_in::<usize>();
    let _ = client.select::<User>().load_in::<isize>();

    // 3. Small integers (i8, u8, u16, u32) only support lossless upcast Encode by default, NOT Decode
    let _ = client.select::<User>().load_in::<i8>();
    let _ = client.select::<User>().load_in::<u8>();
    let _ = client.select::<User>().load_in::<u16>();
    let _ = client.select::<User>().load_in::<u32>();
    let _ = client.select::<User>().load_in::<Option<u8>>();
    let _ = client.select::<User>().load_in::<(i32, u16)>();
}

fn main() {}