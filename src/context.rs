#[derive(Debug, Clone)]
pub struct Context {
    user_id: u64,
}

impl Context {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}
