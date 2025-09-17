use std::collections::HashMap;

/// Trait that defines the maximum number of words allowed in a request
pub trait PolicyMaxWords: Send + Sync {
    fn get_policy_max_words(&self) -> usize;
}

/// Trait that provides IPA flavor information
pub trait IpaFlavor: Send + Sync {
    fn get_ipa_flavors(&self) -> HashMap<String, HashMap<String, String>>;
}

/// Trait that provides access to embedded/compiled dictionaries (or filesystem)
/// Implementations should return `Some(bytes)` for the given path if available.
pub trait DictGetter: Send + Sync {
    fn get(&self, path: &str) -> Option<Vec<u8>>;
}
