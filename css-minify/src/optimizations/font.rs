use crate::optimizations::transformer::Transform;
use crate::structure::{Parameters, Value};

#[derive(Default, Debug, Clone)]
pub(crate) struct FontTransformer;

impl Transform for FontTransformer {
    fn transform_parameters(&self, mut parameters: Parameters) -> Parameters {
        parameters
            .iter_mut()
            .filter(|(name, _)| matches!(name.as_str(), "font-weight"))
            .for_each(|(_, value): (_, &mut Value)| {
                *value = value
                    .replace("normal", "400")
                    .replace("bold", "700")
                    .replace('\"', "");
            });
        parameters
    }
}

#[cfg(test)]
mod test {
    use crate::optimizations::font::FontTransformer;
    use crate::optimizations::transformer::Transform;
    use crate::structure::{Block, CssEntities, CssEntity, Selector, SelectorWithPseudoClasses};
    use indexmap::map::IndexMap;

    #[test]
    fn test_blocks() {
        assert_eq!(
            FontTransformer::default().transform_many(CssEntities(vec![CssEntity::Block(Block {
                selectors: vec![SelectorWithPseudoClasses(
                    Some(Selector::Class("test".into())),
                    vec![],
                )]
                .into(),
                parameters: {
                    let mut tmp = IndexMap::new();
                    tmp.insert("font-weight".into(), "bold".into());
                    tmp.into()
                },
            }),])),
            CssEntities(vec![CssEntity::Block(Block {
                selectors: vec![SelectorWithPseudoClasses(
                    Some(Selector::Class("test".into())),
                    vec![],
                )]
                .into(),
                parameters: {
                    let mut tmp = IndexMap::new();
                    tmp.insert("font-weight".into(), "700".into());
                    tmp.into()
                },
            }),])
        )
    }
}
