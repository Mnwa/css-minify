use crate::optimizations::transformer::Transform;
use crate::structure::{CssEntities, CssEntity, Media, Parameters, Supports};
use indexmap::map::IndexMap;

#[derive(Default, Debug, Clone)]
pub struct MergeMedia;

impl Transform for MergeMedia {
    fn transform_parameters(&mut self, parameters: Parameters) -> Parameters {
        parameters
    }

    fn transform(&mut self, entity: CssEntity) -> CssEntity {
        match entity {
            CssEntity::Media(mut m) => {
                m.screen = m.screen.replace(": ", ":");
                CssEntity::Media(m)
            }
            e => e,
        }
    }

    fn transform_many(&mut self, entities: CssEntities) -> CssEntities {
        let mut media = IndexMap::new();
        entities
            .0
            .iter()
            .filter_map(|e| match e {
                CssEntity::Media(m) => Some(m),
                _ => None,
            })
            .for_each(|m| {
                media
                    .entry(m.screen.clone())
                    .and_modify(|media: &mut Media| {
                        media
                            .entities
                            .0
                            .append(&mut self.transform_many(m.entities.clone()).0);
                    })
                    .or_insert(m.clone());
            });
        let mut non_media_entities = CssEntities(
            entities
                .0
                .into_iter()
                .filter(|e| !matches!(e, CssEntity::Media(_)))
                .map(|e| match e {
                    CssEntity::Supports(Supports {
                        conditions,
                        entities,
                    }) => Supports {
                        conditions,
                        entities: self.transform_many(entities),
                    }
                    .into(),
                    entity => entity,
                })
                .collect(),
        );
        non_media_entities.0.append(
            &mut media
                .into_iter()
                .map(|(_, m)| m)
                .map(|m| self.transform(m.into()))
                .collect(),
        );
        non_media_entities
    }
}

#[cfg(test)]
mod test {
    use crate::optimizations::merge_media::MergeMedia;
    use crate::optimizations::transformer::Transform;
    use crate::structure::{
        Block, CssEntities, CssEntity, Media, Selector, SelectorWithPseudoClasses, Value,
    };

    #[test]
    fn test_media() {
        assert_eq!(
            MergeMedia::default().transform_many(CssEntities(vec![
                CssEntity::Media(Media {
                    screen: Value::from("only screen and (max-width: 992px)"),
                    entities: vec![CssEntity::Block(Block {
                        selectors: vec![SelectorWithPseudoClasses(
                            Some(Selector::Class("test".into())),
                            None
                        )]
                        .into(),
                        parameters: Default::default()
                    })]
                    .into()
                }),
                CssEntity::Media(Media {
                    screen: Value::from("only screen and (max-width: 992px)"),
                    entities: vec![CssEntity::Block(Block {
                        selectors: vec![SelectorWithPseudoClasses(
                            Some(Selector::Class("test2".into())),
                            None
                        )]
                        .into(),
                        parameters: Default::default()
                    })]
                    .into()
                })
            ])),
            CssEntities(vec![CssEntity::Media(Media {
                screen: Value::from("only screen and (max-width:992px)"),
                entities: vec![
                    CssEntity::Block(Block {
                        selectors: vec![SelectorWithPseudoClasses(
                            Some(Selector::Class("test".into())),
                            None
                        )]
                        .into(),
                        parameters: Default::default()
                    }),
                    CssEntity::Block(Block {
                        selectors: vec![SelectorWithPseudoClasses(
                            Some(Selector::Class("test2".into())),
                            None
                        )]
                        .into(),
                        parameters: Default::default()
                    })
                ]
                .into()
            })])
        )
    }
}
