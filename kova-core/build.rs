use std::collections::HashSet;
use std::fs;

struct BlockDef {
    source: &'static str,
    marker: &'static str,
    targets: &'static [&'static str],
}

fn main() {
    let distribution_matrix = vec![
        BlockDef {
            source: "src/dsl/function/scalar/abs.rs",
            marker: "COMMON_PROJECTION",
            targets: &[
                "src/dsl/function/scalar/lower.rs",
                "src/dsl/function/scalar/upper.rs",
                "src/dsl/function/scalar/length.rs",
                "src/dsl/function/aggregate/sum.rs",
                "src/dsl/function/aggregate/min.rs",
                "src/dsl/function/aggregate/max.rs",
                "src/dsl/function/aggregate/avg.rs",
                "src/dsl/function/aggregate/count.rs",
            ],
        },
        BlockDef {
            source: "src/dsl/function/scalar/abs.rs",
            marker: "COMMON_COMPARATOR",
            targets: &[
                "src/dsl/function/scalar/lower.rs",
                "src/dsl/function/scalar/upper.rs",
                "src/dsl/function/scalar/length.rs",
                "src/dsl/function/aggregate/sum.rs",
                "src/dsl/function/aggregate/min.rs",
                "src/dsl/function/aggregate/max.rs",
                "src/dsl/function/aggregate/avg.rs",
                "src/dsl/function/aggregate/count.rs",
            ],
        },
        BlockDef {
            source: "src/dsl/function/scalar/lower.rs",
            marker: "STRING_OPS",
            targets: &[
                "src/dsl/function/scalar/upper.rs",
                // Btrim, Concat, JsonbText
            ],
        },
        BlockDef {
            source: "src/dsl/function/aggregate/sum.rs",
            marker: "AGGREGATE_OPS",
            targets: &[
                "src/dsl/function/aggregate/min.rs",
                "src/dsl/function/aggregate/max.rs",
                "src/dsl/function/aggregate/avg.rs",
                "src/dsl/function/aggregate/count.rs",
            ],
        },
    ];

    let mut tracked_files = HashSet::new();
    for def in &distribution_matrix {
        tracked_files.insert(def.source);
        for target in def.targets {
            tracked_files.insert(*target);
        }
    }
    for file in tracked_files {
        println!("cargo:rerun-if-changed={}", file);
    }

    for def in distribution_matrix {
        distribute_block(&def);
    }
}

fn distribute_block(def: &BlockDef) {
    let source_content = fs::read_to_string(def.source)
        .unwrap_or_else(|_| panic!("Failed to read source file: {}", def.source));

    let start_marker = format!("// @SOURCE_{}_START", def.marker);
    let end_marker = format!("// @SOURCE_{}_END", def.marker);

    let start_idx = source_content.find(&start_marker)
        .unwrap_or_else(|| panic!("Missing {} in {}", start_marker, def.source));
    let end_idx = source_content[start_idx..].find(&end_marker)
        .unwrap_or_else(|| panic!("Missing {} in {}", end_marker, def.source)) + start_idx;

    let block_content = &source_content[start_idx + start_marker.len()..end_idx];

    for target in def.targets {
        inject(target, def.marker, block_content);
    }
}

fn inject(target_file: &str, marker: &str, block_content: &str) {
    let original = fs::read_to_string(target_file)
        .unwrap_or_else(|_| panic!("Failed to read target file: {}", target_file));

    let start_marker = format!("// @GENERATED_{}_START", marker);
    let end_marker = format!("// @GENERATED_{}_END", marker);

    let start_idx = original.find(&start_marker)
        .unwrap_or_else(|| panic!("Missing {} in {}", start_marker, target_file));
    let end_idx = original[start_idx..].find(&end_marker)
        .unwrap_or_else(|| panic!("Missing {} in {}", end_marker, target_file)) + start_idx;

    let prefix = &original[..start_idx + start_marker.len()];
    let suffix = &original[end_idx..];

    let new_content = format!("{}{}{}", prefix, block_content, suffix);

    if original != new_content {
        fs::write(target_file, new_content).unwrap();
    }
}