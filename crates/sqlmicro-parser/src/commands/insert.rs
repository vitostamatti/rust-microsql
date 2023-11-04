use nom::{
    bytes::complete::tag_no_case,
    character::complete::multispace1,
    error::context,
    sequence::{preceded, tuple},
};
use nom_supreme::ParserExt;
use serde::{Deserialize, Serialize};

use crate::{
    parse::{comma_sep, identifier, Parse},
    value::Value,
};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InsertStatement {
    pub table: String,
    pub values: Vec<Value>,
}

impl<'a> Parse<'a> for InsertStatement {
    fn parse(input: crate::parse::RawSpan<'a>) -> crate::parse::ParseResult<'a, Self> {
        let (rem, (_, _, table, _, values)) = context(
            "Insert Statement",
            tuple((
                tag_no_case("insert"),
                preceded(multispace1, tag_no_case("into")),
                preceded(multispace1, identifier.context("Table Name")),
                preceded(multispace1, tag_no_case("values")),
                preceded(multispace1, comma_sep(Value::parse).context("Values")),
            )),
        )(input)?;

        Ok((rem, InsertStatement { table, values }))
    }
}
