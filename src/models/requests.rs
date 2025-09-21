use serde::{Deserialize, Serialize};

/// Request model for phonemizing a sentence.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PhonemizeSentence {
    #[serde(default)]
    pub ipa_flavors: Vec<String>,
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub languages: Vec<String>,
    #[serde(default)]
    pub sentence: String,
    #[serde(default)]
    pub is_reverse: bool,

    #[serde(default)]
    pub split_sentences: bool,
}

impl PhonemizeSentence {
    /// Init populates fields with sane defaults. Mirrors the Go Init() semantics.
    pub fn init(&mut self) {
        if self.language.is_empty() && !self.languages.is_empty() {
            self.language = self.languages[0].clone();
        }
    }
}
