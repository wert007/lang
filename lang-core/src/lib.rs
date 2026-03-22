use std::{path::PathBuf, sync::Arc};

pub use language::Language;
pub mod vocab {
    pub use crate::language::vocab::{Vocab, VocabId, VocabSex, VocabWithoutId};
}

mod language;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Image(PathBuf);
