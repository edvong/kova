#[test]
fn compile_fail_type_boundary() {
    let t = trybuild::TestCases::new();

    let base_dir = "src/type_check/compile_fail";
    if let Ok(entries) = std::fs::read_dir(base_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|s| s.to_str()) {
                    if dir_name != "features" && dir_name != "default_only" {
                        t.compile_fail(format!("{}/{}/*.rs", base_dir, dir_name));
                    }
                }
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                t.compile_fail(path);
            }
        }
    }

    #[cfg(not(feature = "narrow-cast"))]
    {
        t.compile_fail(format!("{}/default_only/*.rs", base_dir));
        t.compile_fail("src/type_check/compile_fail/features/narrow_cast_bounds.rs");
    }

    #[cfg(feature = "narrow-cast")]
    {
        t.pass("src/type_check/compile_fail/features/narrow_cast_bounds.rs");
    }
}