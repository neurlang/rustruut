use crate::interfaces::{PolicyMaxWords, IpaFlavor, DictGetter};
use crate::models::{requests, responses};
use crate::di::DependencyInjection;

/// Trait for phonemizer usecase orchestration (sentence + word).
pub trait PhonemizeUsecase {
    fn sentence(&self, req: requests::PhonemizeSentence) -> responses::PhonemizeSentence;
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
        Self {
            policy: di.policy,
            ipa: di.ipa,
            dict_getter: di.dict_getter,
            maxwrds,
        }
    }
}

impl<P, I, D> PhonemizeUsecase for PhonemizeUsecaseImpl<P, I, D>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
{
    fn sentence(&self, mut req: requests::PhonemizeSentence) -> responses::PhonemizeSentence {
        req.init();

        // Split into sentences — dummy implementation, no sentencizer service yet.
        let sentences = if req.split_sentences && !req.is_reverse {
            vec![req.sentence.clone()] // TODO: sentencizer
        } else {
            vec![req.sentence.clone()]
        };

        let mut total_words = 0usize;
        let mut all_words = Vec::new();

        for sentence in sentences {
            // Dummy word splitting
            let tokens: Vec<String> = sentence
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            total_words += tokens.len();
            if total_words > self.maxwrds {
                return responses::PhonemizeSentence {
                    words: Vec::new(),
                    error_word_limit_exceeded: true,
                };
            }

            // Dummy phonemization: just return "DUMMY"
            let mut words = Vec::new();
            for (i, tok) in tokens.iter().enumerate() {
                words.push(responses::PhonemizeSentenceWord {
                    clean_word: tok.clone(),
                    phonetic: "DUMMY".to_string(),
                    pos_tags: None,
                    pre_punct: String::new(),
                    post_punct: String::new(),
                    is_first: i == 0,
                    is_last: i + 1 == tokens.len(),
                });
            }
            all_words.extend(words);
        }

        responses::PhonemizeSentence {
            words: all_words,
            error_word_limit_exceeded: false,
        }
    }
}

/// Default constructor for dummy implementation.
pub fn new_default_usecase() -> impl PhonemizeUsecase {
    let di = DependencyInjection::new();
    PhonemizeUsecaseImpl::new(di)
}
