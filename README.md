# rustruut

![:rustruut](https://count.getloli.com/@:rustruut?theme=capoo-1)

Like [goruut](https://github.com/neurlang/goruut) except in Rust


## Run

```console
cargo run --example usage
```

## Test

```console
cargo test
```

## Getting Started

```rust
use rustruut::{DependencyInjection, Phonemizer, models::requests::PhonemizeSentence};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let di = DependencyInjection::new();
    let phonemizer = Phonemizer::new(di);

    let req = PhonemizeSentence {
        ipa_flavors: Vec::new(),
        language: "EnglishAmerican".to_string(),
        languages: Vec::new(),
        sentence: "fast racing car".to_string(),
        is_reverse: false,
        split_sentences: false,
    };

    let resp = phonemizer.sentence(req)?;
    println!("{}", resp.words.iter().map(|w| &w.phonetic).collect::<Vec<_>>().join(" "));

    // Prints: fˈæst ɹˈeɪsɪŋ kˈɑɹ

    // Now, convert it back
    let req_reverse = PhonemizeSentence {
        is_reverse: true,
        ..req
    };

    let resp_reverse = phonemizer.sentence(req_reverse)?;
    println!("{}", resp_reverse.words.iter().map(|w| &w.phonetic).collect::<Vec<_>>().join(" "));

    // Prints: fast racing car

    Ok(())
}
```

> ℹ️ For English, we recommend using `EnglishBritish` or `EnglishAmerican` instead of `English`. These dialect-specific models use high-quality Kokoro Misaki dictionaries and produce better results, especially for reversing IPA back to text.

---

### Uyghur language, our highest quality language

```rust
let req = PhonemizeSentence {
    language: "Uyghur".to_string(),
    sentence: "قىزىل گۈل ئاتا".to_string(),
    is_reverse: false,
    // ... other fields
};

// Prints: qizil gyl ʔɑtɑ

// Now, convert it back
let req_reverse = PhonemizeSentence {
    is_reverse: true,
    // ... other fields same as above
};

// Prints: قىزىل گۈل ئاتا
```

The quality of translation varies across the 136 supported languages.

---

## Advanced Use

### Multi-lingual sentence handling

Use comma (`,`) separated languages in `languages` field. The first language is the preferred language:

```rust
let req = PhonemizeSentence {
    languages: vec!["EnglishBritish".to_string(), "Slovak".to_string()],
    sentence: "hello world ahojte notindictionary!!!!".to_string(),
    // ... other fields
};

// Prints: həlˈoʊ wˈɜɹld aɦɔjcɛ ŋətandəktɪnˈɑːɪ!!!!
```

---

### Numerics handling (English, Arabic)

```rust
let req = PhonemizeSentence {
    language: "EnglishBritish".to_string(),
    sentence: "100 bottles".to_string(),
    // ... other fields
};

// Prints: wˈʌn hˈʌndɹəd bˈɒtəlz
```

---

### Homograph handling (Hebrew3)

```rust
let req = PhonemizeSentence {
    language: "Hebrew3".to_string(),
    sentence: "השרים ביקשו מהשרים לפתוח את הדלתות של בית השרים.".to_string(),
    // ... other fields
};

// Prints: hasaʁˈim bikʃˈu mehasaʁˈim liftˈoaχ ʔˈat hadlatˈot ʃˈel bˈet hasaʁˈim.
```

---

### No punctuation

Punctuation handling is controlled through the response processing. Use the `render_response_with_punct` function to include or exclude punctuation:

```rust
fn render_response_with_punct(resp: &PhonemizeSentenceResponse) -> String {
    resp.words
        .iter()
        .map(|w| format!("{}{}{}", w.pre_punct, w.phonetic, w.post_punct))
        .collect::<Vec<_>>()
        .join(" ")
}

// For no punctuation, use only the phonetic field
resp.words.iter().map(|w| &w.phonetic).collect::<Vec<_>>().join(" ")
```

---

### Force a specific version

Use `CustomVersion` with `DependencyInjection::with_parts()`:

```rust
use rustruut::{DependencyInjection, di};

let di = DependencyInjection::with_parts(
    di::default_impls::DummyPolicy,
    di::default_impls::DummyIpaFlavor,
    di::default_impls::DummyDict,
    di::default_impls::DummyApi,
    di::default_impls::DummyFolder,
    di::custom_impls::CustomVersion::new("0.7.0"),
);
```

Or use the environment variable `RUSTRUUT_VERSION` with default version:

```rust
let di = DependencyInjection {
    policy: di::default_impls::DummyPolicy,
    ipa: di::default_impls::DummyIpaFlavor,
    dict_getter: di::default_impls::DummyDict,
    api: di::default_impls::DummyApi,
    folder: di::default_impls::DummyFolder,
    version: di::custom_impls::CustomVersion::default(),
};
```

---

### Use an online inference API

Possible but need explain (TODO)

---

### Use an extra model

Not possible currently (TODO)

---

### Configure a model download directory for faster startup

Instead of normal DI use this:

```rust
// build DI with defaults (home folder implementations)
let di = DependencyInjection::with_parts(
    di::default_impls::DummyPolicy,
    di::default_impls::DummyIpaFlavor,
    di::default_impls::DummyDict,
    di::default_impls::DummyApi,
    di::custom_impls::CustomFolder::default(),
    di::default_impls::DummyVersion::default(),
);
```
