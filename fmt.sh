#!/bin/bash

# ==========================================
# Inline Rustfmt config (replaces rustfmt.toml)
# Edit parameters directly in this TOML block
# ==========================================
TEMP_CONFIG="rustfmt.toml"

cat << 'EOF' > "$TEMP_CONFIG"
unstable_features = true
style_edition="2024"
max_width = 100
chain_width = 100
fn_call_width = 100
struct_lit_width = 100
fn_params_layout = "Compressed"
# where_single_line = true
brace_style = "PreferSameLine"
EOF

# Cleanup the temp config file on exit, error, or interrupt (Ctrl+C)
trap "rm -f $TEMP_CONFIG" EXIT


# ==========================================
# Perl state machine: indent 'where' block
# Adds 4 spaces to 'where' and its generic bounds
# ==========================================
PERL_SCRIPT='
if (/^[ \t]*where\b/) {
    $_ = "    " . $_;
    $in_w = 1 unless /\{|;/;
} elsif ($in_w) {
    if (/^[ \t]*[\{;]/) {
        $in_w = 0;
    } else {
        $_ = "    " . $_;
        $in_w = 0 if /\{|;/;
    }
}'


# ==========================================
# Execution: Single file vs Workspace
# ==========================================
if [ -n "$1" ]; then
    # Case A: Target file provided (e.g., via RustRover shortcut)
    TARGET_FILE="$1"

    # rustfmt automatically picks up the temp rustfmt.toml
    rustfmt +nightly "$TARGET_FILE"
    perl -pi -e "$PERL_SCRIPT" "$TARGET_FILE"
else
    # Case B: No args (running ./fmt.sh in terminal)

    cargo +nightly fmt

    # Find all .rs files, strictly excluding target/ and .git/ dirs
    find . -type f -name "*.rs" \
        -not -path "*/target/*" \
        -not -path "*/.git/*" \
        -exec perl -pi -e "$PERL_SCRIPT" {} +

    echo "Workspace formatting and where-clause indentation complete."
fi