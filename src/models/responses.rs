use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Word-level response. `pos_tags` is stored as raw JSON so that when the
/// response is serialized the raw JSON array is inlined (same behavior as
/// Go's json.RawMessage).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PhonemizeSentenceWord {
    pub clean_word: String,
    pub phonetic: String,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_null_as_none"
    )]
    pub pos_tags: Option<Value>,

    #[serde(default)]
    pub pre_punct: String,
    #[serde(default)]
    pub post_punct: String,

    #[serde(default)]
    pub is_first: bool,
    #[serde(default)]
    pub is_last: bool,
}

/// Custom deserializer to handle null values for Option<Value>
fn deserialize_null_as_none<'de, D>(deserializer: D) -> Result<Option<Value>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Null => Ok(None),
        _ => Ok(Some(value)),
    }
}

/// Custom deserializer to handle null values for Vec<T>
fn deserialize_null_as_empty_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    let opt = Option::<Vec<T>>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// Top-level response.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PhonemizeSentence {
    #[serde(
        default,
        deserialize_with = "deserialize_null_as_empty_vec"
    )]
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
