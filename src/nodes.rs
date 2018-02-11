use regex::{Regex};

lazy_static! {
	static ref SOKUON_RE: Regex = Regex::new(r"ッ(?P<c>[a-z])").unwrap();
}

static FULLWIDTH_ROMAJI: &'static [&'static str] = &[
	"！", "＂", "＃", "＄", "％", "＆",
	"＇", "（", "）", "＊", "＋", "，",
	"－", "．", "／", "０", "１", "２",
	"３", "４", "５", "６", "７", "８",
	"９", "：", "；", "＜", "＝", "＞",
	"？", "＠", "Ａ", "Ｂ", "Ｃ", "Ｄ",
	"Ｅ", "Ｆ", "Ｇ", "Ｈ", "Ｉ", "Ｊ",
	"Ｋ", "Ｌ", "Ｍ", "Ｎ", "Ｏ", "Ｐ",
	"Ｑ", "Ｒ", "Ｓ", "Ｔ", "Ｕ", "Ｖ",
	"Ｗ", "Ｘ", "Ｙ", "Ｚ", "［", "＼",
	"］", "＾", "＿", "｀", "ａ", "ｂ",
	"ｃ", "ｄ", "ｅ", "ｆ", "ｇ", "ｈ",
	"ｉ", "ｊ", "ｋ", "ｌ", "ｍ", "ｎ",
	"ｏ", "ｐ", "ｑ", "ｒ", "ｓ", "ｔ",
	"ｕ", "ｖ", "ｗ", "ｘ", "ｙ", "ｚ",
	"｛", "｜", "｝", "～"
];

static KANA_TO_ROMAJI: &'static [(&'static str, &'static str)] = &[
	("キャ", "kya"), ("キュ", "kyu"), ("キョ", "kyo"),
	("ギャ", "gya"), ("ギュ", "gyu"), ("ギョ", "gyo"),
	("シャ", "sha"), ("シュ", "shu"), ("ショ", "sho"),
	("ジャ", "ja"),  ("ジュ", "ju"),  ("ジェ", "je"),  ("ジョ", "jo"),
	("ファ", "fa"),  ("フェ", "fe"),  ("フォ", "fo"),
	("ヒャ", "hya"), ("ヒョ", "hyo"),
	("ビャ", "bya"), ("ビョ", "byo"),
	("ピャ", "pya"), ("ピョ", "pyo"),
	("リャ", "rya"), ("リュ", "ryu"), ("リョ", "ryo"),
	("チャ", "cha"), ("チュ", "chu"), ("チェ", "che"), ("チョ", "cho"),
	("ミャ", "mya"), ("ミュ", "myu"), ("ミョ", "myo"),
	("ニャ", "nya"), ("ニュ", "nyu"),
	("ティ", "ti"),  ("ディ", "di"),
	("トゥ", "tu"),
	("ドゥ", "du"),
	("ヴァ", "va"), ("ヴィ", "vi"), ("ヴゥ", "vu"), ("ヴェ", "ve"), ("ヴォ", "vo"),

	("ア", "a"),  ("イ", "i"),   ("ウ", "u"),   ("エ", "e"),  ("オ", "o"),
	("カ", "ka"), ("キ", "ki"),  ("ク", "ku"),  ("ケ", "ke"), ("コ", "ko"),
	("ガ", "ga"), ("ギ", "gi"),  ("グ", "gu"),  ("ゲ", "ge"), ("ゴ", "go"),
	("サ", "sa"), ("シ", "shi"), ("ス", "su"),  ("セ", "se"), ("ソ", "so"),
	("ザ", "za"), ("ジ", "ji"),  ("ズ", "zu"),  ("ゼ", "ze"), ("ゾ", "zo"),
	("タ", "ta"), ("チ", "chi"), ("ツ", "tsu"), ("テ", "te"), ("ト", "to"),
	("ダ", "da"), ("ヂ", "ji"),  ("ヅ", "dzu"), ("デ", "de"), ("ド", "do"),
	("ナ", "na"), ("ニ", "ni"),  ("ヌ", "nu"),  ("ネ", "ne"), ("ノ", "no"),
	("ハ", "ha"), ("ヒ", "hi"),  ("フ", "fu"),  ("ヘ", "he"), ("ホ", "ho"),
	("バ", "ba"), ("ビ", "bi"),  ("ブ", "bu"),  ("ベ", "be"), ("ボ", "bo"),
	("パ", "pa"), ("ピ", "pi"),  ("プ", "pu"),  ("ペ", "pe"), ("ポ", "po"),
	("マ", "ma"), ("ミ", "mi"),  ("ム", "mu"),  ("メ", "me"), ("モ", "mo"),
	("ヤ", "ya"),                ("ユ", "yu"),                ("ヨ", "yo"),
	("ラ", "ra"), ("リ", "ri"),  ("ル", "ru"),  ("レ", "re"), ("ロ", "ro"),
	("ワ", "wa"), ("ヰ", "wi"),                 ("ヱ", "we"), ("ヲ", "wo"),
	("ン", "n"),  ("ヴ", "vu"),  ("ー", "̄")
];

pub type ParagraphVec = Vec<Paragraph>;
pub type Paragraph = Vec<ParagraphLine>;
pub type ParagraphLine = Vec<ParagraphWord>;

pub struct ParagraphWord {
	pub original: String,
	pub kana: String,
}

impl ParagraphWord {
	pub fn new() -> ParagraphWord {
		ParagraphWord {
			original: String::with_capacity(32),
			kana: String::with_capacity(32),
		}
	}

	pub fn is_empty(&self) -> bool {
		return self.original.is_empty();
	}

	pub fn romanize(&self) -> String {
		let original = &self.original;

		match original.as_str() {
			"。" => { return String::new(); }
			"、" => { return String::new(); }
			"は" => { return String::from("wa"); }
			"を" => { return String::from("o"); }
			_ => {}
		}

		use std::ascii::AsciiExt;
		if original.is_ascii() {
			return String::new();
		}

		for fullwidth in FULLWIDTH_ROMAJI {
			if fullwidth == original {
				return String::new();
			}
		}

		let mut roma = self.kana.to_string();

		for pair in KANA_TO_ROMAJI {
			roma = roma.replace(pair.0, pair.1);
		}

		return SOKUON_RE.replace_all(&roma, "$c$c");
	}
}
