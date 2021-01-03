use crate::parsers::parameters::parse_parameters;
use crate::parsers::selector::parse_selectors;
use crate::parsers::useless::non_useless;
use crate::structure::{Block, Blocks};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

pub fn parse_blocks(input: &str) -> IResult<&str, Blocks> {
    map(many0(non_useless(parse_block)), |blocks| blocks.into())(input)
}

pub fn parse_block(input: &str) -> IResult<&str, Block> {
    map(
        tuple((
            non_useless(parse_selectors),
            tag("{"),
            non_useless(parse_parameters),
            tag("}"),
        )),
        |(selectors, _, parameters, _)| Block {
            selectors,
            parameters,
        },
    )(input)
}

#[cfg(test)]
mod test {
    use crate::parsers::block::{parse_block, parse_blocks};
    use crate::parsers::useless::non_useless;
    use crate::structure::{Block, Selector};
    use std::collections::HashMap;

    #[test]
    fn test_block() {
        assert_eq!(
            non_useless(parse_block)(
                r#"
                #some_id, input {
                    padding: 5px 3px; /* Mega comment */
                    color: white;
                }

            "#
            ),
            Ok((
                "",
                Block {
                    selectors: vec![
                        Selector::Id("some_id".into()),
                        Selector::Tag("input".into())
                    ]
                    .into(),
                    parameters: {
                        let mut tmp = HashMap::new();
                        tmp.insert("padding".into(), "5px 3px".into());
                        tmp.insert("color".into(), "white".into());
                        tmp.into()
                    }
                }
            ))
        )
    }

    #[test]
    fn test_blocks() {
        assert_eq!(
            non_useless(parse_blocks)(
                r#"
                #some_id, input {
                    padding: 5px 3px; /* Mega comment */
                    color: white;
                }
                
                
                /* this is are test id */
                #some_id_2, .class {
                    padding: 5px 4px; /* Mega comment */
                    color: black;
                }
            "#
            ),
            Ok((
                "",
                vec![
                    Block {
                        selectors: vec![
                            Selector::Id("some_id".into()),
                            Selector::Tag("input".into())
                        ]
                        .into(),
                        parameters: {
                            let mut tmp = HashMap::new();
                            tmp.insert("padding".into(), "5px 3px".into());
                            tmp.insert("color".into(), "white".into());
                            tmp.into()
                        }
                    },
                    Block {
                        selectors: vec![
                            Selector::Id("some_id_2".into()),
                            Selector::Class("class".into())
                        ]
                        .into(),
                        parameters: {
                            let mut tmp = HashMap::new();
                            tmp.insert("padding".into(), "5px 4px".into());
                            tmp.insert("color".into(), "black".into());
                            tmp.into()
                        }
                    },
                ]
                .into()
            ))
        )
    }
}
