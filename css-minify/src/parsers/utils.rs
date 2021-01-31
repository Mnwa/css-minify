use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, tag, take_until};
use nom::character::complete::char;
use nom::character::complete::multispace1;
use nom::combinator::{map, not, peek};
use nom::error::Error as IError;
use nom::multi::many0;
use nom::sequence::{delimited, preceded, tuple};
use nom::{IResult, Parser};

pub fn non_useless<'a, O, P: Parser<&'a str, O, IError<&'a str>>>(
    parser: P,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, IError<&'a str>> {
    map(tuple((parse_useless, parser, parse_useless)), |(_, r, _)| r)
}

pub fn parse_useless(input: &str) -> IResult<&str, Vec<&str>> {
    many0(alt((multispace1, parse_comment)))(input)
}

pub fn parse_comment(input: &str) -> IResult<&str, &str> {
    map(
        tuple((tag("/*"), take_until("*/"), tag("*/"))),
        |(_, text, _)| text,
    )(input)
}

pub fn parse_to_block_open<'a, T: From<&'a str>>(input: &'a str) -> IResult<&'a str, T> {
    map(is_not("{"), |i: &str| T::from(i.trim()))(input)
}

pub fn is_not_block_ending<'a, O, P: Parser<&'a str, O, IError<&'a str>>>(
    parser: P,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, IError<&'a str>> {
    preceded(peek(not(alt((char('@'), char('{'), char('}'))))), parser)
}

pub fn some_block<'a, O, P: Parser<&'a str, O, IError<&'a str>>>(
    parser: P,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, IError<&'a str>> {
    delimited(char('{'), non_useless(parser), char('}'))
}

pub fn some_block_with_prefix<'a, O, P: Parser<&'a str, O, IError<&'a str>>>(
    prefix: &'a str,
    parser: P,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, IError<&'a str>> {
    preceded(non_useless(tag(prefix)), some_block(parser))
}

pub fn some_block_with_prefix_and_value<
    'a,
    O1,
    O2,
    P1: Parser<&'a str, O1, IError<&'a str>>,
    P2: Parser<&'a str, O2, IError<&'a str>>,
>(
    prefix: &'a str,
    parser1: P1,
    parser2: P2,
) -> impl FnMut(&'a str) -> IResult<&'a str, (O1, O2), IError<&'a str>> {
    preceded(
        non_useless(tag(prefix)),
        tuple((non_useless(parser1), some_block(parser2))),
    )
}

pub fn not_space(s: &str) -> IResult<&str, &str> {
    is_not(" \t\r\n")(s)
}

pub fn space(s: &str) -> IResult<&str, &str> {
    is_a(" \t\r\n")(s)
}

#[cfg(test)]
mod test {
    use crate::parsers::utils::parse_comment;

    #[test]
    fn test_comment() {
        assert_eq!(
            parse_comment("/* ***MEGA COMMENT*** */"),
            Ok(("", " ***MEGA COMMENT*** "))
        )
    }
}
