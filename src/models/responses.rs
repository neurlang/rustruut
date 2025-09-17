use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Word-level response. `pos_tags` is stored as raw JSON so that when the
/// response is serialized the raw JSON array is inlined (same behavior as
/// Go's json.RawMessage).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PhonemizeSentenceWord {
    pub clean_word: String,
    pub phonetic: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_tags: Option<serde_json::Value>,

    #[serde(default)]
    pub pre_punct: String,
    #[serde(default)]
    pub post_punct: String,

    #[serde(default)]
    pub is_first: bool,
    #[serde(default)]
    pub is_last: bool,
}


/// Top-level response.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PhonemizeSentence {
    #[serde(default)]
    pub words: Vec<PhonemizeSentenceWord>,

    #[serde(rename = "ErrorWordLimitExceeded", default, skip_serializing_if = "std::ops::Not::not")]
    pub error_word_limit_exceeded: bool,
}

impl PhonemizeSentence {
    pub fn init(&mut self) {
        if self.words.is_empty() {
            self.words = Vec::new();
        }
    }
}
