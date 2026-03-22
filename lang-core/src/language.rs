use crate::language::vocab::{Vocab, VocabId, VocabWithoutId};

pub(crate) mod vocab;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct LangId(u8);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Language {
    lang_id: LangId,
    vocabs: Vec<Vocab>,
}

impl Language {
    pub fn add_vocab(&mut self, v: impl Into<VocabWithoutId>) -> Vocab {
        let v = v
            .into()
            .with_id(VocabId::new(self.lang_id, self.vocabs.len()));
        self.vocabs.push(v.clone());
        v
    }

    pub(crate) fn new(id: u8) -> Self {
        Self {
            lang_id: LangId(id),
            vocabs: vec![],
        }
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

    #[test]
    fn write_and_read_vocab_csv() {
        let mut lang = Language {
            lang_id: LangId(0),
            vocabs: vec![],
        };
        lang.add_vocab(VocabWithoutId::builder().a("Hello").b("Hallo"));
        lang.add_vocab(VocabWithoutId::builder().a("Goodbye").b("Tschüss"));
        let mut w = csv::Writer::from_path("./create_test_lang.csv").unwrap();
        for v in lang.vocabs.clone() {
            w.serialize(v).unwrap();
        }
        drop(w);
        let mut r = csv::Reader::from_path("./create_test_lang.csv").unwrap();
        let mut lang2 = Language::new(0);
        let v: Result<_, _> = r.deserialize().collect();
        let v = v.unwrap();
        lang2.vocabs = v;
        assert_eq!(lang, lang2);
    }
}
