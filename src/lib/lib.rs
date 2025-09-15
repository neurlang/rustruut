//! Library entrypoint for phonemizer.
//! Implements G2P (grapheme-to-phoneme) IPA phonemizer/dephonemizer
//! for 136+ human languages, including validation and normalization.

pub mod lib;

pub use lib::phonemizer::Phonemizer;
pub use lib::requests;
pub use lib::responses;

