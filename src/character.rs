use ab_glyph::{FontRef, PxScale};

use crate::utils::utils::sort_character_brightness;

pub struct FontData<'a> {
    pub character_list: Vec<char>,
    pub font: FontRef<'a>,
    pub font_data: &'static [u8],
    pub scale: PxScale,
    pub character: char,
}

#[derive(PartialEq)]
pub enum CharacterType {
    Simple,     // simple symbol chracters
    Complex,    // complex symbol characters
    En,         // English
    Ru,         // Russian
    De,         // German
    Fr,         // French
    Es,         // Spanish
    It,         // Italian
    Pt,         // Portuguese
    Pl,         // Polish
    Hi,         // Hindi
    Ar,         // Arabic
    Bn,         // Bengali
    ZhZhuyin,   // Chinese Zhuyin
    JpHiragana, // Japanese Hiragana
    JpKatakana, // Japanese Katakana
    Kr,         // Korean
    Vi,         // Vietnamese
}

impl CharacterType {
    fn get_character_array(&self) -> Vec<char> {
        match self {
            CharacterType::Simple => vec![' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'],
            CharacterType::Complex => vec![
                ' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~',
                '+', '_', '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '/', 't', 'f', 'j',
                'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O',
                'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&',
                '8', '%', 'B', '@', '$',
            ],
            CharacterType::En => vec![
                'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                'Y', 'y', 'Z', 'z',
            ],
            CharacterType::Ru => vec![
                'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ё', 'ё', 'Ж', 'ж',
                'З', 'з', 'И', 'и', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о',
                'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ф', 'ф', 'Х', 'х', 'Ц', 'ц',
                'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ы', 'ы', 'Ь', 'ь', 'Э', 'э', 'Ю', 'ю',
                'Я', 'я',
            ],
            CharacterType::De => vec![
                'A', 'a', 'Ä', 'ä', 'B', 'b', 'ß', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G',
                'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O',
                'o', 'Ö', 'ö', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'Ü',
                'ü', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z',
            ],
            CharacterType::Fr => vec![
                'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                'Y', 'y', 'Z', 'z', 'Æ', 'æ', 'Œ', 'œ', 'Ç', 'ç', 'À', 'à', 'Â', 'â', 'É', 'é',
                'È', 'è', 'Ê', 'ê', 'Ë', 'ë', 'Î', 'î', 'Î', 'ï', 'Ô', 'ô', 'Û', 'û', 'Ù', 'ù',
                'Ÿ', 'ÿ',
            ],
            CharacterType::Es => vec![
                'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                'Y', 'y', 'Z', 'z', 'Ñ', 'ñ', 'á', 'é', 'í', 'ó', 'ú', '¡', '¿',
            ],
            CharacterType::It => vec![
                'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                'Y', 'y', 'Z', 'z', 'À', 'È', 'à', 'è', 'é', 'ì', 'ò', 'ù',
            ],
            CharacterType::Pt => vec![
                'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                'Y', 'y', 'Z', 'z', 'à', 'À', 'á', 'Á', 'â', 'Â', 'ã', 'Ã', 'ç', 'Ç', 'é', 'É',
                'ê', 'Ê', 'í', 'Í', 'ó', 'Ó', 'ô', 'Ô', 'õ', 'Õ', 'ú', 'Ú',
            ],
            CharacterType::Pl => vec![
                'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Z', 'z', 'Ą', 'ą',
                'Ę', 'ę', 'Ó', 'ó', 'Ł', 'ł', 'Ń', 'ń', 'Ż', 'ż', 'Ś', 'ś', 'Ć', 'ć', 'Ź', 'ź',
            ],
            CharacterType::Hi => vec![
                'अ', 'आ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ', 'ए', 'ऐ', 'ओ', 'औ', 'क', 'ख', 'ग', 'घ', 'ङ',
                'च', 'छ', 'ज', 'झ', 'ञ', 'ट', 'ठ', 'ड', 'ढ', 'ण', 'त', 'थ', 'द', 'ध', 'न', 'प',
                'फ', 'ब', 'भ', 'म', 'य', 'र', 'ल', 'व', 'श', 'ष', 'स', 'ह', 'ः', '।', 'ऍ', 'ऑ',
                '०', '१', '२', '३', '४', '५', '६', '७', '८', '९',
            ],
            CharacterType::Ar => vec![
                'ا', 'ب', 'ت', 'ث', 'ج', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط',
                'ظ', 'ع', 'غ', 'ف', 'ق', 'ك', 'ل', 'م', 'ن', 'ه', 'و', 'ي', 'أ', 'إ', 'ؤ', 'ئ',
                'ء',
            ],
            CharacterType::Bn => vec![
                'অ', 'আ', 'ই', 'ঈ', 'উ', 'ঊ', 'ঋ', 'এ', 'ঐ', 'ও', 'ঔ', 'ক', 'খ', 'গ', 'ঘ', 'ঙ',
                'চ', 'ছ', 'জ', 'ঝ', 'ঞ', 'ট', 'ঠ', 'ড', 'ঢ', 'ণ', 'ত', 'থ', 'দ', 'ধ', 'ন', 'প',
                'ফ', 'ব', 'ভ', 'ম', 'য', 'র', 'ল', 'শ', 'ষ', 'স', 'হ', 'ড', 'ঢ', 'য', 'ক', 'খ',
                'গ', 'জ', 'ফ', 'ঃ',
            ],
            CharacterType::ZhZhuyin => vec![
                'ㄅ', 'ㄆ', 'ㄇ', 'ㄈ', 'ㄉ', 'ㄊ', 'ㄋ', 'ㄌ', 'ㄍ', 'ㄎ', 'ㄏ', 'ㄐ', 'ㄑ', 'ㄒ',
                'ㄓ', 'ㄔ', 'ㄕ', 'ㄖ', 'ㄗ', 'ㄘ', 'ㄙ', 'ㄧ', 'ㄨ', 'ㄩ', 'ㄚ', 'ㄛ', 'ㄜ', 'ㄝ',
                'ㄞ', 'ㄟ', 'ㄠ', 'ㄡ', 'ㄢ', 'ㄣ', 'ㄤ', 'ㄥ', 'ㄦ', 'ㄭ',
            ],
            CharacterType::JpHiragana => vec![
                'あ', 'い', 'う', 'え', 'お', 'か', 'き', 'く', 'け', 'こ', 'さ', 'し', 'す', 'せ',
                'そ', 'た', 'ち', 'つ', 'て', 'と', 'な', 'に', 'ぬ', 'ね', 'の', 'は', 'ひ', 'ふ',
                'へ', 'ほ', 'ま', 'み', 'む', 'め', 'も', 'や', 'ゆ', 'よ', 'ら', 'り', 'る', 'れ',
                'ろ', 'わ', 'を', 'ん',
            ],
            CharacterType::JpKatakana => vec![
                'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ', 'ス', 'セ',
                'ソ', 'タ', 'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ', 'ヒ', 'フ',
                'ヘ', 'ホ', 'マ', 'ミ', 'ム', 'メ', 'モ', 'ヤ', 'ユ', 'ヨ', 'ラ', 'リ', 'ル', 'レ',
                'ロ', 'ワ', 'ヲ', 'ン',
            ],
            CharacterType::Kr => vec![
                'ㄱ', 'ㄴ', 'ㄷ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅅ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
                'ㅏ', 'ㅑ', 'ㅓ', 'ㅕ', 'ㅗ', 'ㅛ', 'ㅜ', 'ㅠ', 'ㅡ', 'ㅣ',
            ],
            CharacterType::Vi => vec![
                'A', 'a', 'À', 'à', 'Á', 'á', 'Ả', 'ả', 'Ã', 'ã', 'Ạ', 'ạ', 'Ă', 'ă', 'Ằ', 'ằ',
                'Ắ', 'ắ', 'Ẳ', 'ẳ', 'Ẵ', 'ẵ', 'Ặ', 'ặ', 'Â', 'â', 'Ầ', 'ầ', 'Ấ', 'ấ', 'Ẩ', 'ẩ',
                'Ẫ', 'ẫ', 'Ậ', 'ậ', 'B', 'b', 'C', 'c', 'D', 'd', 'Đ', 'đ', 'E', 'e', 'È', 'è',
                'É', 'é', 'Ẻ', 'ẻ', 'Ẽ', 'ẽ', 'Ẹ', 'ẹ', 'Ê', 'ê', 'Ề', 'ề', 'Ế', 'ế', 'Ể', 'ể',
                'Ễ', 'ễ', 'Ệ', 'ệ', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'Ì', 'ì', 'Í', 'í',
                'Ỉ', 'ỉ', 'Ĩ', 'ĩ', 'Ị', 'ị', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                'O', 'o', 'Ò', 'ò', 'Ó', 'ó', 'Ỏ', 'ỏ', 'Õ', 'õ', 'Ọ', 'ọ', 'Ô', 'ô', 'Ồ', 'ồ',
                'Ố', 'ố', 'Ổ', 'ổ', 'Ỗ', 'ỗ', 'Ộ', 'ộ', 'Ơ', 'ơ', 'Ờ', 'ờ', 'Ớ', 'ớ', 'Ở', 'ở',
                'Ỡ', 'ỡ', 'Ợ', 'ợ', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u',
                'Ù', 'ù', 'Ú', 'ú', 'Ủ', 'ủ', 'Ũ', 'ũ', 'Ụ', 'ụ', 'Ư', 'ư', 'Ừ', 'ừ', 'Ứ', 'ứ',
                'Ử', 'ử', 'Ữ', 'ữ', 'Ự', 'ự', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Ỳ', 'ỳ',
                'Ý', 'ý', 'Ỷ', 'ỷ', 'Ỹ', 'ỹ', 'Ỵ', 'ỵ', 'Z', 'z',
            ],
        }
    }

    pub fn get_character_data<'a>(&self) -> FontData<'a> {
        match self {
            CharacterType::Simple => {
                let font_data: &'static [u8] =
                    include_bytes!("utils/fonts/dejavu/DejaVuSansMono-Bold.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: self.get_character_array(),
                    font,
                    font_data,
                    scale: PxScale { x: 20.0, y: 20.0 },
                    character: '.',
                };
                return font_data_struct;
            }
            CharacterType::Complex => {
                let font_data: &'static [u8] =
                    include_bytes!("utils/fonts/dejavu/DejaVuSansMono-Bold.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: self.get_character_array(),
                    font,
                    font_data,
                    scale: PxScale { x: 20.0, y: 20.0 },
                    character: '.',
                };
                return font_data_struct;
            }
            CharacterType::En => {
                let font_data: &'static [u8] =
                    include_bytes!("utils/fonts/dejavu/DejaVuSansMono-Bold.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: sort_character_brightness(
                        self.get_character_array(),
                        font_data,
                        PxScale { x: 20.0, y: 20.0 },
                    ),
                    font,
                    font_data,
                    scale: PxScale { x: 20.0, y: 20.0 },
                    character: 'A',
                };
                return font_data_struct;
            }
            CharacterType::Ru => {
                let font_data: &'static [u8] =
                    include_bytes!("utils/fonts/dejavu/DejaVuSansMono-Bold.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: sort_character_brightness(
                        self.get_character_array(),
                        font_data,
                        PxScale { x: 20.0, y: 20.0 },
                    ),
                    font,
                    font_data,
                    scale: PxScale { x: 20.0, y: 20.0 },
                    character: 'Ш',
                };
                return font_data_struct;
            }
            CharacterType::De => {
                let font_data: &'static [u8] =
                    include_bytes!("utils/fonts/dejavu/DejaVuSansMono-Bold.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: sort_character_brightness(
                        self.get_character_array(),
                        font_data,
                        PxScale { x: 20.0, y: 20.0 },
                    ),
                    font,
                    font_data,
                    scale: PxScale { x: 20.0, y: 20.0 },
                    character: 'Ü',
                };
                return font_data_struct;
            }
            CharacterType::Fr => {
                let font_data: &'static [u8] =
                    include_bytes!("utils/fonts/dejavu/DejaVuSansMono-Bold.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: sort_character_brightness(
                        self.get_character_array(),
                        font_data,
                        PxScale { x: 20.0, y: 20.0 },
                    ),
                    font,
                    font_data,
                    scale: PxScale { x: 20.0, y: 20.0 },
                    character: 'Ë',
                };
                return font_data_struct;
            }
            CharacterType::Es => {
                let font_data: &'static [u8] =
                    include_bytes!("utils/fonts/dejavu/DejaVuSansMono-Bold.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: sort_character_brightness(
                        self.get_character_array(),
                        font_data,
                        PxScale { x: 20.0, y: 20.0 },
                    ),
                    font,
                    font_data,
                    scale: PxScale { x: 20.0, y: 20.0 },
                    character: 'Ñ',
                };
                return font_data_struct;
            }
            CharacterType::It => {
                let font_data: &'static [u8] =
                    include_bytes!("utils/fonts/dejavu/DejaVuSansMono-Bold.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: sort_character_brightness(
                        self.get_character_array(),
                        font_data,
                        PxScale { x: 20.0, y: 20.0 },
                    ),
                    font,
                    font_data,
                    scale: PxScale { x: 20.0, y: 20.0 },
                    character: 'È',
                };
                return font_data_struct;
            }
            CharacterType::Pt => {
                let font_data: &'static [u8] =
                    include_bytes!("utils/fonts/dejavu/DejaVuSansMono-Bold.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: sort_character_brightness(
                        self.get_character_array(),
                        font_data,
                        PxScale { x: 20.0, y: 20.0 },
                    ),
                    font,
                    font_data,
                    scale: PxScale { x: 20.0, y: 20.0 },
                    character: 'Ó',
                };
                return font_data_struct;
            }
            CharacterType::Pl => {
                let font_data: &'static [u8] =
                    include_bytes!("utils/fonts/dejavu/DejaVuSansMono-Bold.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: sort_character_brightness(
                        self.get_character_array(),
                        font_data,
                        PxScale { x: 20.0, y: 20.0 },
                    ),
                    font,
                    font_data,
                    scale: PxScale { x: 20.0, y: 20.0 },
                    character: 'ł',
                };
                return font_data_struct;
            }
            CharacterType::Hi => {
                let font_data: &'static [u8] =
                    include_bytes!("utils/fonts/monotty/monotty-dev2.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: sort_character_brightness(
                        self.get_character_array(),
                        font_data,
                        PxScale { x: 20.0, y: 20.0 },
                    ),
                    font,
                    font_data,
                    scale: PxScale { x: 20.0, y: 20.0 },
                    character: 'अ',
                };
                return font_data_struct;
            }
            CharacterType::Ar => {
                let font_data: &'static [u8] =
                    include_bytes!("utils/fonts/azarmehr/AzarMehrMonospacedSansBold.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: sort_character_brightness(
                        self.get_character_array(),
                        font_data,
                        PxScale { x: 40.0, y: 20.0 },
                    ),
                    font,
                    font_data,
                    scale: PxScale { x: 40.0, y: 20.0 },
                    character: 'ظ',
                };
                return font_data_struct;
            }
            CharacterType::Bn => {
                let font_data: &'static [u8] = include_bytes!("utils/fonts/mitra/mitra.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: sort_character_brightness(
                        self.get_character_array(),
                        font_data,
                        PxScale { x: 30.0, y: 20.0 },
                    ),
                    font,
                    font_data,
                    scale: PxScale { x: 30.0, y: 20.0 },
                    character: 'আ',
                };
                return font_data_struct;
            }
            CharacterType::ZhZhuyin => {
                let font_data: &'static [u8] = include_bytes!("utils/fonts/simsun/SimSun-Bold.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: sort_character_brightness(
                        self.get_character_array(),
                        font_data,
                        PxScale { x: 10.0, y: 20.0 },
                    ),
                    font,
                    font_data,
                    scale: PxScale { x: 10.0, y: 20.0 },
                    character: 'ㄠ',
                };
                return font_data_struct;
            }
            CharacterType::JpHiragana => {
                let font_data: &'static [u8] =
                    include_bytes!("utils/fonts/arial-unicode/Arial-Unicode-Bold.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: sort_character_brightness(
                        self.get_character_array(),
                        font_data,
                        PxScale { x: 15.0, y: 20.0 },
                    ),
                    font,
                    font_data,
                    scale: PxScale { x: 15.0, y: 20.0 },
                    character: 'あ',
                };
                return font_data_struct;
            }
            CharacterType::JpKatakana => {
                let font_data: &'static [u8] =
                    include_bytes!("utils/fonts/arial-unicode/Arial-Unicode-Bold.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: sort_character_brightness(
                        self.get_character_array(),
                        font_data,
                        PxScale { x: 15.0, y: 20.0 },
                    ),
                    font,
                    font_data,
                    scale: PxScale { x: 15.0, y: 20.0 },
                    character: 'ア',
                };
                return font_data_struct;
            }
            CharacterType::Kr => {
                let font_data: &'static [u8] =
                    include_bytes!("utils/fonts/arial-unicode/Arial-Unicode-Bold.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: sort_character_brightness(
                        self.get_character_array(),
                        font_data,
                        PxScale { x: 15.0, y: 20.0 },
                    ),
                    font,
                    font_data,
                    scale: PxScale { x: 15.0, y: 20.0 },
                    character: 'ㅠ',
                };
                return font_data_struct;
            }
            CharacterType::Vi => {
                let font_data: &'static [u8] =
                    include_bytes!("utils/fonts/roboto/RobotoMono-Bold.ttf");
                let font = FontRef::try_from_slice(font_data).unwrap();
                let font_data_struct: FontData = FontData {
                    character_list: sort_character_brightness(
                        self.get_character_array(),
                        font_data,
                        PxScale { x: 22.5, y: 20.0 },
                    ),
                    font,
                    font_data,
                    scale: PxScale { x: 22.5, y: 20.0 },
                    character: 'Ỵ',
                };
                return font_data_struct;
            }
        }
    }
}
