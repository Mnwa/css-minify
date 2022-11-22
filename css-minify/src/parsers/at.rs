use crate::parsers::css_entity::parse_entities;
use crate::parsers::parameters::parse_parameters;
use crate::parsers::utils::{
    is_not_block_ending, non_useless, not_space, parse_to_block_open, some_block,
    some_block_with_prefix, some_block_with_prefix_and_value, space,
};
use crate::structure::{
    At, CharsetAt, FontFace, ImportAt, KeyframeBlock, KeyframeBlocks, Keyframes, Media, MsViewport,
    NamespaceAt, Page, Supports, Value, Viewport,
};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::char;
use nom::combinator::{into, map, map_parser, opt, rest};
use nom::error::Error as IError;
use nom::multi::many0;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

pub fn parse_media(input: &str) -> IResult<&str, Media> {
    into(some_block_with_prefix_and_value(
        "@media",
        parse_to_block_open,
        parse_entities,
    ))(input)
}

pub fn parse_page(input: &str) -> IResult<&str, Page> {
    into(some_block_with_prefix_and_value(
        "@page",
        opt(parse_to_block_open),
        parse_parameters,
    ))(input)
}

pub fn parse_supports(input: &str) -> IResult<&str, Supports> {
    into(some_block_with_prefix_and_value(
        "@supports",
        parse_to_block_open,
        parse_entities,
    ))(input)
}

pub fn parse_keyframes(input: &str) -> IResult<&str, Keyframes> {
    into(non_useless(tuple((
        alt((
            map(tag("@keyframes"), |_| false),
            map(tag("@-webkit-keyframes"), |_| true),
        )),
        non_useless(parse_to_block_open),
        some_block(parse_keyframe_blocks),
    ))))(input)
}

pub fn parse_keyframe_blocks(input: &str) -> IResult<&str, KeyframeBlocks> {
    into(many0(non_useless(parse_keyframe_block)))(input)
}

pub fn parse_keyframe_block(input: &str) -> IResult<&str, KeyframeBlock> {
    into(tuple((
        non_useless(is_not_block_ending(parse_to_block_open)),
        some_block(parse_parameters),
    )))(input)
}

pub fn parse_font_face(input: &str) -> IResult<&str, FontFace> {
    into(some_block_with_prefix("@font-face", parse_parameters))(input)
}

pub fn parse_viewport(input: &str) -> IResult<&str, Viewport> {
    into(some_block_with_prefix("@viewport", parse_parameters))(input)
}

pub fn parse_ms_viewport(input: &str) -> IResult<&str, MsViewport> {
    into(some_block_with_prefix("@-ms-viewport", parse_parameters))(input)
}

pub fn parse_at(input: &str) -> IResult<&str, At> {
    non_useless(alt((
        into(parse_charset),
        into(parse_namespace),
        into(parse_import),
    )))(input)
}

pub fn parse_charset(input: &str) -> IResult<&str, CharsetAt> {
    map(simple_at("@charset"), |s: &str| Value::from(s).into())(input)
}

pub fn parse_namespace(input: &str) -> IResult<&str, NamespaceAt> {
    map(
        map_parser(
            simple_at("@namespace"),
            tuple((opt(terminated(not_space, space)), rest)),
        ),
        |(prefix, url)| (prefix.map(Value::from), Value::from(url)).into(),
    )(input)
}

pub fn parse_import(input: &str) -> IResult<&str, ImportAt> {
    map(
        map_parser(
            simple_at("@import"),
            tuple((not_space, opt(preceded(space, rest)))),
        ),
        |(url, media_list)| (Value::from(url), media_list.map(Value::from)).into(),
    )(input)
}

fn simple_at<'a>(
    prefix: &'a str,
) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, IError<&'a str>> {
    preceded(tag(prefix), terminated(non_useless(is_not(";")), char(';')))
}

#[cfg(test)]
mod test {
    use crate::parsers::at::{
        parse_charset, parse_font_face, parse_import, parse_keyframes, parse_media,
        parse_ms_viewport, parse_namespace, parse_page, parse_supports, parse_viewport,
    };
    use crate::structure::{
        Block, CssEntity, FontFace, KeyframeBlock, Keyframes, Media, MsViewport, Name, Page,
        Selector, SelectorWithPseudoClasses, Supports, Value, Viewport,
    };
    use indexmap::map::IndexMap;

    #[test]
    fn test_media() {
        assert_eq!(
            parse_media(
                r#"
            @media only screen and (max-width: 992px) {
              .test {
                min-height: 68px; }
            }"#
            ),
            Ok((
                "",
                Media {
                    screen: Value::from("only screen and (max-width: 992px)"),
                    entities: vec![CssEntity::Block(Block {
                        selectors: vec![SelectorWithPseudoClasses(
                            Some(Selector::Class("test".into())),
                            None
                        )]
                        .into(),
                        parameters: {
                            let mut tmp = IndexMap::new();
                            tmp.insert("min-height".into(), "68px".into());
                            tmp.into()
                        }
                    })]
                    .into()
                }
            ))
        )
    }

    #[test]
    fn test_page() {
        assert_eq!(
            parse_page(
                r#"
                @page test {
                    size: a3; }"#
            ),
            Ok((
                "",
                Page {
                    selectors: Some(Name::from("test")),
                    parameters: {
                        let mut tmp: IndexMap<Name, Value> = IndexMap::new();
                        tmp.insert("size".to_string(), "a3".to_string());
                        tmp
                    }
                    .into()
                }
            ))
        )
    }

    #[test]
    fn test_page_without_prefix() {
        assert_eq!(
            parse_page(
                r#"
                @page {
                    size: a3; }"#
            ),
            Ok((
                "",
                Page {
                    selectors: None,
                    parameters: {
                        let mut tmp: IndexMap<Name, Value> = IndexMap::new();
                        tmp.insert("size".to_string(), "a3".to_string());
                        tmp
                    }
                    .into()
                }
            ))
        )
    }

    #[test]
    fn test_supports() {
        assert_eq!(
            parse_supports(
                r#"
            @supports (-ms-ime-align: auto) {
              .test {
                min-height: 68px; }
            }"#
            ),
            Ok((
                "",
                Supports {
                    conditions: Value::from("(-ms-ime-align: auto)"),
                    entities: vec![CssEntity::Block(Block {
                        selectors: vec![SelectorWithPseudoClasses(
                            Some(Selector::Class("test".into())),
                            None
                        )]
                        .into(),
                        parameters: {
                            let mut tmp = IndexMap::new();
                            tmp.insert("min-height".into(), "68px".into());
                            tmp.into()
                        }
                    })]
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
                    webkit_prefix: false,
                    blocks: vec![
                        KeyframeBlock {
                            name: "from".into(),
                            parameters: {
                                let mut tmp = IndexMap::new();
                                tmp.insert("margin-top".into(), "50px".into());
                                tmp.into()
                            },
                        },
                        KeyframeBlock {
                            name: "50%".into(),
                            parameters: {
                                let mut tmp = IndexMap::new();
                                tmp.insert("margin-top".into(), "150px !important".into());
                                tmp.into()
                            }
                        },
                        KeyframeBlock {
                            name: "to".into(),
                            parameters: {
                                let mut tmp = IndexMap::new();
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
            }"#
            ),
            Ok((
                "",
                FontFace {
                    parameters: {
                        let mut tmp: IndexMap<Name, Value> = IndexMap::new();
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
            }"#
            ),
            Ok((
                "",
                Viewport {
                    parameters: {
                        let mut tmp: IndexMap<Name, Value> = IndexMap::new();
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
    fn test_ms_viewport() {
        assert_eq!(
            parse_ms_viewport(
                r#"
            @-ms-viewport {
              min-width: 640px;
              max-width: 800px;
            }"#
            ),
            Ok((
                "",
                MsViewport {
                    parameters: {
                        let mut tmp: IndexMap<Name, Value> = IndexMap::new();
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
    fn test_namespace_without_prefix() {
        assert_eq!(
            parse_namespace("@namespace url(http://www.w3.org/2000/svg);"),
            Ok((
                "",
                (None, Value::from("url(http://www.w3.org/2000/svg)")).into()
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

    #[test]
    fn test_import_url() {
        assert_eq!(
            parse_import("@import url('landscape.css');"),
            Ok(("", (Value::from("url('landscape.css')"), None).into()))
        )
    }
}
