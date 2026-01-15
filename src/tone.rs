use std::{collections::HashMap, sync::LazyLock};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tone {
    None,
    Sac,
    Huyen,
    Hoi,
    Nga,
    Nang,
}

impl Tone {
    #[inline]
    pub const fn idx(self) -> usize {
        self as usize
    }

    #[inline]
    pub const fn from_idx(idx: usize) -> Self {
        match idx {
            0 => Tone::None,
            1 => Tone::Sac,
            2 => Tone::Huyen,
            3 => Tone::Hoi,
            4 => Tone::Nga,
            5 => Tone::Nang,
            _ => unreachable!(),
        }
    }
}

pub type ToneRow = [char; 6];
//None, Sac, Huyen, Hoi, Nga, Nang
// Matrix for vowels with tones
const VOWELS: &[[char; 6]] = &[
    ['a', 'á', 'à', 'ả', 'ã', 'ạ'],
    ['ă', 'ắ', 'ằ', 'ẳ', 'ẵ', 'ặ'],
    ['â', 'ấ', 'ầ', 'ẩ', 'ẫ', 'ậ'],
    ['e', 'é', 'è', 'ẻ', 'ẽ', 'ẹ'],
    ['ê', 'ế', 'ề', 'ể', 'ễ', 'ệ'],
    ['i', 'í', 'ì', 'ỉ', 'ĩ', 'ị'],
    ['o', 'ó', 'ò', 'ỏ', 'õ', 'ọ'],
    ['ô', 'ố', 'ồ', 'ổ', 'ỗ', 'ộ'],
    ['ơ', 'ớ', 'ờ', 'ở', 'ỡ', 'ợ'],
    ['u', 'ú', 'ù', 'ủ', 'ũ', 'ụ'],
    ['ư', 'ứ', 'ừ', 'ử', 'ữ', 'ự'],
    ['y', 'ý', 'ỳ', 'ỷ', 'ỹ', 'ỵ'],
    // Uppercase
    ['A', 'Á', 'À', 'Ả', 'Ã', 'Ạ'],
    ['Ă', 'Ắ', 'Ằ', 'Ẳ', 'Ẵ', 'Ặ'],
    ['Â', 'Ấ', 'Ầ', 'Ẩ', 'Ẫ', 'Ậ'],
    ['E', 'É', 'È', 'Ẻ', 'Ẽ', 'Ẹ'],
    ['Ê', 'Ế', 'Ề', 'Ể', 'Ễ', 'Ệ'],
    ['I', 'Í', 'Ì', 'Ỉ', 'Ĩ', 'Ị'],
    ['O', 'Ó', 'Ò', 'Ỏ', 'Õ', 'Ọ'],
    ['Ô', 'Ố', 'Ồ', 'Ổ', 'Ỗ', 'Ộ'],
    ['Ơ', 'Ớ', 'Ờ', 'Ở', 'Ỡ', 'Ợ'],
    ['U', 'Ú', 'Ù', 'Ủ', 'Ũ', 'Ụ'],
    ['Ư', 'Ứ', 'Ừ', 'Ử', 'Ữ', 'Ự'],
    ['Y', 'Ý', 'Ỳ', 'Ỷ', 'Ỹ', 'Ỵ'],
];

static VOWEL_MAP: LazyLock<HashMap<char, ToneRow>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    for row in VOWELS {
        for &ch in row {
            map.insert(ch, *row);
        }
    }

    map
});

pub fn add_tone(c: char, tone: Tone) -> Option<char> {
    VOWEL_MAP.get(&c).map(|row| row[tone.idx()])
}

pub fn find_tone(c: char) -> Option<Tone> {
    let row = VOWEL_MAP.get(&c)?;
    row.iter().position(|&x| x == c).map(Tone::from_idx)
}

pub fn strip_tone(c: char) -> Option<char> {
    VOWEL_MAP.get(&c).map(|row| row[0])
}

pub fn replace_tone(c: char, tone: Tone) -> Option<char> {
    let row = VOWEL_MAP.get(&c)?;
    Some(row[tone.idx()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_tone_basic() {
        assert_eq!(find_tone('a'), Some(Tone::None));
        assert_eq!(find_tone('á'), Some(Tone::Sac));
        assert_eq!(find_tone('à'), Some(Tone::Huyen));
        assert_eq!(find_tone('ả'), Some(Tone::Hoi));
        assert_eq!(find_tone('ã'), Some(Tone::Nga));
        assert_eq!(find_tone('ạ'), Some(Tone::Nang));
    }

    #[test]
    fn add_tone_basic() {
        assert_eq!(add_tone('a', Tone::None), Some('a'));
        assert_eq!(add_tone('a', Tone::Sac), Some('á'));
        assert_eq!(add_tone('a', Tone::Huyen), Some('à'));
        assert_eq!(add_tone('a', Tone::Hoi), Some('ả'));
        assert_eq!(add_tone('a', Tone::Nga), Some('ã'));
        assert_eq!(add_tone('a', Tone::Nang), Some('ạ'));
    }

    #[test]
    fn strip_tone_basic() {
        assert_eq!(strip_tone('a'), Some('a'));
        assert_eq!(strip_tone('á'), Some('a'));
        assert_eq!(strip_tone('ấ'), Some('â'));
        assert_eq!(strip_tone('ằ'), Some('ă'));
    }

    #[test]
    fn uppercase_vowels_work() {
        assert_eq!(find_tone('A'), Some(Tone::None));
        assert_eq!(find_tone('Á'), Some(Tone::Sac));
        assert_eq!(add_tone('A', Tone::Sac), Some('Á'));
        assert_eq!(add_tone('Ê', Tone::Huyen), Some('Ề'));
    }

    #[test]
    fn non_vowel_characters_are_unchanged() {
        let chars = ['b', 'z', '1', ' ', '\n', '😀', '你'];

        for c in chars {
            assert_eq!(find_tone(c), None);
            assert_eq!(add_tone(c, Tone::Sac), None);
            assert_eq!(add_tone(c, Tone::Huyen), None);
        }
    }
}
