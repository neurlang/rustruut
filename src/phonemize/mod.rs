use crate::interfaces::{PolicyMaxWords, IpaFlavor, DictGetter};
use crate::models::requests::PhonemizeSentence as Req;
use crate::models::responses::PhonemizeSentence as Resp;
use crate::di::DependencyInjection;
use crate::usecases::phonemize::{PhonemizeUsecase, PhonemizeUsecaseImpl};

#[cfg(test)]
mod test;

/// Public-facing Phonemizer API.
/// It owns a usecase internally and exposes a simple `sentence` method.
pub struct Phonemizer<P, I, D>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
{
    usecase: PhonemizeUsecaseImpl<P, I, D>,
}

impl<P, I, D> Phonemizer<P, I, D>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
{
    /// Construct from DI container.
    pub fn new(di: DependencyInjection<P, I, D>) -> Self {
        let usecase = PhonemizeUsecaseImpl::new(di);
        Self { usecase }
    }

    /// Run phonemization on the sentence request and return a response.
    pub fn sentence(&self, req: Req) -> Resp {
        self.usecase.sentence(req)
    }
}
