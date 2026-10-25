use kova::Model;

#[derive(Model)]
pub struct Post {
    #[kova(auto)]
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
}
