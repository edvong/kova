use kova::Cte;

#[derive(Cte)]
pub struct TopSpender {
    pub user_id: i32,
    pub total_amount: f64,
    pub order_count: i64,
}