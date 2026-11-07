pub trait Mode {}

pub struct Sync;
impl Mode for Sync {}

pub struct Async;
impl Mode for Async {}
