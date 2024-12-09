use std::sync::LazyLock;

/// return value includes `0-9`, `A-Z`, `a-z`, `-`, `_`
pub(super) static VALID_ASCII_CHARS: LazyLock<Vec<char>> =
    LazyLock::new(valid_ascii_chars);

/// return value includes `0-9`, `A-Z`, `a-z`, `-`, `_`
fn valid_ascii_chars() -> Vec<char> {
    let valid_range1 = 0x30..=0x39; // 0-9
    let valid_range2 = 0x41..=0x5a; // A-Z
    let valid_range3 = 0x61..=0x7a; // a-z
    let valid_range4 = 0x2d..=0x2d; // -
    let valid_range5 = 0x5f..=0x5f; // _
    vec![
        valid_range1,
        valid_range2,
        valid_range3,
        valid_range4,
        valid_range5,
    ]
    .into_iter()
    .flat_map(|r| r.map(|m| m.into()))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `0-9`, `A-Z`, `a-z`, `-`, `_` を別の方法で定義して一部与える
    #[test]
    fn test_valid_ascii_gives_valid() {
        let valid = [
            'a', 'b', 'c', 'x', 'y', 'z', '0', '1', '2', '7', '8', '9', 'A', 'B', 'C',
            'X', 'Y', 'Z', '_', '-',
        ];
        let valid_range = valid_ascii_chars();
        assert!(valid.into_iter().all(|c| valid_range.contains(&c)));
    }

    /// 無効な範囲外の文字を与える
    #[test]
    fn test_valid_ascii_gives_invalid() {
        let invalid = ['!', '"', '#', '$', '&', '\'', 'あ', 'ア', '栞'];
        let valid_range = valid_ascii_chars();
        assert!(!invalid.into_iter().all(|c| valid_range.contains(&c)));
    }
}
