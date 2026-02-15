#[derive(Clone, Debug)]
pub struct KanaSet {
    name: String,
    pub(crate) chars_type: KanaType,
    pub(crate) dictionary: Vec<Vec<(String, String)>>,
    pub(crate) include_map: [bool; 10],
}

#[derive(Clone, Debug)]
pub enum KanaType {
    Hiragana,
    Katakana,
}

impl KanaSet {
    pub fn hiragana() -> Self {
        Self {
            name: "Хирагана".to_string(),
            chars_type: KanaType::Hiragana,
            dictionary: vec![
                vec![
                    (String::from("ぁ"), String::from("a")),
                    (String::from("ぃ"), String::from("i")),
                    (String::from("ぅ"), String::from("u")),
                    (String::from("ぇ"), String::from("e")),
                    (String::from("ぉ"), String::from("o")),
                ],
                vec![
                    (String::from("さ"), String::from("sa")),
                    (String::from("し"), String::from("shi")),
                    (String::from("す"), String::from("su")),
                    (String::from("せ"), String::from("se")),
                    (String::from("そ"), String::from("so")),
                ],
                vec![
                    (String::from("か"), String::from("ka")),
                    (String::from("き"), String::from("ki")),
                    (String::from("く"), String::from("ku")),
                    (String::from("け"), String::from("ke")),
                    (String::from("こ"), String::from("ko")),
                ],
                // Ряд «та» (たちつてと)
                vec![
                    (String::from("た"), String::from("ta")),
                    (String::from("ち"), String::from("chi")),
                    (String::from("つ"), String::from("tsu")),
                    (String::from("て"), String::from("te")),
                    (String::from("と"), String::from("to")),
                ],
                // Ряд «на» (なにぬねの)
                vec![
                    (String::from("な"), String::from("na")),
                    (String::from("に"), String::from("ni")),
                    (String::from("ぬ"), String::from("nu")),
                    (String::from("ね"), String::from("ne")),
                    (String::from("の"), String::from("no")),
                ],
                // Ряд «ха» (はひふへほ)
                vec![
                    (String::from("は"), String::from("ha")),
                    (String::from("ひ"), String::from("hi")),
                    (String::from("ふ"), String::from("fu")),
                    (String::from("へ"), String::from("he")),
                    (String::from("ほ"), String::from("ho")),
                ],
                // Ряд «ма» (まみむめも)
                vec![
                    (String::from("ま"), String::from("ma")),
                    (String::from("み"), String::from("mi")),
                    (String::from("む"), String::from("mu")),
                    (String::from("め"), String::from("me")),
                    (String::from("も"), String::from("mo")),
                ],
                // Ряд «я» (やゆよ) — только 3 символа
                vec![
                    (String::from("や"), String::from("ya")),
                    (String::from("ゆ"), String::from("yu")),
                    (String::from("よ"), String::from("yo")),
                ],
                // Ряд «ра» (らりるれろ)
                vec![
                    (String::from("ら"), String::from("ra")),
                    (String::from("り"), String::from("ri")),
                    (String::from("る"), String::from("ru")),
                    (String::from("れ"), String::from("re")),
                    (String::from("ろ"), String::from("ro")),
                ],
                vec![
                    (String::from("わ"), String::from("wa")),
                    (String::from("を"), String::from("wo")),
                    (String::from("ん"), String::from("n")),
                ],
            ],
            include_map: [true; 10],
        }
    }

    pub fn katakana() -> Self {
        Self {
            name: "Катакана".to_string(),
            chars_type: KanaType::Katakana,
            dictionary: vec![
                vec![
                    (String::from("ァ"), String::from("a")),
                    (String::from("ィ"), String::from("i")),
                    (String::from("ゥ"), String::from("u")),
                    (String::from("ェ"), String::from("e")),
                    (String::from("ォ"), String::from("o")),
                ],
                vec![
                    (String::from("サ"), String::from("sa")),
                    (String::from("シ"), String::from("shi")),
                    (String::from("ス"), String::from("su")),
                    (String::from("セ"), String::from("se")),
                    (String::from("ソ"), String::from("so")),
                ],
                vec![
                    (String::from("カ"), String::from("ka")),
                    (String::from("キ"), String::from("ki")),
                    (String::from("ク"), String::from("ku")),
                    (String::from("ケ"), String::from("ke")),
                    (String::from("コ"), String::from("ko")),
                ],
                // Ряд «та» (タチツテト)
                vec![
                    (String::from("タ"), String::from("ta")),
                    (String::from("チ"), String::from("chi")),
                    (String::from("ツ"), String::from("tsu")),
                    (String::from("テ"), String::from("te")),
                    (String::from("ト"), String::from("to")),
                ],
                // Ряд «на» (ナニヌネノ)
                vec![
                    (String::from("ナ"), String::from("na")),
                    (String::from("ニ"), String::from("ni")),
                    (String::from("ヌ"), String::from("nu")),
                    (String::from("ネ"), String::from("ne")),
                    (String::from("ノ"), String::from("no")),
                ],
                // Ряд «ха» (ハヒフヘホ)
                vec![
                    (String::from("ハ"), String::from("ha")),
                    (String::from("ヒ"), String::from("hi")),
                    (String::from("フ"), String::from("fu")),
                    (String::from("ヘ"), String::from("he")),
                    (String::from("ホ"), String::from("ho")),
                ],
                // Ряд «ма» (マミムメモ)
                vec![
                    (String::from("マ"), String::from("ma")),
                    (String::from("ミ"), String::from("mi")),
                    (String::from("ム"), String::from("mu")),
                    (String::from("メ"), String::from("me")),
                    (String::from("モ"), String::from("mo")),
                ],
                // Ряд «я» (ヤユヨ) — только 3 символа
                vec![
                    (String::from("ヤ"), String::from("ya")),
                    (String::from("ユ"), String::from("yu")),
                    (String::from("ヨ"), String::from("yo")),
                ],
                // Ряд «ра» (ラリルレロ)
                vec![
                    (String::from("ラ"), String::from("ra")),
                    (String::from("リ"), String::from("ri")),
                    (String::from("ル"), String::from("ru")),
                    (String::from("レ"), String::from("re")),
                    (String::from("ロ"), String::from("ro")),
                ],
                vec![
                    (String::from("ワ"), String::from("wa")),
                    (String::from("ヲ"), String::from("wo")),
                    (String::from("ン"), String::from("n")),
                ],
            ],
            include_map: [true; 10],
        }
    }

/*    pub fn next(&mut self) -> (String, String) {
        let current_set = self.list();

        let mut rand = rand::rng();
        let index: u32 = rand.random();
        current_set[index as usize % current_set.len()].clone()
    }*/

    pub fn list(&self) -> Vec<(String, String)> {
        let mut current_set: Vec<(String, String)> = Vec::new();

        for i in 0..10 {
            if self.include_map[i] {
                self.dictionary[i].iter().for_each(|v| {
                    current_set.push(v.clone());
                });
            }
        }

        current_set
    }
}

impl Default for KanaSet {
    fn default() -> Self {
        KanaSet::hiragana()
    }
}
impl PartialEq<Self> for KanaSet {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
