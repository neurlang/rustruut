// examples/usage.rs
use rustruut::{
    di, interfaces::Folder, models::requests::PhonemizeSentence, DependencyInjection, Phonemizer,
};
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // example input (Hebrew)
    let input = "הַכּ֫וֹחַ לְֽשַׁנּוֹת מַתְחִיל בָּרֶ֫גַע שֶׁבּוֹ אַתָּה מַאֲמִין שֶׁזֶּה אֶפְשָׁרִי!";

    // build DI with defaults (home folder implementations)
    //let di = DependencyInjection::with_parts(
    //    di::default_impls::DummyPolicy::default(),
    //    di::default_impls::DummyIpaFlavor::default(),
    //    di::default_impls::DummyDict::default(),
    //    di::default_impls::DummyApi::default(),
    //    di::custom_impls::CustomFolder::default(),
    //);
    let di = DependencyInjection::new();

    // construct the phonemizer (types are inferred from the DI)
    let phonemizer = Phonemizer::new(di);

    // build the request model (matches your Rust request struct)
    let req = PhonemizeSentence {
        ipa_flavors: Vec::new(),
        language: "Hebrew2".to_string(),
        languages: Vec::new(),
        sentence: input.to_string(),
        is_reverse: false,
        split_sentences: false,
    };

    // run the phonemizer
    let resp = phonemizer.sentence(req)?;

    // pretty-print JSON response
    println!("{}", serde_json::to_string_pretty(&resp).unwrap());

    Ok(())
}
