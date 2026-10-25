use kova::Model;

#[derive(Model)]
pub struct Comment {
    #[kova(auto)]
    pub id: i32,
    pub post_id: i32,
    pub user_id: i32,
    pub content: String,
}