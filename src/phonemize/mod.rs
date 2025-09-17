use crate::interfaces::{PolicyMaxWords, IpaFlavor, DictGetter};
use crate::models::requests::PhonemizeSentence as Req;
use crate::models::responses::{PhonemizeSentence as Resp, PhonemizeSentenceWord};

/// Main Phonemizer struct. It is generic over the three traits and stores
/// concrete implementations passed via the DependencyInjection holder.
pub struct Phonemizer<P, I, D>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
{
    policy: P,
    ipa: I,
    dict_getter: D,
}

impl<P, I, D> Phonemizer<P, I, D>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
{
    /// Construct from an explicit, compile-time typed DependencyInjection
    /// container. Use `DependencyInjection::new()` for the dummy defaults.
    pub fn new(di: crate::di::DependencyInjection<P, I, D>) -> Self {
        Self {
            policy: di.policy,
            ipa: di.ipa,
            dict_getter: di.dict_getter,
        }
    }

    /// Run phonemization on the sentence request and return a response.
    /// This is a dummy implementation: it enforces the policy's max-words
    /// and otherwise returns a simple per-token placeholder phonetic.
    pub fn sentence(&self, mut req: Req) -> Resp {
        req.init();

        // Simple tokenization: split on Unicode whitespace
        let tokens: Vec<&str> = req
            .sentence
            .split_whitespace()
            .filter(|t| !t.is_empty())
            .collect();

        let max_words = self.policy.get_policy_max_words();
        if tokens.len() > max_words {
            return Resp {
                words: vec![],
                error_word_limit_exceeded: true,
            };
        }

        let mut words = Vec::with_capacity(tokens.len());
        for (i, tok) in tokens.iter().enumerate() {
            words.push(PhonemizeSentenceWord {
                clean_word: tok.to_string(),
                phonetic: "DUMMY".to_string(),
                pos_tags: None,
                pre_punct: String::new(),
                post_punct: String::new(),
                is_first: i == 0,
                is_last: i + 1 == tokens.len(),
            });
        }

        Resp {
            words,
            error_word_limit_exceeded: false,
        }
    }
}
