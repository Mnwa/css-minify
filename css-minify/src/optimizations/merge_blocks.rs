use crate::optimizations::transformer::Transform;
use crate::structure::{Block, CssEntities, CssEntity, Media, Parameters, Supports};
use indexmap::map::IndexMap;

#[derive(Default, Debug, Clone)]
pub struct MergeBlocks;

impl Transform for MergeBlocks {
    fn transform_parameters(&mut self, parameters: Parameters) -> Parameters {
        parameters
    }

    fn transform(&mut self, entity: CssEntity) -> CssEntity {
        entity
    }

    fn transform_many(&mut self, entities: CssEntities) -> CssEntities {
        let mut blocks_to_merge = IndexMap::new();
        entities
            .0
            .iter()
            .filter_map(|e| match e {
                CssEntity::Block(b) => Some(b),
                _ => None,
            })
            .for_each(|b| {
                blocks_to_merge
                    .entry(b.selectors.clone().to_string())
                    .and_modify(|block: &mut Block| {
                        b.parameters.0.iter().for_each(|(n, v)| {
                            block.parameters.0.insert(n.clone(), v.clone());
                        });
                    })
                    .or_insert(b.clone());
            });
        let mut non_block_entities = CssEntities(
            entities
                .0
                .into_iter()
                .filter(|e| !matches!(e, CssEntity::Block(_)))
                .map(|e| match e {
                    CssEntity::Media(Media { screen, entities }) => Media {
                        screen,
                        entities: self.transform_many(entities),
                    }
                    .into(),
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
        non_block_entities
            .0
            .append(&mut blocks_to_merge.into_iter().map(|(_, m)| m.into()).collect());
        non_block_entities
    }
}

#[cfg(test)]
mod test {
    use crate::optimizations::merge_blocks::MergeBlocks;
    use crate::optimizations::transformer::Transform;
    use crate::structure::{Block, CssEntities, CssEntity, Selector};
    use indexmap::map::IndexMap;

    #[test]
    fn test_blocks() {
        assert_eq!(
            MergeBlocks::default().transform_many(CssEntities(vec![
                CssEntity::Block(Block {
                    selectors: vec![Selector::Class("test".into())].into(),
                    parameters: {
                        let mut tmp = IndexMap::new();
                        tmp.insert("background-color".into(), "#f64e60 !important".into());
                        tmp.into()
                    }
                }),
                CssEntity::Block(Block {
                    selectors: vec![Selector::Class("test".into())].into(),
                    parameters: {
                        let mut tmp = IndexMap::new();
                        tmp.insert("color".into(), "#f64e60 !important".into());
                        tmp.into()
                    }
                }),
            ])),
            CssEntities(vec![CssEntity::Block(Block {
                selectors: vec![Selector::Class("test".into())].into(),
                parameters: {
                    let mut tmp = IndexMap::new();
                    tmp.insert("background-color".into(), "#f64e60 !important".into());
                    tmp.insert("color".into(), "#f64e60 !important".into());
                    tmp.into()
                }
            })])
        )
    }
}
