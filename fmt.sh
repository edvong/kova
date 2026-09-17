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
# Perl state machine (Slurp mode)
# Goal: Indent 'where' block 4 spaces, drop comma, pull up '{'
# ==========================================
PERL_SCRIPT='
# Split file into lines, preserving newlines
my @lines = split(/^/m, $_);
my $in_w = 0;

for (my $i = 0; $i < @lines; $i++) {
    if ($lines[$i] =~ /^[ \t]*where\b/) {
        # Edge case: if rustfmt puts "where ..., {" on a single line
        $lines[$i] =~ s/,[ \t]*\{/ {/;
        $lines[$i] = "    " . $lines[$i];
        $in_w = 1 unless $lines[$i] =~ /\{|;/;
    } elsif ($in_w) {
        if ($lines[$i] =~ /^[ \t]*\{/) {
            $in_w = 0;

            # Found isolated "{", merge it to the previous line
            my $prev = $lines[$i-1];
            my $nl = ($prev =~ s/(\r?\n)$//) ? $1 : "\n";
            $prev =~ s/[ \t]+$//;

            # Case A: comma + comment -> drop comma, insert "{" before comment
            if ($prev =~ s/,[ \t]*(\/\/.*)$/ {$1/) {}
            # Case B: ends with comma -> drop comma, append " {"
            elsif ($prev =~ s/,$//) { $prev .= " {"; }
            # Case C: comment only -> insert "{" before comment
            elsif ($prev =~ s/([ \t]*)(\/\/.*)$/ {$1$2/) {}
            # Case D: normal -> append " {"
            else { $prev .= " {"; }

            $lines[$i-1] = $prev . $nl;

            # Remove "{" from current line, drop line entirely if empty
            $lines[$i] =~ s/^[ \t]*\{//;
            $lines[$i] = "" if $lines[$i] =~ /^[ \t]*\n$/;

        } elsif ($lines[$i] =~ /^[ \t]*;/) {
            $in_w = 0; # Found ";", exit state
        } else {
            # Kill inline comma if rustfmt puts "{" at the end of this constraint line
            $lines[$i] =~ s/,[ \t]*\{/ {/;

            $lines[$i] = "    " . $lines[$i];
            $in_w = 0 if $lines[$i] =~ /\{|;/;
        }
    }
}
# Join lines back into a single string
$_ = join("", @lines);
'

# ==========================================
# Execution: Single file vs Workspace
# ==========================================
if [ -n "$1" ]; then
    # Case A: IDE triggered (Target file provided)
    TARGET_FILE="$1"

    rustfmt +nightly "$TARGET_FILE"
    # -0777 enables Slurp mode (reads whole file into memory)
    perl -i -0777 -pe "$PERL_SCRIPT" "$TARGET_FILE"
else
    # Case B: Terminal triggered (Format workspace)
    cargo +nightly fmt

    find . -type f -name "*.rs" \
        -not -path "*/target/*" \
        -not -path "*/.git/*" \
        -exec perl -i -0777 -pe "$PERL_SCRIPT" {} +

    echo "Workspace formatting, where-clause indentation, and brace folding complete."
fi