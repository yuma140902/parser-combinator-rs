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

pub fn digits<'a>(s: &'a str) -> Result<ParsingResult<'a, i32>, DenialReason> {
    // 数字ではない最初の文字のインデックス
    let boundary = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());

    if let Ok(value) = s[..boundary].parse() {
        Ok(ParsingResult {
            first: value,
            rest: &s[boundary..],
        })
    } else {
        Err(DenialReason::Deny)
    }
}

pub fn character<'a>(c: char) -> impl Fn(&'a str) -> Result<ParsingResult<'a, ()>, DenialReason> {
    move |s| {
        let mut chars = s.chars();
        if chars.next() == Some(c) {
            Ok(ParsingResult {
                first: (),
                rest: chars.as_str(),
            })
        } else {
            Err(DenialReason::Deny)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common::ParsingResult,
        parsers::{character, digit, digits},
    };

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

    #[test]
    fn character_a() {
        let parser = character('A');
        assert_eq!(
            parser("Abcd"),
            Ok(ParsingResult {
                first: (),
                rest: "bcd"
            })
        );
        assert_eq!(
            parser("A"),
            Ok(ParsingResult {
                first: (),
                rest: ""
            })
        );
        assert!(parser("Banana").is_err());
        assert!(parser("abcd").is_err());
        assert!(parser("        A").is_err());
        assert!(parser("").is_err());
    }

    #[test]
    fn character_kanji() {
        let parser = character('令');
        assert_eq!(
            parser("令和"),
            Ok(ParsingResult {
                first: (),
                rest: "和"
            })
        );
        assert_eq!(
            parser("令"),
            Ok(ParsingResult {
                first: (),
                rest: ""
            })
        );
        assert!(parser("平成").is_err());
        assert!(parser("         令").is_err());
        assert!(parser("").is_err());
    }

    #[test]
    fn character_emoji() {
        let parser = character('\u{1f363}');
        assert_eq!(
            parser("\u{1f363}寿司"),
            Ok(ParsingResult {
                first: (),
                rest: "寿司"
            })
        );
        assert_eq!(
            parser("\u{1f363}"),
            Ok(ParsingResult {
                first: (),
                rest: ""
            })
        );
        assert!(parser("\u{1f364}エビフライ").is_err());
    }

    #[test]
    fn digits_true() {
        assert_eq!(
            digits("123"),
            Ok(ParsingResult {
                first: 123,
                rest: ""
            })
        );
        assert_eq!(
            digits("456abc"),
            Ok(ParsingResult {
                first: 456,
                rest: "abc"
            })
        );
        assert_eq!(digits("7"), Ok(ParsingResult { first: 7, rest: "" }));
        assert_eq!(
            digits("12ab3"),
            Ok(ParsingResult {
                first: 12,
                rest: "ab3"
            })
        );
        assert_eq!(
            digits("12.3"),
            Ok(ParsingResult {
                first: 12,
                rest: ".3"
            })
        );
        assert_eq!(
            digits("0x12ff"),
            Ok(ParsingResult {
                first: 0,
                rest: "x12ff"
            })
        );
        assert_eq!(
            digits("010"),
            Ok(ParsingResult {
                first: 10,
                rest: ""
            })
        );
    }

    #[test]
    fn digits_false() {
        assert!(digits("-123").is_err());
        assert!(digits("EF01").is_err());
        assert!(digits("IV").is_err());
        assert!(digits("五").is_err());
        assert!(digits("      123").is_err());
        assert!(digits("").is_err());
        assert!(digits("abc").is_err());
    }
}
