use std::collections::HashMap;
use crate::interfaces::{PolicyMaxWords, IpaFlavor, DictGetter};
use crate::models::{requests, responses};
use crate::di::DependencyInjection;
use super::rustruut::{Goruut, RustruutError};

/// Trait for phonemizer usecase orchestration (sentence + word).
pub trait PhonemizeUsecase {
    fn sentence(&self, req: requests::PhonemizeSentence) -> Result<responses::PhonemizeSentence, RustruutError>;
}

/// A concrete phonemize usecase implementation.
/// Generic over the three DI traits, keeps them around for orchestration.
pub struct PhonemizeUsecaseImpl<P, I, D>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
{
    policy: P,
    ipa: I,
    dict_getter: D,
    maxwrds: usize,
    goruut: Option<Goruut<P, I, D>>,
    error: Option<RustruutError>,
}

impl<P, I, D> PhonemizeUsecaseImpl<P, I, D>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
{
    /// Construct from DI container.
    pub fn new(di: DependencyInjection<P, I, D>) -> Self {
        let maxwrds = di.policy.get_policy_max_words();
        let models = HashMap::new();
        
        let getter = di.dict_getter.clone();
        let ipa = di.ipa.clone();
	let policy = di.policy.clone();

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
            maxwrds,
            error,
            goruut,
        }
    }
}

impl<P, I, D> PhonemizeUsecase for PhonemizeUsecaseImpl<P, I, D>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
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
