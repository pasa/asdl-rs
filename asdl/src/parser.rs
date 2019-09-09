use nom::IResult;
use std::str;

use super::ast::*;

use std::ops::{RangeFrom, RangeTo};

use nom::{Slice, InputIter, Offset, AsChar, InputTakeAtPosition};
use nom::error::ParseError;
use nom::character::is_alphanumeric;
use nom::character::complete::{char, one_of};
use nom::sequence::{pair, tuple};
use nom::bytes::complete::{take_while_m_n, take_while, is_a};
use nom::branch::alt;
use nom::multi::{separated_list, many0, separated_nonempty_list};
use nom::combinator::{map, opt, recognize};
use nom::character::complete::{multispace0, multispace1, line_ending, not_line_ending, space0};

pub(crate) fn parse(i: &str) -> IResult<&str, Root> {
    map(
        tuple((multispace0, opt(pair(comments, multispace1)), many0(ty))),
        |(_, comments, types)| Root::new(types, comments.map(|(c, _)| c).unwrap_or_default()),
    )(i)
}

fn comment_line(i: &str) -> IResult<&str, &str> {
    let (i, (_, _, _, comment)) = tuple((space0, is_a("//"), space0, not_line_ending))(i)?;
    Ok((i, comment))
}

fn comments(i: &str) -> IResult<&str, Vec<&str>> {
    separated_list(line_ending, comment_line)(i)
}

fn ty(i: &str) -> IResult<&str, Type> {
    alt((map(prod_type, |t| t.into()), map(sum_type, |t| t.into())))(i)
}

fn prod_type(i: &str) -> IResult<&str, ProdType> {
    map(
        tuple((comments, multispace0, type_id, char_ms0('='), fields)),
        |(comments, _, type_id, _, fields)| ProdType::new(type_id, fields, comments),
    )(i)
}

fn sum_type(i: &str) -> IResult<&str, SumType> {
    map(
        tuple((
            multispace0,
            comments,
            opt(line_ending),
            space0,
            type_id,
            char_ms0('='),
            constructors,
            opt(attrs),
        )),
        |(_, comments, _, _, type_id, _, constructors, attrs)| {
            SumType::new(type_id, constructors, attrs, comments)
        },
    )(i)
}

fn attrs(i: &str) -> IResult<&str, Attrs> {
    map(pair(is_a("attributes"), fields), |(_, flds)| Attrs::new(flds))(i)
}

fn constructors(i: &str) -> IResult<&str, Vec<Constr>> {
    separated_nonempty_list(char_ms0('|'), constructor)(i)
}

fn constructor(i: &str) -> IResult<&str, Constr> {
    map(
        tuple((multispace0, comments, multispace0, con_id, multispace0, opt(fields))),
        |(_, comments, _, con_id, _, fields)| {
            Constr::new(con_id, fields.unwrap_or_else(|| vec![]), comments)
        },
    )(i)
}

fn fields(i: &str) -> IResult<&str, Vec<Field>> {
    let fields = separated_list(char_ms0(','), field);
    let (i, (_, fields, _)) = tuple((char_ms0('('), fields, char_ms0(')')))(i)?;
    Ok((i, fields))
}

fn field(i: &str) -> IResult<&str, Field> {
    let (i, (type_id, rep, name)) = tuple((type_id, field_arity, opt(pair(multispace1, id))))(i)?;
    let name = name.map(|n| n.1);
    if let Some(rep) = rep {
        match rep {
            '*' => Ok((i, Repeated::new(type_id, name).into())),
            '?' => Ok((i, Optional::new(type_id, name).into())),
            _ => unreachable!(),
        }
    } else {
        Ok((i, Required::new(type_id, name).into()))
    }
}

fn type_id(i: &str) -> IResult<&str, TypeId> {
    map(
        recognize(pair(
            take_while_m_n(1, 1, is_lowercase),
            take_while(is_alphanumeric_or_underscore),
        )),
        TypeId,
    )(i)
}

fn con_id(i: &str) -> IResult<&str, ConstrId> {
    map(
        recognize(pair(
            take_while_m_n(1, 1, is_uppercase),
            take_while(is_alphanumeric_or_underscore),
        )),
        ConstrId,
    )(i)
}

fn id(i: &str) -> IResult<&str, Id> {
    map(
        recognize(pair(take_while_m_n(1, 1, is_alpha), take_while(is_alphanumeric_or_underscore))),
        Id,
    )(i)
}

fn field_arity(i: &str) -> IResult<&str, Option<char>> {
    opt(one_of("*?"))(i)
}

fn is_alpha(a: char) -> bool {
    is_alphanumeric(a as u8)
}

fn is_uppercase(a: char) -> bool {
    a.is_uppercase()
}

fn is_lowercase(a: char) -> bool {
    a.is_lowercase()
}

pub fn char_ms0<I, E: ParseError<I>>(c: char) -> impl Fn(I) -> IResult<I, I, E>
where
    I: Slice<RangeFrom<usize>>
        + Slice<RangeTo<usize>>
        + Offset
        + InputIter
        + InputTakeAtPosition
        + Clone,
    <I as InputIter>::Item: AsChar + Clone,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    recognize(tuple((multispace0, char(c), multispace0)))
}

fn is_alphanumeric_or_underscore(a: char) -> bool {
    is_alphanumeric(a as u8) || a == '_'
}

#[cfg(test)]
mod tests {

    use super::*;
    use nom;
    use nom::error::ErrorKind;
    use nom::Err;
    use insta::assert_debug_snapshot_matches;

    #[test]
    fn parse_type_id() {
        assert_eq!(type_id("aBcd1_Efg"), Ok(("", TypeId("aBcd1_Efg"))));
        assert_eq!(type_id("aBcd1,Efg"), Ok((",Efg", TypeId("aBcd1"))));
        assert_eq!(type_id("ABcd1Efg"), Err(Err::Error(("ABcd1Efg", ErrorKind::TakeWhileMN))));
    }

    #[test]
    fn parse_con_id() {
        assert_eq!(con_id("ABcd1_Efg"), Ok(("", ConstrId("ABcd1_Efg"))));
        assert_eq!(con_id("ABcd1,Efg"), Ok((",Efg", ConstrId("ABcd1"))));
        assert_eq!(con_id("aBcd1Efg"), Err(Err::Error(("aBcd1Efg", ErrorKind::TakeWhileMN))));
    }

    #[test]
    fn parse_id() {
        assert_eq!(id("ABcd1_Efg"), Ok(("", Id("ABcd1_Efg"))));
        assert_eq!(id("ABcd1,Efg"), Ok((",Efg", Id("ABcd1"))));
        assert_eq!(id("aBcd1_Efg"), Ok(("", Id("aBcd1_Efg"))));
        assert_eq!(id("aBcd1,Efg"), Ok((",Efg", Id("aBcd1"))));
        assert_eq!(id("_aBcd1Efg"), Err(Err::Error(("_aBcd1Efg", ErrorKind::TakeWhileMN))));
    }

    #[test]
    fn parse_field_arity() {
        assert_eq!(field_arity("? abcd"), Ok((" abcd", Some('?'))));
        assert_eq!(field_arity("* abcd"), Ok((" abcd", Some('*'))));
        assert_eq!(field_arity(" abcd"), Ok((" abcd", None)));
    }

    #[test]
    fn parse_field() {
        assert_eq!(field("type,"), Ok((",", Required::new(TypeId("type"), None).into())));
        assert_eq!(field("type?,"), Ok((",", Optional::new(TypeId("type"), None).into())));
        assert_eq!(field("type*,"), Ok((",", Repeated::new(TypeId("type"), None).into())));

        assert_eq!(
            field("type  name,"),
            Ok((",", Required::new(TypeId("type"), Some(Id("name"))).into()))
        );
        assert_eq!(
            field("type?  name,"),
            Ok((",", Optional::new(TypeId("type"), Some(Id("name"))).into()))
        );
        assert_eq!(
            field("type*  name,"),
            Ok((",", Repeated::new(TypeId("type"), Some(Id("name"))).into()))
        );
    }

    #[test]
    fn parse_fields() {
        assert_eq!(
            fields(" ( type1, type2? name  ) "),
            Ok((
                "",
                vec![
                    Required::new(TypeId("type1"), None).into(),
                    Optional::new(TypeId("type2"), Some(Id("name"))).into()
                ]
            ))
        );
    }

    #[test]
    fn parse_constructor() {
        assert_eq!(
            constructor("ConstrId( type1, type2? name  ) "),
            Ok((
                "",
                Constr::new(
                    ConstrId("ConstrId"),
                    vec![
                        Required::new(TypeId("type1"), None).into(),
                        Optional::new(TypeId("type2"), Some(Id("name"))).into()
                    ],
                    vec![]
                )
            ))
        );

        assert_eq!(
            constructor("ConstrId"),
            Ok(("", Constr::new(ConstrId("ConstrId"), vec![], vec![])))
        );
    }

    #[test]
    fn parse_constructors() {
        assert_eq!(
            constructors("ConstrId1( type1, type2? name  ) | ConstrId2"),
            Ok((
                "",
                vec![
                    Constr::new(
                        ConstrId("ConstrId1"),
                        vec![
                            Required::new(TypeId("type1"), None).into(),
                            Optional::new(TypeId("type2"), Some(Id("name"))).into()
                        ],
                        vec![]
                    ),
                    Constr::new(ConstrId("ConstrId2"), vec![], vec![])
                ]
            ))
        );
    }

    #[test]
    fn parse_comment_line() {
        let asdl = r#"  //comment line1"#;
        assert_eq!(comment_line(asdl), Ok(("", "comment line1")));
    }

    #[test]
    fn parse_comments() {
        let asdl = r#"  // comment line1
                        // comment line2"#;
        assert_eq!(comments(asdl), Ok(("", vec!["comment line1", "comment line2"])));
    }

    #[test]
    fn parse_constructors_with_comments() {
        let asdl = r#"  // ConstrId1 comment
                        ConstrId1( type1, type2? name  ) |
                        // ConstrId2 comment line1
                        // ConstrId2 comment line2
                        ConstrId2"#;
        assert_debug_snapshot_matches!(constructors(asdl));
    }

    #[test]
    fn parse_sum_type() {
        let asdl = r#"
                    // SumType comment line 1
                    // SumType comment line 2
                    sumType = 
                        // ConstrId1 comment
                        ConstrId1( type1, type2? name  ) |
                        // ConstrId2 comment line1
                        // ConstrId2 comment line2
                        ConstrId2"#;
        assert_debug_snapshot_matches!(sum_type(asdl));
    }

    #[test]
    fn parse_prod_type() {
        let asdl = r#"  // prodType comment line 1
                        // prodType comment line 2
                        prodType = ( type1, type2? name  )"#;
        assert_debug_snapshot_matches!(prod_type(asdl));
    }

    #[test]
    fn parse_empty_asdl() {
        let asdl = "";
        assert_debug_snapshot_matches!(parse(asdl));
    }
}
