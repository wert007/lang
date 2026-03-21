use std::sync::Arc;

use crate::{Image, language::LangId};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VocabId {
    pub(super) lang_id: LangId,
    pub(super) index: usize,
}

#[derive(Debug, Clone)]
pub struct Vocab {
    id: VocabId,
    a: Arc<str>,
    b: Arc<str>,
    plural: Option<Arc<str>>,
    article: Option<Arc<str>>,
    sex: VocabSex,
    image: Option<Image>,
}

#[derive(Debug, Clone, bon::Builder)]
#[builder(derive(Into))]
pub struct VocabWithoutId {
    #[builder(into)]
    a: Arc<str>,
    #[builder(into)]
    b: Arc<str>,
    #[builder(into)]
    plural: Option<Arc<str>>,
    #[builder(into)]
    article: Option<Arc<str>>,
    #[builder(default)]
    sex: VocabSex,
    #[builder(into)]
    image: Option<Image>,
}

impl VocabWithoutId {
    pub fn with_id(self, id: VocabId) -> Vocab {
        Vocab {
            id,
            a: self.a,
            b: self.b,
            plural: self.plural,
            article: self.article,
            sex: self.sex,
            image: self.image,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VocabSex {
    #[default]
    None,
    Male,
    Neutral,
    Female,
}
