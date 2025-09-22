use super::rustruut::{Goruut, RustruutError};
use crate::di::DependencyInjection;
use crate::interfaces::{Api, DictGetter, IpaFlavor, PolicyMaxWords};
use crate::models::{requests, responses};
use std::collections::HashMap;
use std::sync::Arc;

// State enum: either we have a ready Goruut or a stored error from constructor
enum GoruutState<P, I, D, A>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
{
    Ready(Arc<Goruut<P, I, D, A>>),
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
/// Generic over the three DI traits, keeps them around for orchestration.
pub struct PhonemizeUsecaseImpl<P, I, D, A>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
{
    policy: P,
    ipa: I,
    dict_getter: D,
    api: A,
    maxwrds: usize,
    state: Arc<GoruutState<P, I, D, A>>,
}

impl<P, I, D, A> PhonemizeUsecaseImpl<P, I, D, A>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
{
    /// Construct from DI container.
    pub fn new(di: DependencyInjection<P, I, D, A>) -> Self {
        let maxwrds = di.policy.get_policy_max_words();
        let models = HashMap::new();

        let getter = di.dict_getter.clone();
        let ipa = di.ipa.clone();
        let policy = di.policy.clone();
        let api = di.api.clone();

        // Create goruut and handle the result properly
        let goruut_result = Goruut::new(di, None, None, None, models);

        // Extract the result or error
        let state = match goruut_result {
            Ok(g) => Arc::new(GoruutState::Ready(Arc::new(g))),
            Err(e) => Arc::new(GoruutState::Failed(Arc::new(e))),
        };

        Self {
            policy: policy,
            ipa: ipa,
            dict_getter: getter,
            api: api,
            maxwrds,
            state,
        }
    }
}

impl<P, I, D, A> PhonemizeUsecase for PhonemizeUsecaseImpl<P, I, D, A>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
{
    fn sentence(
        &self,
        mut req: requests::PhonemizeSentence,
    ) -> Result<responses::PhonemizeSentence, RustruutError> {
        req.init();

        // Handle the case where goruut might be errored from constructor
        match &*self.state {
            GoruutState::Ready(g) => g.phonemize(req),
            GoruutState::Failed(err) => {
                Err(RustruutError::Generic(format!("goruut not available: {}", err)))
            },
        }
    }
}

/// Default constructor for dummy implementation.
pub fn new_default_usecase() -> impl PhonemizeUsecase {
    let di = DependencyInjection::new();
    PhonemizeUsecaseImpl::new(di)
}
