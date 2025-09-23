//! phonemizer library - explicit constructor injection with traits

// PUBLIC
pub mod di;
pub mod interfaces;
pub mod phonemize;
pub mod models {
    pub mod requests;
    pub mod responses;
}

pub use di::DependencyInjection;
pub use interfaces::{Api, DictGetter, Folder, IpaFlavor, PolicyMaxWords};
pub use models::requests::PhonemizeSentence as PhonemizeSentenceReq;
pub use models::responses::{PhonemizeSentence as PhonemizeSentenceResp, PhonemizeSentenceWord};
pub use phonemize::Phonemizer;

// INTERNAL USE
pub mod usecases {
    pub mod config;
    pub mod executable;
    pub mod phonemize;
    pub mod platform;
    pub mod release;
    pub mod rustruut;
}
