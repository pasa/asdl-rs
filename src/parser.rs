mod generated;

use nom::IResult;
use std::str;
pub(crate) use generated::*;

use std::ops::{RangeFrom, RangeTo};

use nom::combinator::recognize;
use nom::character::complete::char;
use nom::character::is_alphanumeric;
use nom::sequence::pair;
use nom::error::ParseError;
use nom::InputTakeAtPosition;
use nom::AsChar;
use nom::bytes::complete::take_while_m_n;
use nom::bytes::complete::take_while;
use nom::branch::alt;
use nom::character::complete::one_of;
use nom::combinator::opt;
use nom::sequence::tuple;
use nom::character::complete::multispace1;
use nom::character::complete::multispace0;
use nom::multi::separated_list;
use nom::{Slice, InputIter, Offset};
use nom::combinator::map;
use nom::bytes::complete::is_a;
use nom::multi::many0;

pub(crate) fn parse(i: &str) -> IResult<&str, Root> {
    map(many0(ty), Root::new)(i)
}

fn ty(i: &str) -> IResult<&str, Type> {
    alt((map(prod_type, |t| t.into()), map(sum_type, |t| t.into())))(i)
}

fn prod_type(i: &str) -> IResult<&str, ProdType> {
    map(tuple((multispace0, type_id, char_ms0('='), fields)), |(_, type_id, _, fields)| {
        ProdType::new(type_id, fields)
    })(i)
}

fn sum_type(i: &str) -> IResult<&str, SumType> {
    map(
        tuple((multispace0, type_id, char_ms0('='), constructors, opt(attrs))),
        |(_, type_id, _, constructors, attrs)| SumType::new(type_id, constructors, attrs),
    )(i)
}

fn attrs(i: &str) -> IResult<&str, Attrs> {
    map(pair(is_a("attributes"), fields), |(_, flds)| Attrs::new(flds))(i)
}

fn constructors(i: &str) -> IResult<&str, Vec<Constr>> {
    separated_list(char_ms0('|'), constructor)(i)
}

fn constructor(i: &str) -> IResult<&str, Constr> {
    map(pair(con_id, opt(fields)), |(con_id, fields)| {
        Constr::new(con_id, fields.unwrap_or_else(|| vec![]))
    })(i)
}

fn fields(i: &str) -> IResult<&str, Vec<Field>> {
    let fields = separated_list(char_ms0(','), field);
    let (i, (_, fields, _)) = tuple((char_ms0('('), fields, char_ms0(')')))(i)?;
    Ok((i, fields))
}

fn field(i: &str) -> IResult<&str, Field> {
    let (i, (type_id, rep, name)) =
        tuple((type_id, field_repetition, opt(pair(multispace1, id))))(i)?;
    let name = name.map(|n| n.1);
    if let Some(rep) = rep {
        match rep {
            '*' => Ok((i, Sequence::new(type_id, name).into())),
            '?' => Ok((i, Opt::new(type_id, name).into())),
            _ => unreachable!(),
        }
    } else {
        Ok((i, Single::new(type_id, name).into()))
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

fn field_repetition(i: &str) -> IResult<&str, Option<char>> {
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
    fn parse_field_repetition() {
        assert_eq!(field_repetition("? abcd"), Ok((" abcd", Some('?'))));
        assert_eq!(field_repetition("* abcd"), Ok((" abcd", Some('*'))));
        assert_eq!(field_repetition(" abcd"), Ok((" abcd", None)));
    }

    #[test]
    fn parse_field() {
        assert_eq!(field("type,"), Ok((",", Single::new(TypeId("type"), None).into())));
        assert_eq!(field("type?,"), Ok((",", Opt::new(TypeId("type"), None).into())));
        assert_eq!(field("type*,"), Ok((",", Sequence::new(TypeId("type"), None).into())));

        assert_eq!(
            field("type  name,"),
            Ok((",", Single::new(TypeId("type"), Some(Id("name"))).into()))
        );
        assert_eq!(
            field("type?  name,"),
            Ok((",", Opt::new(TypeId("type"), Some(Id("name"))).into()))
        );
        assert_eq!(
            field("type*  name,"),
            Ok((",", Sequence::new(TypeId("type"), Some(Id("name"))).into()))
        );
    }

    #[test]
    fn parse_fields() {
        assert_eq!(
            fields(" ( type1, type2? name  ) "),
            Ok((
                "",
                vec![
                    Single::new(TypeId("type1"), None).into(),
                    Opt::new(TypeId("type2"), Some(Id("name"))).into()
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
                        Single::new(TypeId("type1"), None).into(),
                        Opt::new(TypeId("type2"), Some(Id("name"))).into()
                    ]
                )
            ))
        );

        assert_eq!(constructor("ConstrId"), Ok(("", Constr::new(ConstrId("ConstrId"), vec![]))));
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
                            Single::new(TypeId("type1"), None).into(),
                            Opt::new(TypeId("type2"), Some(Id("name"))).into()
                        ]
                    ),
                    Constr::new(ConstrId("ConstrId2"), vec![])
                ]
            ))
        );
    }
}
