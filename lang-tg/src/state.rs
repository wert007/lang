use std::sync::Arc;

use parking_lot::Mutex;
use teloxide::types::{Message, UserId};

#[derive(Debug, Clone, Default)]
pub struct State {
    user: Arc<Mutex<Option<UserId>>>,
}

impl State {
    pub fn init(&self, m: &Message) {
        *self.user.lock() = m.from.as_ref().map(|f| f.id);
    }
}
