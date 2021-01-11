use crate::parsers::block::parse_blocks;
use crate::parsers::useless::non_useless;
use crate::structure::{At, CharsetAt, ImportAt, Media, NamespaceAt, Value};
use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, tag};
use nom::combinator::{into, map, map_parser, rest};
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::IResult;

pub fn parse_media(input: &str) -> IResult<&str, Media> {
    map(
        non_useless(tuple((
            tag("@media"),
            non_useless(parse_screen),
            tag("{"),
            non_useless(parse_blocks),
            tag("}"),
        ))),
        |(_, screen, _, blocks, _)| Media { screen, blocks },
    )(input)
}

pub fn parse_at(input: &str) -> IResult<&str, At> {
    non_useless(alt((
        into(parse_charset),
        into(parse_namespace),
        into(parse_import),
    )))(input)
}

pub fn parse_screen(input: &str) -> IResult<&str, Value> {
    map(is_not("{"), |i: &str| Value::from(i.trim()))(input)
}

pub fn parse_charset(input: &str) -> IResult<&str, CharsetAt> {
    map(
        preceded(
            tag("@charset"),
            terminated(non_useless(is_not(";")), tag(";")),
        ),
        |s: &str| Value::from(s).into(),
    )(input)
}

pub fn parse_namespace(input: &str) -> IResult<&str, NamespaceAt> {
    map(
        map_parser(
            preceded(
                tag("@namespace"),
                terminated(non_useless(is_not(";")), tag(";")),
            ),
            alt((
                separated_pair(is_not(" \t\r\n"), is_a(" \t\r\n"), rest),
                map(non_useless(rest), |s| ("", s)),
            )),
        ),
        |(prefix, url)| {
            if prefix.is_empty() {
                (None, Value::from(url))
            } else {
                (Some(Value::from(prefix)), Value::from(url))
            }
            .into()
        },
    )(input)
}

pub fn parse_import(input: &str) -> IResult<&str, ImportAt> {
    map(
        map_parser(
            preceded(
                tag("@import"),
                terminated(non_useless(is_not(";")), tag(";")),
            ),
            alt((
                separated_pair(is_not(" \t\r\n"), is_a(" \t\r\n"), rest),
                map(non_useless(rest), |s| ("", s)),
            )),
        ),
        |(url, media_list)| {
            if media_list.is_empty() {
                (Value::from(url), None)
            } else {
                (Value::from(url), Some(Value::from(media_list)))
            }
            .into()
        },
    )(input)
}

mod test {
    use crate::parsers::at::{parse_charset, parse_import, parse_media, parse_namespace};
    use crate::structure::{Block, Media, Selector, Value};
    use std::collections::HashMap;

    #[test]
    fn test_media() {
        assert_eq!(
            parse_media(
                r#"
            @media only screen and (max-width: 992px) {
              .test {
                min-height: 68px; }
            }
    "#
            ),
            Ok((
                "",
                Media {
                    screen: Value::from("only screen and (max-width: 992px)"),
                    blocks: vec![Block {
                        selectors: vec![Selector::Class("test".into())].into(),
                        parameters: {
                            let mut tmp = HashMap::new();
                            tmp.insert("min-height".into(), "68px".into());
                            tmp.into()
                        }
                    }]
                    .into()
                }
            ))
        )
    }

    #[test]
    fn test_charset() {
        assert_eq!(
            parse_charset("@charset \"UTF-8\";"),
            Ok(("", Value::from("\"UTF-8\"").into()))
        )
    }

    #[test]
    fn test_namespace() {
        assert_eq!(
            parse_namespace("@namespace svg url(http://www.w3.org/2000/svg);"),
            Ok((
                "",
                (
                    Some(Value::from("svg")),
                    Value::from("url(http://www.w3.org/2000/svg)")
                )
                    .into()
            ))
        )
    }

    #[test]
    fn test_import() {
        assert_eq!(
            parse_import("@import url('landscape.css') screen and (orientation:landscape);"),
            Ok((
                "",
                (
                    Value::from("url('landscape.css')"),
                    Some(Value::from("screen and (orientation:landscape)")),
                )
                    .into()
            ))
        )
    }
}
