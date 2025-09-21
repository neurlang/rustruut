use std::collections::HashMap;

pub struct Languages {
    languages_iso639: HashMap<String, String>,
    languages_non_iso639: Vec<String>,
}

impl Languages {
    pub fn new() -> Self {
        let mut languages_iso639 = HashMap::new();
        // Add ISO 639 language codes
        languages_iso639.insert("af".to_string(), "Afrikaans".to_string());
        languages_iso639.insert("am".to_string(), "Amharic".to_string());
        languages_iso639.insert("ar".to_string(), "Arabic".to_string());
        languages_iso639.insert("az".to_string(), "Azerbaijani".to_string());
        languages_iso639.insert("be".to_string(), "Belarusian".to_string());
        languages_iso639.insert("bn".to_string(), "Bengali".to_string());
        languages_iso639.insert("my".to_string(), "Burmese".to_string());
        languages_iso639.insert("ceb".to_string(), "Cebuano".to_string());
        languages_iso639.insert("ce".to_string(), "Chechen".to_string());
        languages_iso639.insert("zh".to_string(), "ChineseMandarin".to_string());
        languages_iso639.insert("cs".to_string(), "Czech".to_string());
        languages_iso639.insert("da".to_string(), "Danish".to_string());
        languages_iso639.insert("nl".to_string(), "Dutch".to_string());
        languages_iso639.insert("dz".to_string(), "Dzongkha".to_string());
        languages_iso639.insert("en".to_string(), "English".to_string());
        languages_iso639.insert("eo".to_string(), "Esperanto".to_string());
        languages_iso639.insert("fa".to_string(), "Farsi".to_string());
        languages_iso639.insert("fi".to_string(), "Finnish".to_string());
        languages_iso639.insert("fr".to_string(), "French".to_string());
        languages_iso639.insert("de".to_string(), "German".to_string());
        languages_iso639.insert("el".to_string(), "Greek".to_string());
        languages_iso639.insert("gu".to_string(), "Gujarati".to_string());
        languages_iso639.insert("ha".to_string(), "Hausa".to_string());
        languages_iso639.insert("he".to_string(), "Hebrew".to_string());
        languages_iso639.insert("hi".to_string(), "Hindi".to_string());
        languages_iso639.insert("hu".to_string(), "Hungarian".to_string());
        languages_iso639.insert("is".to_string(), "Icelandic".to_string());
        languages_iso639.insert("id".to_string(), "Indonesian".to_string());
        languages_iso639.insert("tts".to_string(), "Isan".to_string());
        languages_iso639.insert("it".to_string(), "Italian".to_string());
        languages_iso639.insert("jam".to_string(), "Jamaican".to_string());
        languages_iso639.insert("ja".to_string(), "Japanese".to_string());
        languages_iso639.insert("jv".to_string(), "Javanese".to_string());
        languages_iso639.insert("kk".to_string(), "Kazakh".to_string());
        languages_iso639.insert("ko".to_string(), "Korean".to_string());
        languages_iso639.insert("lb".to_string(), "Luxembourgish".to_string());
        languages_iso639.insert("mk".to_string(), "Macedonian".to_string());
        languages_iso639.insert("ml".to_string(), "Malayalam".to_string());
        languages_iso639.insert("ms".to_string(), "MalayLatin".to_string());
        languages_iso639.insert("mt".to_string(), "Maltese".to_string());
        languages_iso639.insert("mr".to_string(), "Marathi".to_string());
        languages_iso639.insert("mn".to_string(), "Mongolian".to_string());
        languages_iso639.insert("ne".to_string(), "Nepali".to_string());
        languages_iso639.insert("no".to_string(), "Norwegian".to_string());
        languages_iso639.insert("ps".to_string(), "Pashto".to_string());
        languages_iso639.insert("pl".to_string(), "Polish".to_string());
        languages_iso639.insert("pt".to_string(), "Portuguese".to_string());
        languages_iso639.insert("pa".to_string(), "Punjabi".to_string());
        languages_iso639.insert("ro".to_string(), "Romanian".to_string());
        languages_iso639.insert("ru".to_string(), "Russian".to_string());
        languages_iso639.insert("sk".to_string(), "Slovak".to_string());
        languages_iso639.insert("es".to_string(), "Spanish".to_string());
        languages_iso639.insert("sw".to_string(), "Swahili".to_string());
        languages_iso639.insert("sv".to_string(), "Swedish".to_string());
        languages_iso639.insert("ta".to_string(), "Tamil".to_string());
        languages_iso639.insert("te".to_string(), "Telugu".to_string());
        languages_iso639.insert("th".to_string(), "Thai".to_string());
        languages_iso639.insert("bo".to_string(), "Tibetan".to_string());
        languages_iso639.insert("tr".to_string(), "Turkish".to_string());
        languages_iso639.insert("uk".to_string(), "Ukrainian".to_string());
        languages_iso639.insert("ur".to_string(), "Urdu".to_string());
        languages_iso639.insert("ug".to_string(), "Uyghur".to_string());
        languages_iso639.insert("vi".to_string(), "VietnameseNorthern".to_string());
        languages_iso639.insert("zu".to_string(), "Zulu".to_string());
        languages_iso639.insert("hy".to_string(), "Armenian".to_string());
        languages_iso639.insert("eu".to_string(), "Basque".to_string());
        languages_iso639.insert("bg".to_string(), "Bulgarian".to_string());
        languages_iso639.insert("ca".to_string(), "Catalan".to_string());
        languages_iso639.insert("ny".to_string(), "Chichewa".to_string());
        languages_iso639.insert("hr".to_string(), "Croatian".to_string());
        languages_iso639.insert("et".to_string(), "Estonian".to_string());
        languages_iso639.insert("gl".to_string(), "Galician".to_string());
        languages_iso639.insert("ka".to_string(), "Georgian".to_string());
        languages_iso639.insert("km".to_string(), "KhmerCentral".to_string());
        languages_iso639.insert("lo".to_string(), "Lao".to_string());
        languages_iso639.insert("lv".to_string(), "Latvian".to_string());
        languages_iso639.insert("lt".to_string(), "Lithuanian".to_string());
        languages_iso639.insert("sr".to_string(), "Serbian".to_string());
        languages_iso639.insert("tl".to_string(), "Tagalog".to_string());
        languages_iso639.insert("yo".to_string(), "Yoruba".to_string());
        languages_iso639.insert("sq".to_string(), "Albanian".to_string());
        languages_iso639.insert("an".to_string(), "Aragonese".to_string());
        languages_iso639.insert("as".to_string(), "Assamese".to_string());
        languages_iso639.insert("ba".to_string(), "Bashkir".to_string());
        languages_iso639.insert("bpy".to_string(), "BishnupriyaManipuri".to_string());
        languages_iso639.insert("bs".to_string(), "Bosnian".to_string());
        languages_iso639.insert("chr".to_string(), "Cherokee".to_string());
        languages_iso639.insert("cu".to_string(), "Chuvash".to_string());
        languages_iso639.insert("gla".to_string(), "GaelicScottish".to_string());
        languages_iso639.insert("gle".to_string(), "GaelicIrish".to_string());
        languages_iso639.insert("kl".to_string(), "Greenlandic".to_string());
        languages_iso639.insert("gn".to_string(), "Guarani".to_string());
        languages_iso639.insert("ht".to_string(), "HaitianCreole".to_string());
        languages_iso639.insert("haw".to_string(), "Hawaiian".to_string());
        languages_iso639.insert("io".to_string(), "Ido".to_string());
        languages_iso639.insert("ia".to_string(), "Interlingua".to_string());
        languages_iso639.insert("kn".to_string(), "Kannada".to_string());
        languages_iso639.insert("quc".to_string(), "Kiche".to_string());
        languages_iso639.insert("kok".to_string(), "Konkani".to_string());
        languages_iso639.insert("ku".to_string(), "Kurdish".to_string());
        languages_iso639.insert("ky".to_string(), "Kyrgyz".to_string());
        languages_iso639.insert("qdb".to_string(), "LangBelta".to_string());
        languages_iso639.insert("ltg".to_string(), "Latgalian".to_string());
        languages_iso639.insert("la".to_string(), "LatinClassical".to_string());
        languages_iso639.insert("lat".to_string(), "LatinEcclesiastical".to_string());
        languages_iso639.insert("lfn".to_string(), "LinguaFrancaNova".to_string());
        languages_iso639.insert("jbo".to_string(), "Lojban".to_string());
        languages_iso639.insert("smj".to_string(), "LuleSaami".to_string());
        languages_iso639.insert("mi".to_string(), "Maori".to_string());
        languages_iso639.insert("nah".to_string(), "NahuatlCentral".to_string());
        languages_iso639.insert("nci".to_string(), "NahuatlMecayapan".to_string());
        languages_iso639.insert("ncz".to_string(), "NahuatlTetelcingo".to_string());
        languages_iso639.insert("nog".to_string(), "Nogai".to_string());
        languages_iso639.insert("om".to_string(), "Oromo".to_string());
        languages_iso639.insert("pap".to_string(), "Papiamento".to_string());
        languages_iso639.insert("qu".to_string(), "Quechua".to_string());
        languages_iso639.insert("qya".to_string(), "Quenya".to_string());
        languages_iso639.insert("tn".to_string(), "Setswana".to_string());
        languages_iso639.insert("shn".to_string(), "ShanTaiYai".to_string());
        languages_iso639.insert("sjn".to_string(), "Sindarin".to_string());
        languages_iso639.insert("sd".to_string(), "Sindhi".to_string());
        languages_iso639.insert("si".to_string(), "Sinhala".to_string());
        languages_iso639.insert("sl".to_string(), "Slovenian".to_string());
        languages_iso639.insert("tt".to_string(), "Tatar".to_string());
        languages_iso639.insert("tk".to_string(), "Turkmen".to_string());
        languages_iso639.insert("uz".to_string(), "Uzbek".to_string());
        languages_iso639.insert("cyw".to_string(), "WelshNorth".to_string());
        languages_iso639.insert("cys".to_string(), "WelshSouth".to_string());
        languages_iso639.insert("yue".to_string(), "Cantonese".to_string());
        // Add more languages as needed

        let languages_non_iso639 = vec![
            "BengaliDhaka".to_string(),
            "BengaliRahr".to_string(),
            "MalayArab".to_string(),
            "VietnameseCentral".to_string(),
            "VietnameseSouthern".to_string(),
            "EnglishAmerican".to_string(),
            "EnglishBritish".to_string(),
            "NahuatlClassical".to_string(),
            "Hebrew2".to_string(),
            "Hebrew3".to_string(),
            "MinnanTawianese".to_string(),
            "MinnanHokkien".to_string(),
            "MinnanTawianese2".to_string(),
            "MinnanHokkien2".to_string(),
        ];

        Self {
            languages_iso639,
            languages_non_iso639,
        }
    }

    pub fn get_supported_languages(&self) -> Vec<String> {
        let mut languages: Vec<String> = self.languages_iso639.keys().cloned().collect();
        languages.extend(self.languages_non_iso639.clone());
        languages
    }

    pub fn get_all_supported_languages(&self) -> Vec<String> {
        let mut languages: Vec<String> = self.languages_iso639.keys().cloned().collect();
        languages.extend(self.languages_iso639.values().cloned());
        languages.extend(self.languages_non_iso639.clone());
        languages
    }

    pub fn get(&self, value: &str) -> Option<String> {
        if value.len() == 2 || value.len() == 3 {
            self.languages_iso639.get(value).cloned()
        } else {
            Some(value.to_string())
        }
    }
}
