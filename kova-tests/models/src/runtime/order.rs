use kova::Model;

#[derive(Debug, PartialEq, Model)]
pub struct Order {
    #[kova(auto)]
    pub id: i32,
    pub user_id: i32,
    pub amount: f64,
    pub discount: Option<f64>,
    pub is_active: bool,
    pub raw_data: Vec<u8>,
}
