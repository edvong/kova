#![allow(unreachable_code, dead_code, unused)]

use crate::Result;
use crate::codec::conversion::AppCodec;
use crate::codec::models::EventLog;
use kova::Options;

fn compile_check_fts5() -> Result<()> {
    // Hooking up AppCodec to support custom types like Uuid inside EventLog.
    let client = Options::in_memory().build()?.codec::<AppCodec>();

    // ==========================================
    // 1. BM25 scoring and ordering tests
    // ==========================================
    let _ = client
        .select::<EventLog>()
        .filter(EventLog::description.matches("rust OR orm"))
        // bm25() outputs a Function node; calling desc() directly flips it into an Ordering node.
        .order_by(EventLog::description.bm25().desc())
        .load_as::<EventLog>()?;

    // ==========================================
    // 2. Polymorphic operand tests for Snippet and Highlight
    // ==========================================
    // Simulating dynamic highlight tags read from a config or API (owned Strings).
    let dynamic_tag_left = String::from("<mark class=\"match\">");
    let dynamic_tag_right = String::from("</mark>");

    let _ = client
        .select::<EventLog>()
        .columns((
            EventLog::id,
            // Scenario A: Pure static literals. Zero overhead at compile time, inferred as &'static str.
            EventLog::description.highlight(1, "<b>", "</b>").alias(EventLog::description), // Overriding the original name
            // Scenario B: Mixing dynamic Strings with static literals.
            // IntoOperand flexing here: seamlessly accepts dynamic Strings for the left/right parameters,
            // and converts them into SQL placeholders (?) during rendering to completely dodge SQL injection.
            EventLog::description
                .snippet(
                    1,
                    dynamic_tag_left,  // String
                    dynamic_tag_right, // String
                    "...",             // &'static str
                    64,                // i32
                )
                .alias(EventLog::metadata),
        ))
        .filter(EventLog::description.matches("performance"))
        .order_by(EventLog::description.bm25().asc())
        // Strict tuple boundary checks:
        // EventLog::id -> Uuid
        // Highlight -> String (FTS5 functions strictly return TEXT)
        // Snippet -> String
        .load_in::<(uuid::Uuid, String, String)>()?;

    Ok(())
}
