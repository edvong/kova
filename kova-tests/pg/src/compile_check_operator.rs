#![allow(unreachable_code, dead_code, unused)]

use crate::Result;
use crate::codec::conversion::AppCodec;
use crate::codec::models::EventLog;
use crate::models::order::Order;
use crate::models::user::User;
use kova::Options;
use kova::dsl::Subselect;
use kova::dsl::func::{All, Any};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

/// Stress Test: JSONB Chain & Advanced JSON Operators
async fn compile_check_jsonb_operators() -> Result<()> {
    let client = Options::from("").build()?.with::<AppCodec>();

    let _ = client
        .select::<EventLog>()
        .filter(
            // 1. json_path (->): Extract child node, then run json_contains (@>) comparison
            EventLog::metadata.json_path("user").json_contains(HashMap::new())
                // 2. json_path_array (#>): Extract deep path, check if key exists (?)
                | EventLog::metadata
                .json_path_array(vec!["deep", "node"])
                .json_has_key("target_key")
                // 3. json_concat (||): Merge JSONB, check if it's contained by target (<@)
                | EventLog::metadata
                .json_concat(HashMap::new())
                .json_contained_by(HashMap::new())
                // 4. json_delete (-): Drop top-level key, check if any keys exist (?|)
                | EventLog::metadata
                .json_delete("temp")
                .json_has_any_keys(vec!["k1".to_string(), "k2".to_string()])
                // 5. json_delete_path (#-): Deep delete, check if all keys exist (?&)
                | EventLog::metadata
                .json_delete_path(vec!["nested".to_string(), "old".to_string()])
                .json_has_all_keys(vec!["k1".to_string()])
                // 6. json_path_exists (@?) & json_path_predicate (@@)
                | EventLog::metadata.json_path_exists("$.user ? (@.age > 18)")
                | EventLog::metadata.json_path_predicate("$.active == true"),
        )
        .load_as::<EventLog>()
        .await?;

    Ok(())
}

/// Stress Test: JSONB Text Extraction & Text Operators
async fn compile_check_jsonb_text_operators() -> Result<()> {
    let client = Options::from("").build()?;

    let _ = client
        .select::<EventLog>()
        .filter(
            // 1. json_path_text (->>): Cast to text, then do equality check (=)
            EventLog::metadata.json_path_text("status").eq("active")
                // 2. ILike (ILIKE): Case-insensitive fuzzy match
                | EventLog::metadata.json_path_text("status").ilike("act%")
                // 3. Matches (~): Regex match
                | EventLog::metadata
                .json_path_text_array(vec!["user".to_string(), "email".to_string()])
                .matches("^[a-z]+@")
                // 4. InList (IN): List inclusion
                | EventLog::metadata.json_path_text("role").in_list(vec!["admin", "user"])
                // 5. IsNotNull (IS NOT NULL): Verify key exists and isn't null
                | EventLog::metadata.json_path_text("deleted_at").is_not_null(),
        )
        .load_as::<EventLog>()
        .await?;

    Ok(())
}

/// Stress Test: ANY / ALL Predicates with Arrays and Subqueries
async fn compile_check_any_all_and_subselect() -> Result<()> {
    let client = Options::from("").build()?.with::<AppCodec>();

    // ==========================================
    // Scenario 1: Native arrays & borrowed slices
    // ==========================================
    let id_vec = vec![1, 2, 3];
    let id_slice: &[i32] = &[4, 5, 6];

    let _ = client
        .select::<User>()
        .filter(
            User::id.eq(Any(id_vec))
                | User::id.ne(All(vec![9, 10]))
                | User::id.gt(Any(vec![0]))
                | User::id.le(All(id_slice)), // Works seamlessly with &[T] zero-overhead slices
        )
        .load_as::<User>()
        .await?;

    // ==========================================
    // Scenario 2: Custom codec types (utilizing the Encode system)
    // ==========================================
    let uuids = vec![Uuid::new_v4(), Uuid::new_v4()];
    let urls = vec![Url::parse("https://github.com").unwrap()];

    let _ = client
        .select::<EventLog>()
        .filter(
            // Hint: Verifies that ANY smoothly uses the Encode escape hatch implemented for
            // Uuid and Url to yield a valid Operand, dodging generic conflicts with Vec<u8> (Blob).
            EventLog::id.eq(Any(uuids)) | EventLog::website.eq(Any(urls)),
        )
        .load_as::<EventLog>()
        .await?;

    // ==========================================
    // Scenario 3: Subselect as ANY/ALL argument (Strict type alignment check)
    // ==========================================
    let _ = client
        .select::<Order>()
        .filter(
            // Subquery: SELECT id FROM user WHERE is_active = true
            // Note: The returned Chain<i32> MUST perfectly align with Order::user_id's type (i32)
            Order::user_id.eq(Any(
                Subselect::<User>::columns(User::id).filter(User::is_active.eq(true)),
            ))
                // Subquery: SELECT amount FROM order WHERE id = 1
                // Note: The returned Chain<f64> MUST perfectly align with Order::amount's type (f64)
                | Order::amount.gt(All(
                Subselect::<Order>::columns(Order::amount).filter(Order::id.eq(1)),
            )),
        )
        .load_as::<Order>()
        .await?;

    Ok(())
}