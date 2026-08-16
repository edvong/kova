#[derive(Copy, Clone, PartialEq)]
pub enum QueryContext {
    // Insert,
    InsertUpsert, // Trigger `excluded` here
    Update,
    Select,
    Delete,
}
