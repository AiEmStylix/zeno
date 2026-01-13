#[derive(Clone, Copy, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_tone_basic() {
        assert_eq!(find_tone('a'), Tone::None);
        assert_eq!(find_tone('á'), Tone::Sac);
        assert_eq!(find_tone('à'), Tone::Huyen);
        assert_eq!(find_tone('ả'), Tone::Hoi);
        assert_eq!(find_tone('ã'), Tone::Nga);
        assert_eq!(find_tone('ạ'), Tone::Nang);
    }

    #[test]
    fn add_tone_basic() {
        assert_eq!(add_tone('a', Tone::None), 'a');
        assert_eq!(add_tone('a', Tone::Sac), 'á');
        assert_eq!(add_tone('a', Tone::Huyen), 'à');
        assert_eq!(add_tone('a', Tone::Hoi), 'ả');
        assert_eq!(add_tone('a', Tone::Nga), 'ã');
        assert_eq!(add_tone('a', Tone::Nang), 'ạ');
    }

    #[test]
    fn non_vowel_characters_are_unchanged() {
        let chars = ['b', 'z', '1', ' ', '\n', '😀', '你'];

        for c in chars {
            assert_eq!(find_tone(c), Tone::None);
            assert_eq!(add_tone(c, Tone::Sac), c);
            assert_eq!(add_tone(c, Tone::Huyen), c);
        }
    }
}
