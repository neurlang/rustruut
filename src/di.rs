use super::interfaces::{PolicyMaxWords, IpaFlavor, DictGetter, Api};

/// A small, explicit, compile-time DI container that simply holds the three
/// expected components. The generic parameters allow compile-time wiring of
/// concrete implementations.
#[derive(Debug, Clone)]
pub struct DependencyInjection<P = crate::di::default_impls::DummyPolicy,
                               I = crate::di::default_impls::DummyIpaFlavor,
                               D = crate::di::default_impls::DummyDict,
                               A = crate::di::default_impls::DummyApi>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
{
    pub policy: P,
    pub ipa: I,
    pub dict_getter: D,
    pub api: A,
}

impl DependencyInjection<crate::di::default_impls::DummyPolicy,
                         crate::di::default_impls::DummyIpaFlavor,
                         crate::di::default_impls::DummyDict,
                         crate::di::default_impls::DummyApi> {
    /// Creates a DI container pre-filled with dummy implementations.
    pub fn new() -> Self {
        Self {
            policy: crate::di::default_impls::DummyPolicy::default(),
            ipa: crate::di::default_impls::DummyIpaFlavor::default(),
            dict_getter: crate::di::default_impls::DummyDict::default(),
            api: crate::di::default_impls::DummyApi::default(),
        }
    }
}

impl<P, I, D, A> DependencyInjection<P, I, D, A>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
{
    pub fn with_parts(policy: P, ipa: I, dict_getter: D, api: A) -> Self {
        Self { policy, ipa, dict_getter, api }
    }
}

/// Default/dummy implementations are provided here so that callers can use
/// `DependencyInjection::new()` to get sensible defaults.
pub mod default_impls {
    use super::super::interfaces::{PolicyMaxWords, IpaFlavor, DictGetter, Api};
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

    #[derive(Debug, Clone, Default)]
    pub struct DummyApi;

    impl Api for DummyApi {
        fn get_api_path(&self) -> &str {
            ""
        }
    }
}
