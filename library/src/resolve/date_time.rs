use {
    chrono::*,
    compris::resolve::*,
    dateparser::*,
    kutil::std::{
        error::*,
        string::{ParseError, ParseStr},
    },
};

/// [DateTime] that implements [Resolve].
pub type ResolveDateTime = ResolveParseStr<DateTime<Utc>, ParseDateTime>;

//
// ParseDateTime
//

/// [ParseStr] for [DateTime].
///
/// See [dateparser](https://docs.rs/dateparser/latest/dateparser/#accepted-date-formats).
#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParseDateTime {}

impl ParseStr<DateTime<Utc>> for ParseDateTime {
    fn parse(representation: &str) -> Result<DateTime<Utc>, ParseError> {
        Ok(parse(representation).into_string()?)
    }
}
