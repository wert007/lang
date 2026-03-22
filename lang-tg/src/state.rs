use std::sync::Arc;

use lang_core::Language;
use parking_lot::Mutex;
use teloxide::types::{Message, Update, UserId};

#[derive(Debug, Clone, Default)]
pub struct State {
    user: Arc<Mutex<Option<UserId>>>,
    language: Arc<Mutex<Option<Language>>>,
}

impl State {
    pub fn init(&self, m: &Update) {
        if self.user.lock().is_some() {
            return;
        }
        *self.user.lock() = m.from().map(|f| f.id);
        let mut l = Language::new(0);
        l.add_vocab(("hello", "hallo"));
        *self.language.lock() = Some(l);
    }

    pub(crate) fn lang(&self) -> Language {
        self.language.lock().clone().unwrap()
    }
}
