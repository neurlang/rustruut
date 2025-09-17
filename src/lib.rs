pub mod phonemize;

pub struct RustRuut {
    phonemizer: phonemize::Phonemizer,
}

impl RustRuut {
    pub fn new() -> Self {
        Self {
            phonemizer: phonemize::Phonemizer::new(),
        }
    }

    pub fn phonemize(&self, text: &str) -> String {
        self.phonemizer.phonemize(text)
    }
}