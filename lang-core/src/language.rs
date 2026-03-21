use crate::language::vocab::{Vocab, VocabId, VocabWithoutId};

pub(crate) mod vocab;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LangId(usize);

pub struct Language {
    lang_id: LangId,
    vocabs: Vec<Vocab>,
}

impl Language {
    pub fn add_vocab(&mut self, v: impl Into<VocabWithoutId>) -> Vocab {
        let v = v.into().with_id(VocabId {
            lang_id: self.lang_id,
            index: self.vocabs.len(),
        });
        self.vocabs.push(v.clone());
        v
    }
}

#[cfg(test)]
mod lang_test {
    use super::*;

    #[test]
    fn compiles() {
        let mut lang = Language {
            lang_id: LangId(0),
            vocabs: vec![],
        };
        lang.add_vocab(VocabWithoutId::builder().a("a").b("b"));
    }
}
