use crate::common::{DenialReason, ParsingResult};

pub fn digit<'a>(s: &'a str) -> Result<ParsingResult<'a, i32>, DenialReason> {
    // 2文字目の先頭バイトのインデックス
    let boundary = s.char_indices().nth(1).map(|(i, _)| i).unwrap_or(s.len());

    if let Ok(digit) = s[..boundary].parse() {
        Ok(ParsingResult {
            first: digit,
            rest: &s[boundary..],
        })
    } else {
        Err(DenialReason::Deny)
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::ParsingResult, parsers::digit};

    #[test]
    fn digit_true() {
        assert_eq!(
            digit("123"),
            Ok(ParsingResult {
                first: 1,
                rest: "23"
            })
        );
        assert_eq!(
            digit("2Abcd"),
            Ok(ParsingResult {
                first: 2,
                rest: "Abcd"
            })
        );
        assert_eq!(digit("9"), Ok(ParsingResult { first: 9, rest: "" }));
        assert_eq!(
            digit("85"),
            Ok(ParsingResult {
                first: 8,
                rest: "5"
            })
        );
        assert_eq!(
            digit("5漢字"),
            Ok(ParsingResult {
                first: 5,
                rest: "漢字"
            })
        );
        assert_eq!(
            digit("4\u{1f363}"),
            Ok(ParsingResult {
                first: 4,
                rest: "\u{1f363}"
            })
        );
    }

    #[test]
    fn digit_false() {
        assert!(digit("abcd").is_err());
        assert!(digit("A").is_err());
        assert!(digit("").is_err());
        assert!(digit("        123").is_err());
        assert!(digit("漢字").is_err());
        assert!(digit("\u{1f363}").is_err());
    }
}
