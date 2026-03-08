use super::rustruut::{Goruut, RustruutError};
use crate::di::DependencyInjection;
use crate::interfaces::{Api, DictGetter, Folder, IpaFlavor, PolicyMaxWords, Version};
use crate::models::{requests, responses};
use std::collections::HashMap;
use std::sync::Arc;

// State enum: either we have a ready Goruut or a stored error from constructor
enum GoruutState<P, I, D, A, F, V>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
    F: Folder,
    V: Version,
{
    Ready(Arc<Goruut<P, I, D, A, F, V>>),
    Failed(Arc<RustruutError>),
}

/// Trait for phonemizer usecase orchestration (sentence + word).
pub trait PhonemizeUsecase {
    fn sentence(
        &self,
        req: requests::PhonemizeSentence,
    ) -> Result<responses::PhonemizeSentence, RustruutError>;
}

/// A concrete phonemize usecase implementation.
/// Generic over the DI traits, keeps them around for orchestration.
pub struct PhonemizeUsecaseImpl<P, I, D, A, F, V>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
    F: Folder,
    V: Version,
{
    policy: P,
    ipa: I,
    dict_getter: D,
    api: A,
    version: V,
    maxwrds: usize,
    state: Arc<GoruutState<P, I, D, A, F, V>>,
}

impl<P, I, D, A, F, V> PhonemizeUsecaseImpl<P, I, D, A, F, V>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
    F: Folder,
    V: Version,
{
    /// Construct from DI container.
    pub fn new(di: DependencyInjection<P, I, D, A, F, V>) -> Self {
        let maxwrds = di.policy.get_policy_max_words();
        let models = HashMap::new();

        let version_str = di.version.clone().get_version().map(|s| s.to_string());
        let folder_dir = di.folder.get_download_dir().map(|s| s.to_string());
        let api_path = di.api.get_api_path().to_string();

        let version = if version_str.is_none() && api_path.is_empty() {
            None
        } else {
            version_str.as_deref()
        };

        let policy = di.policy.clone();
        let ipa = di.ipa.clone();
        let dict_getter = di.dict_getter.clone();
        let api = di.api.clone();
        let version_provider = di.version.clone();

        let goruut_result = Goruut::new(di, version, folder_dir.as_deref(), None, models);

        let state = match goruut_result {
            Ok(g) => Arc::new(GoruutState::Ready(Arc::new(g))),
            Err(e) => Arc::new(GoruutState::Failed(Arc::new(e))),
        };

        Self {
            policy,
            ipa,
            dict_getter,
            api,
            version: version_provider,
            maxwrds,
            state,
        }
    }
}

impl<P, I, D, A, F, V> PhonemizeUsecase for PhonemizeUsecaseImpl<P, I, D, A, F, V>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
    F: Folder,
    V: Version,
{
    fn sentence(
        &self,
        mut req: requests::PhonemizeSentence,
    ) -> Result<responses::PhonemizeSentence, RustruutError> {
        req.init();

        match &*self.state {
            GoruutState::Ready(g) => g.phonemize(req),
            GoruutState::Failed(err) => Err(RustruutError::Generic(format!(
                "goruut not available: {}",
                err
            ))),
        }
    }
}

/// Default constructor for dummy implementation.
pub fn new_default_usecase() -> impl PhonemizeUsecase {
    let di = crate::di::DependencyInjection::<
        crate::di::default_impls::DummyPolicy,
        crate::di::default_impls::DummyIpaFlavor,
        crate::di::default_impls::DummyDict,
        crate::di::default_impls::DummyApi,
        crate::di::default_impls::DummyFolder,
        crate::di::default_impls::DummyVersion,
    >::new();
    PhonemizeUsecaseImpl::new(di)
}
