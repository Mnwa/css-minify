use crate::parsers::parameters::parse_parameters;
use crate::parsers::selector::parse_selectors;
use crate::parsers::useless::non_useless;
use crate::structure::Block;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

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
    use crate::parsers::block::parse_block;
    use crate::parsers::useless::non_useless;
    use crate::structure::{Block, Selector};
    use indexmap::map::IndexMap;

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
                        let mut tmp = IndexMap::new();
                        tmp.insert("padding".into(), "5px 3px".into());
                        tmp.insert("color".into(), "white".into());
                        tmp.into()
                    }
                }
            ))
        )
    }
}
