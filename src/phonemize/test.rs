#[cfg(test)]
mod tests {
    use crate::Phonemizer;
    use crate::di::DependencyInjection;
    use crate::models::requests::PhonemizeSentence as Req;
    use crate::models::responses::PhonemizeSentence as Resp;

    /// Convert a PhonemizeSentence into a human-readable string with punctuation.
    /// Each word is rendered as `pre_punct + phonetic + post_punct` and joined by spaces.
    fn render_response_with_punct(resp: &Resp) -> String {
        resp.words
            .iter()
            .map(|w| format!("{}{}{}", w.pre_punct, w.phonetic, w.post_punct))
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn test_hebrew_phonemization() -> Result<(), Box<dyn std::error::Error>> {
        let di = DependencyInjection::new();
        let p = Phonemizer::new(di);

        let input = "הַכּ֫וֹחַ לְֽשַׁנּוֹת מַתְחִיל בָּרֶ֫גַע שֶׁבּוֹ אַתָּה מַאֲמִין שֶׁזֶּה אֶפְשָׁרִי!";
        let expected = "כ\u{5bc}\u{5ab}ahox ל\u{5bd}\u{5b0}ʃaˈnot mtˈxile ר\u{5b6}\u{5ab}baˈgaˈa ʃeboˈ aˈta maʔamin ʃzea efʃaˈri!";
        //let best = "hakˈoaχ leʃanˈot matχˈil baʁˈeɡa ʃebˈo ʔatˈa maʔamˈin ʃezˈe ʔefʃaʁˈi!";

        let req = Req {
            ipa_flavors: vec![],
            language: "Hebrew2".to_string(),
            languages: vec![],
            sentence: input.to_string(),
            is_reverse: false,
            split_sentences: false,
        };

        let result = p.sentence(req)?;
        assert_eq!(render_response_with_punct(&result), expected);
        Ok(())
    }

    #[test]
    fn smoke_test_default_di() -> Result<(), Box<dyn std::error::Error>> {
        let di = DependencyInjection::new();
        let p = Phonemizer::new(di);

        let req = Req {
            ipa_flavors: vec![],
            language: "English".to_string(),
            languages: vec![],
            sentence: "hello world".to_string(),
            is_reverse: false,
            split_sentences: false,
        };

        let res = p.sentence(req)?;
        assert_eq!(res.words.len(), 2);
        assert_eq!(res.words[0].clean_word, "hello");
        assert_eq!(res.words[1].clean_word, "world");
        Ok(())
    }

    #[test]
    fn respects_max_words_policy() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Clone)]
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
            language: "English".to_string(),
            languages: vec![],
            sentence: "one two three".to_string(),
            is_reverse: false,
            split_sentences: false,
        };

        let res = p.sentence(req)?;
        assert!(res.error_word_limit_exceeded);
        assert_eq!(res.words.len(), 0);
        Ok(())
    }
}
