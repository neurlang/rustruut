pub struct Phonemizer {

}

impl Phonemizer {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn phonemize(&self, text: &str) -> String {
        text.to_string()
    }
}

#[cfg(test)]
mod test;