use crate::parsers::block::parse_blocks;
use crate::parsers::parameters::parse_parameters;
use crate::parsers::useless::{is_not_block_ending, non_useless, parse_to_block_open};
use crate::structure::{
    At, CharsetAt, FontFace, ImportAt, KeyframeBlock, KeyframeBlocks, Keyframes, Media,
    NamespaceAt, Value, Viewport,
};
use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, tag};
use nom::character::complete::char;
use nom::combinator::{into, map, map_parser, not, rest};
use nom::multi::many0;
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::IResult;

pub fn parse_media(input: &str) -> IResult<&str, Media> {
    map(
        non_useless(tuple((
            tag("@media"),
            non_useless(parse_to_block_open),
            tag("{"),
            non_useless(parse_blocks),
            tag("}"),
        ))),
        |(_, screen, _, blocks, _)| Media { screen, blocks },
    )(input)
}

pub fn parse_keyframes(input: &str) -> IResult<&str, Keyframes> {
    map(
        non_useless(tuple((
            tag("@keyframes"),
            non_useless(parse_to_block_open),
            tag("{"),
            non_useless(parse_keyframe_blocks),
            tag("}"),
        ))),
        |(_, name, _, blocks, _)| Keyframes { name, blocks },
    )(input)
}

pub fn parse_keyframe_blocks(input: &str) -> IResult<&str, KeyframeBlocks> {
    map(many0(non_useless(parse_keyframe_block)), |blocks| {
        blocks.into()
    })(input)
}

pub fn parse_keyframe_block(input: &str) -> IResult<&str, KeyframeBlock> {
    map(
        tuple((
            non_useless(is_not_block_ending(parse_to_block_open)),
            tag("{"),
            non_useless(parse_parameters),
            tag("}"),
        )),
        |(name, _, parameters, _)| KeyframeBlock { name, parameters },
    )(input)
}

pub fn parse_font_face(input: &str) -> IResult<&str, FontFace> {
    map(
        non_useless(tuple((
            non_useless(tag("@font-face")),
            tag("{"),
            non_useless(parse_parameters),
            tag("}"),
        ))),
        |(_, _, parameters, _)| FontFace { parameters },
    )(input)
}

pub fn parse_viewport(input: &str) -> IResult<&str, Viewport> {
    map(
        non_useless(tuple((
            non_useless(tag("@viewport")),
            tag("{"),
            non_useless(parse_parameters),
            tag("}"),
        ))),
        |(_, _, parameters, _)| Viewport { parameters },
    )(input)
}

pub fn parse_at(input: &str) -> IResult<&str, At> {
    non_useless(alt((
        into(parse_charset),
        into(parse_namespace),
        into(parse_import),
    )))(input)
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
    use crate::parsers::at::{
        parse_charset, parse_font_face, parse_import, parse_keyframes, parse_media,
        parse_namespace, parse_viewport,
    };
    use crate::structure::{
        Block, FontFace, KeyframeBlock, Keyframes, Media, Name, Selector, Value, Viewport,
    };
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
    fn test_keyframes() {
        assert_eq!(
            parse_keyframes(
                r#"
            @keyframes important1 {
              from { margin-top: 50px; }
              50%  { margin-top: 150px !important; } /* ignored */
              to   { margin-top: 100px; }
            }
    "#
            ),
            Ok((
                "",
                Keyframes {
                    name: Value::from("important1"),
                    blocks: vec![
                        KeyframeBlock {
                            name: "from".into(),
                            parameters: {
                                let mut tmp = HashMap::new();
                                tmp.insert("margin-top".into(), "50px".into());
                                tmp.into()
                            }
                        },
                        KeyframeBlock {
                            name: "50%".into(),
                            parameters: {
                                let mut tmp = HashMap::new();
                                tmp.insert("margin-top".into(), "150px !important".into());
                                tmp.into()
                            }
                        },
                        KeyframeBlock {
                            name: "to".into(),
                            parameters: {
                                let mut tmp = HashMap::new();
                                tmp.insert("margin-top".into(), "100px".into());
                                tmp.into()
                            }
                        }
                    ]
                    .into()
                }
            ))
        )
    }

    #[test]
    fn test_font_face() {
        assert_eq!(
            parse_font_face(
                r#"
            @font-face {
                font-family: "Open Sans";
                src: url(/fonts/OpenSans-Regular-webfont.woff2) format("woff2");
            }
    "#
            ),
            Ok((
                "",
                FontFace {
                    parameters: {
                        let mut tmp: HashMap<Name, Value> = HashMap::new();
                        tmp.insert("font-family".to_string(), "\"Open Sans\"".to_string());
                        tmp.insert(
                            "src".to_string(),
                            "url(/fonts/OpenSans-Regular-webfont.woff2) format(\"woff2\")"
                                .to_string(),
                        );
                        tmp
                    }
                    .into()
                }
            ))
        )
    }

    #[test]
    fn test_viewport() {
        assert_eq!(
            parse_viewport(
                r#"
            @viewport {
              min-width: 640px;
              max-width: 800px;
            }
    "#
            ),
            Ok((
                "",
                Viewport {
                    parameters: {
                        let mut tmp: HashMap<Name, Value> = HashMap::new();
                        tmp.insert("min-width".to_string(), "640px".to_string());
                        tmp.insert("max-width".to_string(), "800px".to_string());
                        tmp
                    }
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
