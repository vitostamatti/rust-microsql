use nom::{
    bytes::complete::tag_no_case, character::complete::multispace1, error::context, sequence::tuple,
};
use nom_supreme::ParserExt;
use serde::{Deserialize, Serialize};

use crate::parse::{comma_sep, identifier, Parse, ParseResult};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SelectStatement {
    pub table: String,
    pub fields: Vec<String>,
}

impl<'a> Parse<'a> for SelectStatement {
    fn parse(input: crate::parse::RawSpan<'a>) -> ParseResult<'a, Self> {
        let (rem, (_, _, fields, _, _, _, table)) = context(
            "Select Statement",
            tuple((
                tag_no_case("select"),
                multispace1,
                comma_sep(identifier).context("Select Columns"),
                multispace1,
                tag_no_case("from"),
                multispace1,
                identifier.context("From Table"),
            )),
        )(input)?;

        Ok((rem, SelectStatement { fields, table }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select() {
        let expected = SelectStatement {
            table: "t1".into(),
            fields: vec!["foo".into(), "bar".into()],
        };

        let query = SelectStatement::parse_from_raw("select foo, bar from t1;")
            .unwrap()
            .1;

        assert_eq!(expected, query);
    }
}
