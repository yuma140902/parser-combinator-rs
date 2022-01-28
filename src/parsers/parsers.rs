use crate::common::{DenialReason, ParsingResult};

// トレイト`Parser<'a, T>`を実装する型はトレイト`Fn(&'a str) -> Result<ParsingResult<'a, T>, DenialReason>`を必ず実装する
// すなわち`Parser...`ならば`Fn...`
pub trait Parser<'a, T>: Fn(&'a str) -> Result<ParsingResult<'a, T>, DenialReason> {}
// トレイト`Fn(&'a str) -> Result<ParsingResult<'a, T>, DenialReason>`を実装するすべての型Fに対してトレイト`Parser<'a, T>`を実装する
// すなわち`Fn...`ならば`Parser...`
impl<'a, T, F> Parser<'a, T> for F where F: Fn(&'a str) -> Result<ParsingResult<'a, T>, DenialReason>
{}
// 上記2行の定義によりトレイト`Parser<'a, T>`とトレイト`Fn(&'a str) -> Result<ParsingResult<'a, T>, DenialReason>`は等価になる

pub fn single_digit<'a>(s: &'a str) -> Result<ParsingResult<'a, i32>, DenialReason> {
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

pub fn character<'a>(c: char) -> impl Parser<'a, ()> {
    move |s: &'a str| {
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

pub fn string<'a>(criteria: &'a str) -> impl Parser<'a, ()> {
    move |s: &'a str| {
        if let Some(rest) = s.strip_prefix(criteria) {
            Ok(ParsingResult { first: (), rest })
        } else {
            Err(DenialReason::Deny)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common::ParsingResult,
        parsers::{character, digits, single_digit},
    };

    use super::string;

    #[test]
    fn digit_true() {
        assert_eq!(
            single_digit("123"),
            Ok(ParsingResult {
                first: 1,
                rest: "23"
            })
        );
        assert_eq!(
            single_digit("2Abcd"),
            Ok(ParsingResult {
                first: 2,
                rest: "Abcd"
            })
        );
        assert_eq!(single_digit("9"), Ok(ParsingResult { first: 9, rest: "" }));
        assert_eq!(single_digit("0"), Ok(ParsingResult { first: 0, rest: "" }));
        assert_eq!(
            single_digit("85"),
            Ok(ParsingResult {
                first: 8,
                rest: "5"
            })
        );
        assert_eq!(
            single_digit("5漢字"),
            Ok(ParsingResult {
                first: 5,
                rest: "漢字"
            })
        );
        assert_eq!(
            single_digit("4\u{1f363}"),
            Ok(ParsingResult {
                first: 4,
                rest: "\u{1f363}"
            })
        );
    }

    #[test]
    fn digit_false() {
        assert!(single_digit("abcd").is_err());
        assert!(single_digit("A").is_err());
        assert!(single_digit("").is_err());
        assert!(single_digit("        123").is_err());
        assert!(single_digit("abc123").is_err());
        assert!(single_digit("漢字").is_err());
        assert!(single_digit("\u{1f363}").is_err());
        assert!(single_digit("f").is_err());
        assert!(single_digit("IV").is_err());
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
        assert!(parser("BBBBa").is_err());
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

    #[test]
    fn string_true() {
        let parser = string("abc");

        assert_eq!(
            parser("abc"),
            Ok(ParsingResult {
                first: (),
                rest: ""
            })
        );
        assert_eq!(
            parser("abcdef"),
            Ok(ParsingResult {
                first: (),
                rest: "def"
            })
        );
        assert_eq!(
            parser("abcabc"),
            Ok(ParsingResult {
                first: (),
                rest: "abc"
            })
        );
    }

    #[test]
    fn string_false() {
        let parser = string("abc");

        assert!(parser("ABC").is_err());
        assert!(parser("  abc").is_err());
        assert!(parser("foobar").is_err());
        assert!(parser("defabc").is_err());
        assert!(parser("").is_err());
    }
}
