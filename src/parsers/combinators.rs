use crate::common::ParsingResult;

use super::Parser;

/// パーサを受け取り、先頭の空白を無視するパーサを返す
///
/// 空白とは、UnicodeのWhite_Spaceのこと
pub fn lexeme<'a, T>(parser: impl Parser<'a, T>) -> impl Parser<'a, T> {
    move |s: &'a str| parser(s.trim_start())
}

/// パース結果に関数を適用するパーサ
pub fn map<'a, T, S>(parser: impl Parser<'a, T>, f: impl Fn(T) -> S) -> impl Parser<'a, S> {
    move |s: &'a str| {
        parser(s).map(|ParsingResult { first, rest }| ParsingResult {
            first: f(first),
            rest,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::ParsingResult, parsers::digits};

    use super::*;

    #[test]
    fn lexeme_true() {
        let parser = lexeme(digits);
        assert_eq!(
            parser("     123abc"),
            Ok(ParsingResult {
                first: 123,
                rest: "abc"
            })
        );
        assert_eq!(
            parser("456def"),
            Ok(ParsingResult {
                first: 456,
                rest: "def"
            })
        );
        assert_eq!(
            parser("    789    ghi"),
            Ok(ParsingResult {
                first: 789,
                rest: "    ghi"
            })
        );
        assert_eq!(
            parser(" 123 "),
            Ok(ParsingResult {
                first: 123,
                rest: " "
            })
        );
    }

    #[test]
    fn lexeme_false() {
        let parser = lexeme(digits);
        assert!(parser("A 123 BC").is_err());
        assert!(parser("    ").is_err());
        assert!(parser("").is_err());
    }

    #[test]
    fn lexeme_whitespace_true() {
        let parser = lexeme(digits);
        let expected = Ok(ParsingResult {
            first: 123,
            rest: "",
        });

        // 注意: すべての空白類を網羅しているわけではない
        assert_eq!(parser(" 123"), expected);
        assert_eq!(parser("\t123"), expected);
        assert_eq!(parser(" \t \t \t 123"), expected);
        assert_eq!(parser("\n123"), expected);
        assert_eq!(parser("\r123"), expected);
        assert_eq!(parser("\x0B123"), expected); // 垂直タブ
        assert_eq!(parser("\u{3000}123"), expected); // 全角スペース
    }

    #[test]
    fn lexeme_whitespace_false() {
        let parser = lexeme(digits);

        assert!(parser("\0123").is_err());
        assert!(parser(".123").is_err());
    }

    #[test]
    fn map_true() {
        let parser = map(digits, |x| x + 1);

        assert_eq!(
            parser("123"),
            Ok(ParsingResult {
                first: 124,
                rest: ""
            })
        );
    }
}
