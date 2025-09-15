use super::usecases::{IPhonemizeUsecase, PhonemizeUsecase};
use super::interfaces::{DictGetter, IpaFlavor, PolicyMaxWords};

pub struct Phonemizer {
    uc: Box<dyn IPhonemizeUsecase>,
}

struct Dummy;

impl IpaFlavor for Dummy {
    fn get_ipa_flavors(&self) -> std::collections::HashMap<String, std::collections::HashMap<String, String>> {
        std::collections::HashMap::new()
    }
}

impl PolicyMaxWords for Dummy {
    fn get_policy_max_words(&self) -> usize {
        usize::MAX
    }
}

impl Phonemizer {
    pub fn new(di: Option<DependencyInjection>) -> Self {
        let mut di = di.unwrap_or_else(DependencyInjection::new);

        // Inject defaults if not already present
        di.add::<dyn DictGetter, _>(super::dicts::DictGetter {});
        di.add::<dyn IpaFlavor, _>(Dummy);
        di.add::<dyn PolicyMaxWords, _>(Dummy);

        let uc = PhonemizeUsecase::new(di);
        Phonemizer { uc: Box::new(uc) }
    }

    pub fn sentence(&self, req: super::requests::PhonemizeSentence) -> super::responses::PhonemizeSentence {
        self.uc.sentence(req)
    }
}

