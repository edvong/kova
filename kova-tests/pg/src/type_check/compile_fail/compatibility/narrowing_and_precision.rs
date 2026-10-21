use kova::Options;
use kova::dsl::Column;
use models::type_check::order::Order;
use models::type_check::user::User;

const I8_COL: Column<i8> = Column::new("dummy", "i8_col", false);
const I16_COL: Column<i16> = Column::new("dummy", "i16_col", false);
const F32_COL: Column<f32> = Column::new("dummy", "f32_col", false);
const I64_COL: Column<i64> = Column::new("dummy", "i64_col", false);
const OPT_I32_COL: Column<Option<i32>> = Column::new("dummy", "opt_i32_col", false);

fn main() {
    let client = Options::from("").build().unwrap();

    // 1. Implicit integer narrowing: i64 -> i32, i32 -> i16, i16 -> i8, Option<i64> -> Option<i32>
    let _ = client.select::<User>().filter(User::id.eq(100_i64));
    let _ = client.select::<User>().filter(I16_COL.eq(100_i32));
    let _ = client.select::<User>().filter(I8_COL.eq(10_i16));
    let _ = client.select::<User>().filter(OPT_I32_COL.eq(Some(100_i64)));

    // 2. Implicit float narrowing: f64 -> Column<f32> and f64 -> Column<i32>
    let _ = client.select::<User>().filter(F32_COL.eq(1.5_f64));
    let _ = client.select::<User>().filter(User::id.eq(1.5_f64));

    // 3. Lossy integer-to-float conversions: i64 -> f64 (53-bit mantissa) and i32 -> f32 (24-bit mantissa)
    let _ = client.select::<Order>().filter(Order::amount.eq(100_i64));
    let _ = client.select::<Order>().filter(Order::discount.eq(100_i64));
    let _ = client.select::<User>().filter(F32_COL.eq(100_i32));

    // 4. Same-width unsigned to signed conversion: u8 -> i8, u16 -> i16, u32 -> i32, u64 -> i64
    let _ = client.select::<User>().filter(I8_COL.eq(10_u8));
    let _ = client.select::<User>().filter(I16_COL.eq(10_u16));
    let _ = client.select::<User>().filter(User::id.eq(10_u32));
    let _ = client.select::<User>().filter(I64_COL.eq(10_u64));

    // 5. Asymmetric column-to-column & arithmetic narrowing
    let _ = client.select::<User>().filter(User::id.eq(Order::amount));
    let _ = client.select::<User>().columns(User::id.add(10_i64));
    let _ = client.select::<User>().filter(User::id.between(1_i64, 10_i64));
}