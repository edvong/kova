use kova::Model;

#[derive(Debug, Clone, PartialEq, Model)]
pub struct User {
    #[kova(auto)]
    pub id: i32,
    pub name: String,
    pub bio: Option<String>,
    pub balance: f64,
    pub is_active: bool,
}