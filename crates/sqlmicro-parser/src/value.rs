use std::str::FromStr;

use bigdecimal::BigDecimal;
use derive_more::Display;
use nom::{
    branch::alt,
    bytes::complete::{take_until, take_while},
    character::complete::multispace0,
    error::context,
    sequence::{preceded, terminated, tuple},
    Parser,
};
use nom_supreme::tag::complete::tag;
use serde::{Deserialize, Serialize};

use crate::parse::{peek_then_cut, Parse, ParseResult, RawSpan};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Display)]
pub enum Value {
    Number(BigDecimal),
    String(String),
}

fn parse_string_value(input: RawSpan<'_>) -> ParseResult<'_, Value> {
    let (rem, (_, value, _)) = context(
        "String Literal",
        tuple((
            tag("'"),
            take_until("'").map(|s: RawSpan| Value::String(s.fragment().to_string())),
            tag("'"),
        )),
    )(input)?;

    Ok((rem, value))
}

fn parse_number_value(input: RawSpan<'_>) -> ParseResult<'_, Value> {
    let (rem, digits) = context("Number Literal", take_while(|c: char| c.is_numeric()))(input)?;

    let digits = digits.fragment();

    Ok((rem, Value::Number(BigDecimal::from_str(digits).unwrap())))
}

impl<'a> Parse<'a> for Value {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        context(
            "Value",
            preceded(
                multispace0,
                terminated(
                    alt((peek_then_cut("'", parse_string_value), parse_number_value)),
                    multispace0,
                ),
            ),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let expected = Value::String("123abc new".into());
        let expected_rem = "fart '123'";

        let (rem, val) = Value::parse_from_raw("'123abc new' fart '123'").unwrap();

        assert_eq!(val, expected);
        assert_eq!(expected_rem, rem.fragment().to_string());
    }

    #[test]
    fn test_number() {
        let num = BigDecimal::from_str("123456").unwrap();
        let expected = Value::Number(num);

        assert_eq!(Value::parse_from_raw("123456").unwrap().1, expected);
    }
}
