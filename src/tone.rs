#[derive(Clone, Copy)]
pub enum Tone {
    None,
    Sac,
    Huyen,
    Hoi,
    Nga,
    Nang,
}

//None, Sac, Huyen, Hoi, Nga, Nang
// Matrix for vowels with tones
const VOWELS: [&str; 6] = [
    "aăâeêioôơuưyAĂÂEÊIOÔƠUƯY",
    "áắấeếíoốớúứýÁẮẤÉẾÍÓỐỚÚỨÝ",
    "àằầèềìòồờùừỳÀẰẦÈỀÌÒỒỜÙỪỲ",
    "ảẳẩẻểỉỏổởủửỷẢẲẨẺỂỈỎỔỞỦỬỶ",
    "ãẵẫẽễĩõỗỡũữỹÃẴẪẼỄĨÕỖỠŨỮỸ",
    "ạặậẹệịọộợụựỵẠẶẬẸỆỊỌỘỢỤỰỴ",
];

impl Tone {
    fn idx(self) -> usize {
        match self {
            Self::None => 0,
            Self::Sac => 1,
            Self::Huyen => 2,
            Self::Hoi => 3,
            Self::Nga => 4,
            Self::Nang => 5,
        }
    }
}

pub fn find_tone(c: char) -> Tone {
    for (tone_index, row) in VOWELS.iter().enumerate() {
        if row.contains(c) {
            return match tone_index {
                0 => Tone::None,
                1 => Tone::Sac,
                2 => Tone::Huyen,
                3 => Tone::Hoi,
                4 => Tone::Nga,
                5 => Tone::Nang,
                _ => Tone::None,
            };
        }
    }
    Tone::None
}

pub fn add_tone(c: char, tone: Tone) -> char {
    for (_tone_index, row) in VOWELS.iter().enumerate() {
        if let Some(col_index) = row.chars().position(|x| x == c) {
            return VOWELS[tone.idx()].chars().nth(col_index).unwrap();
        }
    }
    c
}
