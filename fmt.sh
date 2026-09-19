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
# Perl state machine (Tri-track)
# Track C: Shield macro_rules! to prevent infinite indents
# Track B: Hunt down any isolated '{' for long signatures
# Track A: Indent 'where' block 4 spaces
# ==========================================
PERL_SCRIPT='
my @lines = split(/^/m, $_);
my $wait_brace = 0;
my $in_w = 0;
my $in_macro = 0;
my $macro_indent = "";

for (my $i = 0; $i < @lines; $i++) {

    # --- Track C: Macro Isolation Shield ---
    # Trigger: Enter macro block
    if (!$in_macro && $lines[$i] =~ /^([ \t]*)macro_rules!\s+\w+/) {
        # Check if it is a single-line macro
        if ($lines[$i] !~ /\{.*\}/) {
            $in_macro = 1;
            $macro_indent = $1;
        }
        next; # Skip processing this line
    }
    # Execution: Ignore everything inside
    if ($in_macro) {
        # Exit state ONLY when matching the exact closing brace indent
        if ($lines[$i] =~ /^$macro_indent[\}\)\]];?\s*$/) {
            $in_macro = 0;
        }
        next; # Skip processing inner contents
    }

    # --- Track B Trigger: Catch start of any item declaration ---
    if ($lines[$i] =~ /^[ \t]*(?:(?:pub(?:\([^)]+\))?|async|const|unsafe|extern|default)[ \t]+)*(fn|struct|enum|union|trait|impl)\b/) {
        $wait_brace = 1;
        $wait_brace = 0 if $lines[$i] =~ /\{|;/;
    }

    # --- Track A Trigger: Handle where clauses ---
    if ($lines[$i] =~ /^[ \t]*where\b/) {
        $in_w = 1;
        $in_w = 0 if $lines[$i] =~ /\{|;/;

        $lines[$i] =~ s/,[ \t]*\{/ {/;
        $lines[$i] = "    " . $lines[$i];
    } elsif ($in_w) {
        if ($lines[$i] !~ /^[ \t]*[\{;]/) {
            $lines[$i] =~ s/,[ \t]*\{/ {/;
            $lines[$i] = "    " . $lines[$i];
        }
    }

    # --- Execution: Fold the orphan brace ---
    if ($wait_brace && $lines[$i] =~ /^[ \t]*\{[ \t]*\r?\n?$/) {
        my $prev = $lines[$i-1];

        my $nl = ($prev =~ s/(\r?\n)$//) ? $1 : "\n";
        $prev =~ s/[ \t]+$//;

        if ($prev =~ s/,[ \t]*(\/\/.*)$/ {$1/) {}
        elsif ($prev =~ s/,$//) { $prev .= " {"; }
        elsif ($prev =~ s/([ \t]*)(\/\/.*)$/ {$1$2/) {}
        else { $prev .= " {"; }

        $lines[$i-1] = $prev . $nl;
        $lines[$i] = "";

        $wait_brace = 0;
        $in_w = 0;
    }

    # --- Safety Fallback Reset ---
    if ($lines[$i] =~ /\{|;/) {
        $wait_brace = 0;
        $in_w = 0;
    }
}
$_ = join("", @lines);
'

# ==========================================
# Execution: Single file vs Workspace
# ==========================================
if [ -n "$1" ]; then
    # Case A: IDE triggered (Target file provided)
    TARGET_FILE="$1"

    # rustfmt +nightly "$TARGET_FILE"
    rustfmt +nightly "$TARGET_FILE" || {
        echo "rustfmt failed on $TARGET_FILE. Aborting perl formatting."
        exit 1
    }

    # -0777 enables Slurp mode (reads whole file into memory)
    perl -i -0777 -pe "$PERL_SCRIPT" "$TARGET_FILE"
else
    # Case B: Terminal triggered (Format workspace)
    # cargo +nightly fmt
    cargo +nightly fmt || {
        echo "cargo fmt failed. Aborting perl formatting."
        exit 1
    }

    find . -type f -name "*.rs" \
        -not -path "*/target/*" \
        -not -path "*/.git/*" \
        -exec perl -i -0777 -pe "$PERL_SCRIPT" {} +

    echo "Workspace formatting, where-clause indentation, and brace folding complete."
fi