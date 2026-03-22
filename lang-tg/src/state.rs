use std::sync::{Arc, atomic::AtomicUsize};

use lang_core::Language;
use parking_lot::Mutex;
use teloxide::types::{Update, UserId};

#[derive(Debug, Clone, Default)]
pub struct Session {
    vocabs_asked: Arc<AtomicUsize>,
}
impl Session {
    pub(crate) fn asked(&self, _v: &lang_core::vocab::Vocab) -> usize {
        self.vocabs_asked
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}

#[derive(Debug, Clone, Default)]
pub struct State {
    user: Arc<Mutex<Option<UserId>>>,
    language: Arc<Mutex<Option<Language>>>,
    session: Session,
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

    pub(crate) fn session(&self) -> &Session {
        &self.session
    }
}
