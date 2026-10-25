#[derive(Default)]
pub struct Locking {
    pub locks: Vec<LockItem>,
}

pub struct LockItem {
    pub mode: LockMode,
    pub tables: Vec<&'static str>,
    pub wait: Option<WaitPolicy>,
}

pub enum LockMode {
    Update,
    NoKeyUpdate,
    Share,
    KeyShare,
}

pub enum WaitPolicy {
    NoWait,
    SkipLocked,
}
