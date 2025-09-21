use crate::interfaces::{PolicyMaxWords, IpaFlavor, DictGetter};

/// A small, explicit, compile-time DI container that simply holds the three
/// expected components. The generic parameters allow compile-time wiring of
/// concrete implementations.
#[derive(Debug)]
pub struct DependencyInjection<P = crate::di::default_impls::DummyPolicy,
                               I = crate::di::default_impls::DummyIpaFlavor,
                               D = crate::di::default_impls::DummyDict>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
{
    pub policy: P,
    pub ipa: I,
    pub dict_getter: D,
}

impl DependencyInjection<crate::di::default_impls::DummyPolicy,
                         crate::di::default_impls::DummyIpaFlavor,
                         crate::di::default_impls::DummyDict> {
    /// Creates a DI container pre-filled with dummy implementations.
    pub fn new() -> Self {
        Self {
            policy: crate::di::default_impls::DummyPolicy::default(),
            ipa: crate::di::default_impls::DummyIpaFlavor::default(),
            dict_getter: crate::di::default_impls::DummyDict::default(),
        }
    }
}

impl<P, I, D> DependencyInjection<P, I, D>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
{
    pub fn with_parts(policy: P, ipa: I, dict_getter: D) -> Self {
        Self { policy, ipa, dict_getter }
    }
}

/// Default/dummy implementations are provided here so that callers can use
/// `DependencyInjection::new()` to get sensible defaults.
pub mod default_impls {
    use super::super::interfaces::{PolicyMaxWords, IpaFlavor, DictGetter};
    use std::collections::HashMap;

    #[derive(Debug, Clone, Default)]
    pub struct DummyPolicy;

    impl PolicyMaxWords for DummyPolicy {
        fn get_policy_max_words(&self) -> usize {
            9999999
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct DummyIpaFlavor;

    impl IpaFlavor for DummyIpaFlavor {
        fn get_ipa_flavors(&self) -> HashMap<String, HashMap<String, String>> {
            HashMap::new()
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct DummyDict;

    impl DictGetter for DummyDict {
        fn get(&self, _path: &str) -> Option<Vec<u8>> {
            None
        }
    }
}
