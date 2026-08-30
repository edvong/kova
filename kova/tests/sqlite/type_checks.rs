use kova::types::Engine;
use kova_core::types::{FromValue, IntoValue};

const fn assert_into_value<'a, T: IntoValue<'a, Engine>>() {}
const fn assert_from_value<T: FromValue<Engine>>() {}

fn check_sqlite_type_conversions<'a>() {
    // --- Native Aligned Types (Mapped to SQLite Int8 / Float8) ---
    assert_into_value::<i64>(); assert_from_value::<i64>();
    assert_into_value::<f64>(); assert_from_value::<f64>();
    assert_into_value::<bool>(); assert_from_value::<bool>();

    // --- Safe Cast Numeric Types ---
    assert_into_value::<i8>(); assert_from_value::<i8>();
    assert_into_value::<u8>(); assert_from_value::<u8>();
    assert_into_value::<i16>(); assert_from_value::<i16>();
    assert_into_value::<u16>(); assert_from_value::<u16>();
    assert_into_value::<i32>(); assert_from_value::<i32>();
    assert_into_value::<u32>(); assert_from_value::<u32>();
    assert_into_value::<u64>(); assert_from_value::<u64>();
    assert_into_value::<isize>(); assert_from_value::<isize>();
    assert_into_value::<usize>(); assert_from_value::<usize>();
    assert_into_value::<f32>(); assert_from_value::<f32>();
    assert_into_value::<&'a i32>(); // Reference types are implemented automatically

    // --- Text Types (Ownership & Zero-copy) ---
    assert_into_value::<String>(); assert_from_value::<String>();
    assert_into_value::<&'a String>();
    assert_into_value::<&'a str>();
    // &'a str intentionally does not implement FromValue to prevent dangling references

    // --- Binary Types ---
    assert_into_value::<Vec<u8>>(); assert_from_value::<Vec<u8>>();
    assert_into_value::<&'a [u8]>();

    // --- Fixed-Size Arrays ---
    assert_into_value::<[u8; 16]>(); assert_from_value::<[u8; 16]>();
    assert_into_value::<[u8; 32]>(); assert_from_value::<[u8; 32]>();

    // --- Options Blanket Impl ---
    assert_into_value::<Option<i32>>(); assert_from_value::<Option<i32>>();
    assert_into_value::<Option<String>>(); assert_from_value::<Option<String>>();
    assert_into_value::<Option<&'a str>>();
}