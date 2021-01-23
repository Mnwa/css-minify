use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::character::complete::char;
use nom::character::complete::multispace1;
use nom::combinator::{map, not};
use nom::error::Error as IError;
use nom::multi::many0;
use nom::sequence::{preceded, tuple};
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
    preceded(not(alt((char('@'), char('{'), char('}')))), parser)
}

#[cfg(test)]
mod test {
    use crate::parsers::useless::parse_comment;

    #[test]
    fn test_comment() {
        assert_eq!(
            parse_comment("/* ***MEGA COMMENT*** */"),
            Ok(("", " ***MEGA COMMENT*** "))
        )
    }
}
