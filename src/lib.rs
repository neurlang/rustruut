//! phonemizer library - explicit constructor injection with traits

// PUBLIC
pub mod interfaces;
pub mod di;
pub mod phonemize;
pub mod models {
    pub mod requests;
    pub mod responses;
}

pub use phonemize::Phonemizer;
pub use interfaces::{PolicyMaxWords, IpaFlavor, DictGetter};
pub use di::DependencyInjection;
pub use models::requests::PhonemizeSentence as PhonemizeSentenceReq;
pub use models::responses::{PhonemizeSentence as PhonemizeSentenceResp, PhonemizeSentenceWord};

// INTERNAL USE
pub mod usecases {
    pub mod phonemize;
}
