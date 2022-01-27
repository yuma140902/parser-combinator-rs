#[derive(Debug, PartialEq)]
pub enum DenialReason {
    Deny,
}

#[derive(Debug, PartialEq)]
pub struct ParsingResult<'a, T> {
    pub first: T,
    pub rest: &'a str,
}
