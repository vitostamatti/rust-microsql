use nom::{
    branch::alt,
    character::complete::char,
    character::complete::multispace0,
    combinator::map,
    error::context,
    sequence::{preceded, tuple},
};
use serde::{Deserialize, Serialize};

use crate::{
    parse::{peek_then_cut, Parse, ParseResult, RawSpan},
    CreateStatement, InsertStatement, SelectStatement,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SqlQuery {
    Create(CreateStatement),
    Insert(InsertStatement),
    Select(SelectStatement),
}

impl<'a> Parse<'a> for SqlQuery {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        let (rest, (query, _, _, _)) = context(
            "Query",
            preceded(
                multispace0,
                tuple((
                    alt((
                        peek_then_cut("create", map(CreateStatement::parse, SqlQuery::Create)),
                        peek_then_cut("select", map(SelectStatement::parse, SqlQuery::Select)),
                        peek_then_cut("insert", map(InsertStatement::parse, SqlQuery::Insert)),
                    )),
                    multispace0,
                    char(';'),
                    multispace0,
                )),
            ),
        )(input)?;

        Ok((rest, query))
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse::Parse, Column, CreateStatement, SqlTypeInfo};

    use super::SqlQuery;

    #[test]
    fn test_create() {
        let expected = CreateStatement {
            table: "foo".into(),
            columns: vec![
                Column {
                    name: "col1".into(),
                    type_info: SqlTypeInfo::Int,
                },
                Column {
                    name: "col2".into(),
                    type_info: SqlTypeInfo::String,
                },
                Column {
                    name: "col3".into(),
                    type_info: SqlTypeInfo::String,
                },
            ],
        };

        let query_raw = "CREATE TABLE foo (col1 int, col2 string, col3 string);";

        let query = SqlQuery::parse_from_raw(query_raw).unwrap().1;

        assert_eq!(SqlQuery::Create(expected), query);
    }
}
