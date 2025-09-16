use super::*;

#[test]
fn test_hebrew_phonemization() {
    let phonemizer = Phonemizer::new();
    let input = "הַ|כּ֫וֹחַ לְֽשַׁנּוֹת מַתְחִיל בָּ|רֶ֫גַע שֶׁ|בּוֹ אַתָּה מַאֲמִין שֶׁ|זֶּה אֶפְשָׁרִי!";
    let expected = "hakˈoaχ leʃanˈot matχˈil baʁˈeɡa ʃebˈo ʔatˈa maʔamˈin ʃezˈe ʔefʃaʁˈi!";
    
    let result = phonemizer.phonemize(input);
    assert_eq!(result, expected);
}
