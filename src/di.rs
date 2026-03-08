use super::interfaces::{Api, DictGetter, Folder, IpaFlavor, PolicyMaxWords, Version};

/// Dependency Injection container with type parameters for all dependencies.
///
/// # Type Parameters
/// - `P`: Policy for maximum words (default: `DummyPolicy`)
/// - `I`: IPA flavor provider (default: `DummyIpaFlavor`)
/// - `D`: Dictionary getter (default: `DummyDict`)
/// - `A`: API path provider (default: `DummyApi`)
/// - `F`: Folder path provider (default: `DummyFolder`)
/// - `V`: Version provider (default: `DummyVersion`)
///
/// # Examples
///
/// Using default implementations:
/// ```
/// use rustruut::DependencyInjection;
/// let di: DependencyInjection = DependencyInjection::new();
/// ```
///
/// Using custom version:
/// ```
/// use rustruut::{DependencyInjection, di};
/// let di = DependencyInjection::with_parts(
///     di::default_impls::DummyPolicy,
///     di::default_impls::DummyIpaFlavor,
///     di::default_impls::DummyDict,
///     di::default_impls::DummyApi,
///     di::default_impls::DummyFolder,
///     di::custom_impls::CustomVersion::new("0.7.0"),
/// );
/// ```
///
/// Using custom API path:
/// ```
/// use rustruut::{DependencyInjection, di};
/// let di = DependencyInjection {
///     policy: di::default_impls::DummyPolicy,
///     ipa: di::default_impls::DummyIpaFlavor,
///     dict_getter: di::default_impls::DummyDict,
///     api: di::custom_impls::CustomApi::new("http://localhost:8080"),
///     folder: di::default_impls::DummyFolder,
///     version: di::default_impls::DummyVersion,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct DependencyInjection<
    P = crate::di::default_impls::DummyPolicy,
    I = crate::di::default_impls::DummyIpaFlavor,
    D = crate::di::default_impls::DummyDict,
    A = crate::di::default_impls::DummyApi,
    F = crate::di::default_impls::DummyFolder,
    V = crate::di::default_impls::DummyVersion,
> where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
    F: Folder,
    V: Version,
{
    pub policy: P,
    pub ipa: I,
    pub dict_getter: D,
    pub api: A,
    pub folder: F,
    pub version: V,
}

impl<P, I, D, A, F, V> DependencyInjection<P, I, D, A, F, V>
where
    P: PolicyMaxWords + Default,
    I: IpaFlavor + Default,
    D: DictGetter + Default,
    A: Api + Default,
    F: Folder + Default,
    V: Version + Default,
{
    pub fn new() -> Self {
        Self {
            policy: Default::default(),
            ipa: Default::default(),
            dict_getter: Default::default(),
            api: Default::default(),
            folder: Default::default(),
            version: Default::default(),
        }
    }
}

impl<P, I, D, A, F, V> DependencyInjection<P, I, D, A, F, V>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
    F: Folder,
    V: Version,
{
    pub fn with_parts(policy: P, ipa: I, dict_getter: D, api: A, folder: F, version: V) -> Self {
        Self {
            policy,
            ipa,
            dict_getter,
            api,
            folder,
            version,
        }
    }
}

pub mod custom_impls {
    use super::super::interfaces::{Api, DictGetter, Folder, IpaFlavor, PolicyMaxWords, Version};

    #[derive(Debug, Clone, Default)]
    pub struct CustomFolder;

    impl Folder for CustomFolder {
        fn get_download_dir(&self) -> Option<&str> {
            Some("")
        }
    }

    #[derive(Debug, Clone)]
    pub struct CustomVersion {
        version: String,
    }

    impl CustomVersion {
        pub fn new(version: &str) -> Self {
            Self {
                version: version.to_string(),
            }
        }
    }

    impl Default for CustomVersion {
        fn default() -> Self {
            Self {
                version: env!("RUSTRUUT_VERSION").to_string(),
            }
        }
    }

    impl Version for CustomVersion {
        fn get_version(&self) -> Option<&str> {
            Some(&self.version)
        }
    }

    #[derive(Debug, Clone)]
    pub struct CustomApi {
        api_path: String,
    }

    impl CustomApi {
        pub fn new(api_path: &str) -> Self {
            Self {
                api_path: api_path.to_string(),
            }
        }
    }

    impl Default for CustomApi {
        fn default() -> Self {
            Self {
                api_path: String::new(),
            }
        }
    }

    impl Api for CustomApi {
        fn get_api_path(&self) -> &str {
            &self.api_path
        }
    }
}

pub mod default_impls {
    use super::super::interfaces::{Api, DictGetter, Folder, IpaFlavor, PolicyMaxWords, Version};
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

    #[derive(Debug, Clone, Default)]
    pub struct DummyFolder;

    impl Folder for DummyFolder {
        fn get_download_dir(&self) -> Option<&str> {
            None
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct DummyVersion;

    impl Version for DummyVersion {
        fn get_version(&self) -> Option<&str> {
            None
        }
    }
}
