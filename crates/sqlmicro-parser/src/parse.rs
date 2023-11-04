use nom::{
    bytes::complete::{tag_no_case, take_while1},
    character::complete::char,
    character::complete::multispace0,
    combinator::{all_consuming, map, peek},
    multi::separated_list1,
    sequence::{pair, tuple},
    Finish, IResult,
};
use nom_locate::LocatedSpan;

use crate::error::{format_parse_error, FormattedError, ParseError};

// Use nom_locate's LocatedSpan as a wrapper around a string input
pub type RawSpan<'a> = LocatedSpan<&'a str>;

// the result for all of our parsers, they will have our span type as input and can have any output
// this will use a default error type but we will change that latter
pub type ParseResult<'a, T> = IResult<RawSpan<'a>, T, ParseError<'a>>;

/// Implement the parse function to more easily convert a span into a sql
/// command
pub trait Parse<'a>: Sized {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self>;

    fn parse_from_raw(input: &'a str) -> ParseResult<'a, Self> {
        let i = LocatedSpan::new(input);
        Self::parse(i)
    }

    fn parse_format_error(input: &'a str) -> Result<Self, FormattedError<'a>> {
        let span_input = LocatedSpan::new(input);
        match all_consuming(Self::parse)(span_input).finish() {
            Ok((_, query)) => Ok(query),
            Err(e) => Err(format_parse_error(input, e)),
        }
    }
}

/// Parse a unquoted sql identifier
pub(crate) fn identifier(input: RawSpan) -> ParseResult<String> {
    map(take_while1(|c: char| c.is_alphanumeric()), |s: RawSpan| {
        s.fragment().to_string()
    })(input)
}

pub(crate) fn comma_sep<'a, O, E, F>(
    f: F,
) -> impl FnMut(RawSpan<'a>) -> IResult<RawSpan<'a>, Vec<O>, E>
where
    F: nom::Parser<RawSpan<'a>, O, E>,
    E: nom::error::ParseError<RawSpan<'a>>,
{
    separated_list1(tuple((multispace0, char(','), multispace0)), f)
}

/// Check if the input has the passed in tag
/// if so run the parser supplied
/// (with the peeked tag still expected)
/// and cut on error
/// This is useful for alts so we stop on errors
pub(crate) fn peek_then_cut<'a, T, O, E, F>(
    peek_tag: T,
    f: F,
) -> impl FnMut(RawSpan<'a>) -> IResult<RawSpan<'a>, O, E>
where
    T: nom::InputLength + Clone,
    F: nom::Parser<RawSpan<'a>, O, E>,
    E: nom::error::ParseError<RawSpan<'a>> + nom_supreme::tag::TagError<RawSpan<'a>, T>,
    LocatedSpan<&'a str>: nom::Compare<T>,
{
    map(pair(peek(tag_no_case(peek_tag)), f), |(_, f_res)| f_res)
}
