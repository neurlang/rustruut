use super::interfaces::{Api, DictGetter, Folder, IpaFlavor, PolicyMaxWords, Version};
use crate::di::DependencyInjection;
use crate::models::requests::PhonemizeSentence as Req;
use crate::models::responses::PhonemizeSentence as Resp;
use crate::usecases::phonemize::{PhonemizeUsecase, PhonemizeUsecaseImpl};
use crate::usecases::rustruut::RustruutError;

#[cfg(test)]
mod test;

/// Public-facing Phonemizer API.
/// It owns a usecase internally and exposes a simple `sentence` method.
pub struct Phonemizer<P, I, D, A, F, V>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
    F: Folder,
    V: Version,
{
    usecase: PhonemizeUsecaseImpl<P, I, D, A, F, V>,
}

impl<P, I, D, A, F, V> Phonemizer<P, I, D, A, F, V>
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
        let usecase = PhonemizeUsecaseImpl::new(di);
        Self { usecase }
    }

    /// Run phonemization on the sentence request and return a response.
    pub fn sentence(&self, req: Req) -> Result<Resp, RustruutError> {
        self.usecase.sentence(req)
    }
}
