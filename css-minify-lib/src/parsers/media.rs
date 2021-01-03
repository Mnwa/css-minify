use crate::parsers::block::parse_blocks;
use crate::parsers::useless::non_useless;
use crate::structure::{Media, Value};
use nom::bytes::complete::{is_not, tag};
use nom::combinator::map;
use nom::sequence::tuple;
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

pub fn parse_screen(input: &str) -> IResult<&str, Value> {
    map(is_not("{"), |i: &str| Value::from(i.trim()))(input)
}

mod test {
    use crate::parsers::media::parse_media;
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
}
