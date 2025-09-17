use super::*;

#[test]
fn test_hebrew_phonemization() {
    let phonemizer = Phonemizer::new();
    let input = "הַ|כּ֫וֹחַ לְֽשַׁנּוֹת מַתְחִיל בָּ|רֶ֫גַע שֶׁ|בּוֹ אַתָּה מַאֲמִין שֶׁ|זֶּה אֶפְשָׁרִי!";
    let expected = "hakˈoaχ leʃanˈot matχˈil baʁˈeɡa ʃebˈo ʔatˈa maʔamˈin ʃezˈe ʔefʃaʁˈi!";
    
    let result = phonemizer.phonemize(input);
    assert_eq!(result, expected);
}

#[cfg(test)]
mod tests {
    use super::phonemizer::Phonemizer;
    use crate::di::DependencyInjection;
    use crate::models::requests::PhonemizeSentence as Req;

    #[test]
    fn smoke_test_default_di() {
        let di = DependencyInjection::new();
        let p = Phonemizer::new(di);

        let req = Req {
            ipa_flavors: vec![],
            language: "en".to_string(),
            languages: vec![],
            sentence: "hello world".to_string(),
            is_reverse: false,
            split_sentences: false,
        };

        let res = p.sentence(req);
        assert_eq!(res.words.len(), 2);
        assert_eq!(res.words[0].clean_word, "hello");
        assert_eq!(res.words[1].clean_word, "world");
    }

    #[test]
    fn respects_max_words_policy() {
        struct TwoWordPolicy;
        impl crate::interfaces::PolicyMaxWords for TwoWordPolicy {
            fn get_policy_max_words(&self) -> usize { 2 }
        }

        let di = DependencyInjection::with_parts(
            TwoWordPolicy,
            crate::di::default_impls::DummyIpaFlavor::default(),
            crate::di::default_impls::DummyDict::default(),
        );

        let p = Phonemizer::new(di);
        let req = Req {
            ipa_flavors: vec![],
            language: "en".to_string(),
            languages: vec![],
            sentence: "one two three".to_string(),
            is_reverse: false,
            split_sentences: false,
        };

        let res = p.sentence(req);
        assert!(res.error_word_limit_exceeded);
        assert_eq!(res.words.len(), 0);
    }
}
