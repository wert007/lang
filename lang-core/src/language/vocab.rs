use crate::{Image, language::LangId};

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct VocabId(u64);
impl VocabId {
    pub(super) fn new(lang_id: LangId, len: usize) -> Self {
        Self(((lang_id.0 as u64) << 56) | (len as u64) & 0x00FFFFFF_FFFFFFFF)
    }
}

type String = flexstr::SharedStr;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Vocab {
    id: VocabId,
    a: String,
    b: String,
    plural: Option<String>,
    article: Option<String>,
    sex: VocabSex,
    image: Option<Image>,
}

#[derive(Debug, Clone, bon::Builder)]
#[builder(derive(Into))]
pub struct VocabWithoutId {
    #[builder(into)]
    a: String,
    #[builder(into)]
    b: String,
    #[builder(into)]
    plural: Option<String>,
    #[builder(into)]
    article: Option<String>,
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

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum VocabSex {
    #[default]
    None,
    Male,
    Neutral,
    Female,
}
