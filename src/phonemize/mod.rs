use super::interfaces::{PolicyMaxWords, IpaFlavor, DictGetter, Api};
use crate::models::requests::PhonemizeSentence as Req;
use crate::models::responses::PhonemizeSentence as Resp;
use crate::di::DependencyInjection;
use crate::usecases::phonemize::{PhonemizeUsecase, PhonemizeUsecaseImpl};
use crate::usecases::rustruut::{RustruutError};

#[cfg(test)]
mod test;

/// Public-facing Phonemizer API.
/// It owns a usecase internally and exposes a simple `sentence` method.
pub struct Phonemizer<P, I, D, A>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
{
    usecase: PhonemizeUsecaseImpl<P, I, D, A>,
}

impl<P, I, D, A> Phonemizer<P, I, D, A>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
{
    /// Construct from DI container.
    pub fn new(di: DependencyInjection<P, I, D, A>) -> Self {
        let usecase = PhonemizeUsecaseImpl::new(di);
        Self { usecase }
    }

    /// Run phonemization on the sentence request and return a response.
    pub fn sentence(&self, req: Req) -> Result<Resp, RustruutError> {
        self.usecase.sentence(req)
    }
}
