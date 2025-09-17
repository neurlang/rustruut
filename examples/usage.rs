/*
cargo run --example usage
*/
use rustruut::RustRuut;

pub fn main() {
    let input = "הַ|כּ֫וֹחַ לְֽשַׁנּוֹת מַתְחִיל בָּ|רֶ֫גַע שֶׁ|בּוֹ אַתָּה מַאֲמִין שֶׁ|זֶּה אֶפְשָׁרִי!";
    let rustruut = RustRuut::new();
    let output = rustruut.phonemize(input);
    println!("{}", output);
}