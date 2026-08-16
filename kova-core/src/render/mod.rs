
pub trait Render {
    fn render(&self, sql: &mut String);
}