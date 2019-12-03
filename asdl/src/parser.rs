use nom::IResult;

use super::ast::*;

use std::ops::{RangeFrom, RangeTo};

use nom::{Slice, InputIter, Offset, AsChar, InputTakeAtPosition};
use nom::error::{ParseError, VerboseError, context, VerboseErrorKind, ErrorKind};
use nom::character::is_alphanumeric;
use nom::character::complete::{char, one_of};
use nom::sequence::{pair, tuple};
use nom::bytes::complete::{take_while_m_n, take_while, is_a, tag};
use nom::branch::alt;
use nom::multi::{separated_list, many1, separated_nonempty_list};
use nom::combinator::{map, opt, recognize, peek};
use nom::character::complete::{multispace0, multispace1, line_ending, not_line_ending, space0};

#[derive(Clone, Debug, PartialEq)]
pub struct Error<I> {
    /// list of errors accumulated by `VerboseError`, containing the affected
    /// part of input data, and some context
    pub errors: std::vec::Vec<(I, VerboseErrorKind)>,
}

impl<I> Into<VerboseError<I>> for Error<I> {
    fn into(self) -> VerboseError<I> {
        VerboseError { errors: self.errors }
    }
}

impl<I> ParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        Error { errors: vec![(input, VerboseErrorKind::Nom(kind))] }
    }

    fn append(input: I, kind: ErrorKind, mut other: Self) -> Self {
        other.errors.push((input, VerboseErrorKind::Nom(kind)));
        other
    }

    fn from_char(input: I, c: char) -> Self {
        Error { errors: vec![(input, VerboseErrorKind::Char(c))] }
    }

    fn add_context(input: I, ctx: &'static str, mut other: Self) -> Self {
        if let Some((_, VerboseErrorKind::Nom(_))) = other.errors.last() {
            other.errors.pop();
        }
        other.errors.push((input, VerboseErrorKind::Context(ctx)));
        other
    }
}

type ParseResult<'a, T> = IResult<&'a str, T, Error<&'a str>>;

pub(crate) fn parse(i: &str) -> ParseResult<Root> {
    if i.is_empty() {
        return Ok((i, Root::new(vec![], vec![])));
    }
    map(
        tuple((
            multispace0,
            opt(pair(comments, multispace1)),
            multispace0,
            context("Expected at least one type declaration", many1(ty)),
        )),
        |(_, comments, _, types)| Root::new(types, comments.map(|c| c.0).unwrap_or_default()),
    )(i)
}

fn comment_line(i: &str) -> ParseResult<&str> {
    let (i, (_, _, _, comment)) = tuple((
        space0,
        context("Comment line should start with '//'", tag("//")),
        space0,
        not_line_ending,
    ))(i)?;
    Ok((i, comment))
}

fn comments(i: &str) -> ParseResult<Vec<&str>> {
    separated_list(line_ending, comment_line)(i)
}

fn ty(i: &str) -> ParseResult<Type> {
    context(
        "Expected Product or Sum type declaration",
        alt((map(prod_type, |t| t.into()), map(sum_type, |t| t.into()))),
    )(i)
}

fn prod_type(i: &str) -> ParseResult<ProdType> {
    map(
        tuple((comments, multispace0, type_id, char_ms0('='), fields)),
        |(comments, _, type_id, _, fields)| ProdType::new(type_id, fields, comments),
    )(i)
}

fn sum_type(i: &str) -> ParseResult<SumType> {
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

fn attrs(i: &str) -> ParseResult<Attrs> {
    map(pair(is_a("attributes"), fields), |(_, flds)| Attrs::new(flds))(i)
}

fn constructors(i: &str) -> ParseResult<Vec<Constr>> {
    separated_nonempty_list(char_ms0('|'), constructor)(i)
}

fn constructor(i: &str) -> ParseResult<Constr> {
    let (i, _) = multispace0(i)?;
    let (i, comments) = comments(i)?;
    let (i, _) = multispace0(i)?;
    let (i, con_id) = con_id(i)?;
    let (i, _) = multispace0(i)?;
    let (i, next) = opt(peek(char('(')))(i)?;
    let (i, fields) = if let Some(_) = next { fields(i)? } else { (i, vec![]) };
    Ok((i, Constr::new(con_id, fields, comments)))
}

fn fields(i: &str) -> ParseResult<Vec<Field>> {
    let fields = separated_list(char_ms0(','), field);
    map(tuple((char_ms0('('), fields, char_ms0(')'))), |(_, fields, _)| fields)(i)
}

fn field(i: &str) -> ParseResult<Field> {
    let (i, type_id) = type_id(i)?;
    let (i, arity) = peek(context("Expected * ? ) , or whitespace.", one_of("*? ),")))(i)?;
    let (i, arity) = match arity {
        '*' | '?' => one_of("*?")(i)?,
        _ => (i, ' '),
    };
    let (i, name) = opt(pair(multispace1, id))(i)?;
    let name = name.map(|n| n.1);
    match arity {
        '*' => Ok((i, Repeated::new(type_id, name).into())),
        '?' => Ok((i, Optional::new(type_id, name).into())),
        ' ' => Ok((i, Required::new(type_id, name).into())),
        _ => unreachable!(),
    }
}

fn type_id(i: &str) -> ParseResult<TypeId> {
    map(
        recognize(pair(
            context(
                "Type Id should start with lowercase character",
                take_while_m_n(1, 1, is_lowercase),
            ),
            context(
                "The rest of Type Id should be alphanumeric or underscore",
                take_while(is_alphanumeric_or_underscore),
            ),
        )),
        TypeId,
    )(i)
}

fn con_id(i: &str) -> ParseResult<ConstrId> {
    map(
        recognize(pair(
            context(
                "Constructor Id should start with uppercase character",
                take_while_m_n(1, 1, is_uppercase),
            ),
            context(
                "The rest of Constructor Id should be alphanumeric or underscore",
                take_while(is_alphanumeric_or_underscore),
            ),
        )),
        ConstrId,
    )(i)
}

fn id(i: &str) -> ParseResult<Id> {
    map(
        recognize(pair(
            context("Id should start with alpha character", take_while_m_n(1, 1, is_alpha)),
            context(
                "The rest of Id should be alphanumeric or underscore",
                take_while(is_alphanumeric_or_underscore),
            ),
        )),
        Id,
    )(i)
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

pub(crate) fn convert_error(input: &str, err: Error<&str>) -> String {
    nom::error::convert_error(input, err.into())
}

#[cfg(test)]
mod tests {

    use super::*;
    use nom;
    use nom::Err;
    use insta::assert_debug_snapshot_matches;
    use std::fmt::Debug;

    #[macro_export]
    macro_rules! assert_eq_text {
        ($left:expr, $right:expr) => {
            assert_eq_text!($left, $right,)
        };
        ($left:expr, $right:expr, $($tt:tt)*) => {{
            let left = $left.trim();
            let right = $right.trim();
            if left != right {
                let changeset = difference::Changeset::new(right, left, "\n");
                eprintln!("Left:\n{}\n\nRight:\n{}\n\nDiff:\n{}\n", left, right, changeset);
                eprintln!($($tt)*);
                panic!("text differs");
            }
        }};
    }

    #[test]
    fn parse_type_id() {
        assert_eq!(type_id("aBcd1_Efg"), Ok(("", TypeId("aBcd1_Efg"))));
        assert_eq!(type_id("aBcd1,Efg"), Ok((",Efg", TypeId("aBcd1"))));
        assert_error::<TypeId>(
            type_id,
            "ABcd1Efg",
            "
0: at line 0, in Type Id should start with lowercase character:
ABcd1Efg
^
            ",
        );
    }

    #[test]
    fn parse_con_id() {
        assert_eq!(con_id("ABcd1_Efg"), Ok(("", ConstrId("ABcd1_Efg"))));
        assert_eq!(con_id("ABcd1,Efg"), Ok((",Efg", ConstrId("ABcd1"))));
        assert_error::<ConstrId>(
            con_id,
            "aBcd1Efg",
            "
0: at line 0, in Constructor Id should start with uppercase character:
aBcd1Efg
^
            ",
        );
    }

    #[test]
    fn parse_id() {
        assert_eq!(id("ABcd1_Efg"), Ok(("", Id("ABcd1_Efg"))));
        assert_eq!(id("ABcd1,Efg"), Ok((",Efg", Id("ABcd1"))));
        assert_eq!(id("aBcd1_Efg"), Ok(("", Id("aBcd1_Efg"))));
        assert_eq!(id("aBcd1,Efg"), Ok((",Efg", Id("aBcd1"))));
        assert_error::<Id>(
            id,
            "_aBcd1Efg",
            "
0: at line 0, in Id should start with alpha character:
_aBcd1Efg
^
            ",
        );
    }

    #[test]
    fn parse_field() {
        assert_eq!(field("type,"), Ok((",", Required::new(TypeId("type"), None).into())));
        assert_eq!(field("type?,"), Ok((",", Optional::new(TypeId("type"), None).into())));
        assert_eq!(field("type*,"), Ok((",", Repeated::new(TypeId("type"), None).into())));
        assert_eq!(field("type)"), Ok((")", Required::new(TypeId("type"), None).into())));
        assert_eq!(field("type "), Ok((" ", Required::new(TypeId("type"), None).into())));

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
        assert_error::<Field>(
            field,
            "type+  name",
            "
0: at line 0, in Expected * ? ) , or whitespace.:
type+  name
    ^
            ",
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
        assert_error::<Vec<Field>>(
            fields,
            " ( type1 type2? name  ) ",
            "
0: at line 0:
 ( type1 type2? name  ) 
              ^
expected ')', found ?
            ",
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
        assert_error::<Constr>(
            constructor,
            "ConstrId1( type1,",
            "
0: at line 0:
ConstrId1( type1,
                ^
expected ')', found ,
            ",
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
        assert_error::<Vec<Constr>>(
            constructors,
            "ConstrId1( type1, type2? name    ConstrId2",
            "
0: at line 0:
ConstrId1( type1, type2? name    ConstrId2
                                 ^
expected ')', found C
            ",
        );
    }

    #[test]
    fn parse_comment_line() {
        let asdl = "  //comment line1";
        assert_eq!(comment_line(asdl), Ok(("", "comment line1")));
        assert_error::<&str>(
            comment_line,
            "/comment line1",
            "
0: at line 0, in Comment line should start with '//':
/comment line1
^
            ",
        );
    }

    #[test]
    fn parse_comments() {
        let asdl = "  // comment line1
                        // comment line2";
        assert_eq!(comments(asdl), Ok(("", vec!["comment line1", "comment line2"])));
    }

    #[test]
    fn parse_constructors_with_comments() {
        let asdl = r"  // ConstrId1 comment
                        ConstrId1( type1, type2? name  ) |
                        // ConstrId2 comment line1
                        // ConstrId2 comment line2
                        ConstrId2";
        assert_debug_snapshot_matches!("parse_constructors_with_comments", constructors(asdl));
    }

    #[test]
    fn parse_sum_type() {
        let asdl = r"
                    // SumType comment line 1
                    // SumType comment line 2
                    sumType = 
                        // ConstrId1 comment
                        ConstrId1( type1, type2? name  ) |
                        // ConstrId2 comment line1
                        // ConstrId2 comment line2
                        ConstrId2";
        assert_debug_snapshot_matches!("parse_sum_type", sum_type(asdl));
    }

    #[test]
    fn parse_prod_type() {
        let asdl = r#"  // prodType comment line 1
                        // prodType comment line 2
                        prodType = ( type1, type2? name  )"#;
        assert_debug_snapshot_matches!("parse_prod_type", prod_type(asdl));
    }

    #[test]
    fn parse_empty_asdl() {
        let asdl = "";
        assert_debug_snapshot_matches!("parse_empty_asdl", parse(asdl));
    }

    #[test]
    fn parse_error_invalid_single_type() {
        let asdl = r"
// comment line 1
// comment line 2
notType&";
        assert_error::<Root>(
            parse,
            asdl,
            r"
0: at line 3:
notType&
       ^
expected '=', found &

1: at line 3, in Expected Product or Sum type declaration:
notType&
^

2: at line 3, in Expected at least one type declaration:
notType&
^
            ",
        );
    }

    fn assert_error<'a, T: Debug>(
        f: fn(&'a str) -> ParseResult<'a, T>,
        txt: &'a str,
        error_msg: &'a str,
    ) {
        let res = f(txt);
        if let Err(Err::Error(err)) = res {
            let v = convert_error(txt, err);
            assert_eq_text!(error_msg, v)
        } else {
            assert!(false, "Expected error but get result:  {:?}", res);
        }
    }
}
