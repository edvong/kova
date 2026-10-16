#![allow(unreachable_code, dead_code, unused)]

use crate::Result;
use crate::codec::conversion::AppCodec;
use crate::codec::models::EventLog;
use kova::Options;

fn compile_check_fts5() -> Result<()> {
    let client = Options::in_memory().build()?.codec::<AppCodec>();

    // 1. BM25 scoring, custom column weights, comparison, math, and ordering
    let _ = client
        .select::<EventLog>()
        .filter(
            EventLog::description.matches("rust OR orm")
                & EventLog::description.bm25().weights([10.0, 5.0]).lt(0.0)
                & EventLog::description.bm25().mul(-1.0).gt(0.5),
        )
        .order_by(EventLog::description.bm25().weights([10.0, 5.0]).desc())
        .load_as::<EventLog>()?;

    // 2. Polymorphic operand tests for Snippet and Highlight (static &str + dynamic String)
    let dynamic_tag_left = String::from("<mark class=\"match\">");
    let dynamic_tag_right = String::from("</mark>");

    let _ = client
        .select::<EventLog>()
        .columns((
            EventLog::id,
            EventLog::description
                .highlight(1, "<b>", "</b>")
                .alias(EventLog::description),
            EventLog::description
                .snippet(
                    1,
                    &dynamic_tag_left,
                    dynamic_tag_right,
                    "...",
                    64,
                )
                .alias(EventLog::metadata),
        ))
        .filter(EventLog::description.matches("performance"))
        .order_by(EventLog::description.bm25().asc())
        .load_in::<(uuid::Uuid, String, String)>()?;

    Ok(())
}
