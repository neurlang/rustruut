// examples/usage.rs
use rustruut::{
    di, models::requests::PhonemizeSentence, DependencyInjection, Phonemizer,
};
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // example input (Hebrew)
    let input = "הַכּ֫וֹחַ לְֽשַׁנּוֹת מַתְחִיל בָּרֶ֫גַע שֶׁבּוֹ אַתָּה מַאֲמִין שֶׁזֶּה אֶפְשָׁרִי!";

    // Example 1: Using default DI with DummyVersion (returns None)
    println!("=== Example 1: Default DI with DummyVersion ===");
    let di: DependencyInjection = DependencyInjection::new();
    let phonemizer = Phonemizer::new(di);

    let req = PhonemizeSentence {
        ipa_flavors: Vec::new(),
        language: "Hebrew2".to_string(),
        languages: Vec::new(),
        sentence: input.to_string(),
        is_reverse: false,
        split_sentences: false,
    };

    let resp = phonemizer.sentence(req)?;
    println!("{}", serde_json::to_string_pretty(&resp).unwrap());

    // Example 2: Using CustomVersion with a specific version string
    println!("\n=== Example 2: DI with CustomVersion ===");
    let di_with_version = DependencyInjection {
        policy: di::default_impls::DummyPolicy,
        ipa: di::default_impls::DummyIpaFlavor,
        dict_getter: di::default_impls::DummyDict,
        api: di::default_impls::DummyApi,
        folder: di::default_impls::DummyFolder,
        version: di::custom_impls::CustomVersion::new("v0.7.0"),
    };

    let phonemizer2 = Phonemizer::new(di_with_version);

    let req2 = PhonemizeSentence {
        ipa_flavors: Vec::new(),
        language: "Hebrew2".to_string(),
        languages: Vec::new(),
        sentence: input.to_string(),
        is_reverse: false,
        split_sentences: false,
    };

    let resp2 = phonemizer2.sentence(req2)?;
    println!("{}", serde_json::to_string_pretty(&resp2).unwrap());

    // Example 3: Using CustomApi with a specific API path
    println!("\n=== Example 3: DI with CustomApi (external API) ===");
    let di_with_api = DependencyInjection {
        policy: di::default_impls::DummyPolicy,
        ipa: di::default_impls::DummyIpaFlavor,
        dict_getter: di::default_impls::DummyDict,
        api: di::custom_impls::CustomApi::new("https://hashtron.cloud"),
        folder: di::default_impls::DummyFolder,
        version: di::default_impls::DummyVersion,
    };

    let phonemizer3 = Phonemizer::new(di_with_api);

    let req3 = PhonemizeSentence {
        ipa_flavors: Vec::new(),
        language: "Hebrew2".to_string(),
        languages: Vec::new(),
        sentence: input.to_string(),
        is_reverse: false,
        split_sentences: false,
    };

    let resp3 = phonemizer3.sentence(req3)?;
    println!("{}", serde_json::to_string_pretty(&resp3).unwrap());

    Ok(())
}
