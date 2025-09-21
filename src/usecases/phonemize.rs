use std::collections::HashMap;
use crate::interfaces::{PolicyMaxWords, IpaFlavor, DictGetter, Api};
use crate::models::{requests, responses};
use crate::di::DependencyInjection;
use super::rustruut::{Goruut, RustruutError};

/// Trait for phonemizer usecase orchestration (sentence + word).
pub trait PhonemizeUsecase {
    fn sentence(&self, req: requests::PhonemizeSentence) -> Result<responses::PhonemizeSentence, RustruutError>;
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
    goruut: Option<Goruut<P, I, D, A>>,
    error: Option<RustruutError>,
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
        let (goruut, error) = match goruut_result {
            Ok(goruut) => (Some(goruut), None),
            Err(error) => (None, Some(error)),
        };

        Self {
            policy: policy,
            ipa: ipa,
            dict_getter: getter,
            api: api,
            maxwrds,
            error,
            goruut,
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
    fn sentence(&self, mut req: requests::PhonemizeSentence) -> Result<responses::PhonemizeSentence, RustruutError> {
        req.init();
        
        // Handle the case where goruut might be None
        match &self.goruut {
            Some(goruut) => goruut.phonemize(req),
            None => Err(RustruutError::Generic("No goruut available and no error stored".to_string()))
        }
    }
}

/// Default constructor for dummy implementation.
pub fn new_default_usecase() -> impl PhonemizeUsecase {
    let di = DependencyInjection::new();
    PhonemizeUsecaseImpl::new(di)
}
