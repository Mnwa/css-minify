use crate::parsers::parameters::parse_parameters;
use crate::parsers::selector::parse_selectors;
use crate::parsers::utils::{non_useless, some_block};
use crate::structure::Block;
use nom::combinator::into;
use nom::sequence::tuple;
use nom::IResult;

pub fn parse_block(input: &str) -> IResult<&str, Block> {
    into(tuple((
        non_useless(parse_selectors),
        some_block(parse_parameters),
    )))(input)
}

#[cfg(test)]
mod test {
    use crate::parsers::block::parse_block;
    use crate::parsers::utils::non_useless;
    use crate::structure::{Block, Selector, SelectorWithPseudoClasses};
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
                        SelectorWithPseudoClasses(Some(Selector::Id("some_id".into())), vec![]),
                        SelectorWithPseudoClasses(Some(Selector::Tag("input".into())), vec![]),
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
    #[test]
    fn test_block_empty() {
        assert_eq!(
            non_useless(parse_block)(
                r#"
                #some_id, input {
                    padding: 5px 3px; /* Mega comment */
                    color: white
                }

            "#
            ),
            Ok((
                "",
                Block {
                    selectors: vec![
                        SelectorWithPseudoClasses(Some(Selector::Id("some_id".into())), vec![]),
                        SelectorWithPseudoClasses(Some(Selector::Tag("input".into())), vec![]),
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
