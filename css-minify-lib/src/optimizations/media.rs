use crate::optimizations::transformer::Transform;
use crate::structure::{CssEntities, CssEntity, Media, Parameters};
use indexmap::map::IndexMap;

#[derive(Default, Debug, Clone)]
pub struct MediaOptimizer;

impl Transform for MediaOptimizer {
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

    fn transform_many(&mut self, blocks: CssEntities) -> CssEntities {
        let mut media = IndexMap::new();
        blocks
            .0
            .iter()
            .filter_map(|e| match e {
                CssEntity::Media(m) => Some(m),
                _ => None,
            })
            .for_each(|e| {
                media
                    .entry(e.screen.clone())
                    .and_modify(|media: &mut Media| {
                        media
                            .entities
                            .0
                            .append(&mut self.transform_many(e.entities.clone()).0);
                    })
                    .or_insert(e.clone());
            });
        let mut entities = CssEntities(
            blocks
                .0
                .into_iter()
                .filter(|e| !matches!(e, CssEntity::Media(_)))
                .collect(),
        );
        entities.0.append(
            &mut media
                .into_iter()
                .map(|(_, m)| m)
                .map(|m| self.transform(m.into()))
                .collect(),
        );
        entities
    }
}

#[cfg(test)]
mod test {
    use crate::optimizations::media::MediaOptimizer;
    use crate::optimizations::transformer::Transform;
    use crate::structure::{Block, CssEntities, CssEntity, Media, Selector, Value};

    #[test]
    fn test_media() {
        assert_eq!(
            MediaOptimizer::default().transform_many(CssEntities(vec![
                CssEntity::Media(Media {
                    screen: Value::from("only screen and (max-width: 992px)"),
                    entities: vec![CssEntity::Block(Block {
                        selectors: vec![Selector::Class("test".into())].into(),
                        parameters: Default::default()
                    })]
                    .into()
                }),
                CssEntity::Media(Media {
                    screen: Value::from("only screen and (max-width: 992px)"),
                    entities: vec![CssEntity::Block(Block {
                        selectors: vec![Selector::Class("test2".into())].into(),
                        parameters: Default::default()
                    })]
                    .into()
                })
            ])),
            CssEntities(vec![CssEntity::Media(Media {
                screen: Value::from("only screen and (max-width:992px)"),
                entities: vec![
                    CssEntity::Block(Block {
                        selectors: vec![Selector::Class("test".into())].into(),
                        parameters: Default::default()
                    }),
                    CssEntity::Block(Block {
                        selectors: vec![Selector::Class("test2".into())].into(),
                        parameters: Default::default()
                    })
                ]
                .into()
            })])
        )
    }
}
