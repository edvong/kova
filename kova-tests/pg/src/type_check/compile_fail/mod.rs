#[test]
fn compile_fail_type_boundary() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/type_check/compile_fail/**/*.rs");
}
