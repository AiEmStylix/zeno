#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mark {
    None,
    Mu,    // â ê ô
    Trang, // ă
    Moc,   // ơ ư
    Gach,  // đ
}

pub fn add_mark(c: char, mark: Mark) -> Option<char> {
    match (c, mark) {
        ('a', Mark::Mu) => Some('â'),
        ('a', Mark::Trang) => Some('ă'),

        ('e', Mark::Mu) => Some('ê'),

        ('o', Mark::Mu) => Some('ô'),
        ('o', Mark::Moc) => Some('ơ'),

        ('u', Mark::Moc) => Some('ư'),

        ('d', Mark::Gach) => Some('đ'),

        _ => None,
    }
}

pub fn strip_mark(c: char) -> Option<char> {
    match c {
        'â' | 'ă' => Some('a'),
        'ê' => Some('e'),
        'ô' | 'ơ' => Some('o'),
        'ư' => Some('u'),
        'đ' => Some('d'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_add_mark() {
        assert_eq!(add_mark('a', Mark::Mu), Some('â'));
        assert_eq!(add_mark('a', Mark::Trang), Some('ă'));
        assert_eq!(add_mark('e', Mark::Mu), Some('ê'));
        assert_eq!(add_mark('o', Mark::Mu), Some('ô'));
        assert_eq!(add_mark('o', Mark::Moc), Some('ơ'));
        assert_eq!(add_mark('u', Mark::Moc), Some('ư'));
        assert_eq!(add_mark('d', Mark::Gach), Some('đ'));
    }

    #[test]
    fn basic_strip_mark() {
        assert_eq!(strip_mark('â'), Some('a'));
        assert_eq!(strip_mark('ă'), Some('a'));
        assert_eq!(strip_mark('ê'), Some('e'));
        assert_eq!(strip_mark('ô'), Some('o'));
        assert_eq!(strip_mark('ơ'), Some('o'));
        assert_eq!(strip_mark('ư'), Some('u'));
        assert_eq!(strip_mark('đ'), Some('d'));
    }
}
