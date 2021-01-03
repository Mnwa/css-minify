use crate::structure::{Block, CssEntities, CssEntity, Media, Name, Value};
use std::collections::HashMap;

#[derive(Default)]
pub struct Transformer {
    parameters: Vec<TransformerParameterFn>,
}

pub enum TransformerParameterFn {
    Name(Box<dyn FnMut(Name) -> Name>),
    Value(Box<dyn FnMut(Value) -> Value>),
}

impl Transformer {
    pub fn register_parameter(&mut self, transformer: TransformerParameterFn) {
        self.parameters.push(transformer)
    }
}

impl Transform for Transformer {
    fn transform_block(
        &mut self,
        Block {
            selectors,
            parameters,
        }: Block,
    ) -> Block {
        Block {
            selectors,
            parameters: parameters
                .0
                .into_iter()
                .map(|(mut name, mut value)| {
                    for transformer in self.parameters.iter_mut() {
                        match transformer {
                            TransformerParameterFn::Name(t) => name = t(name),
                            TransformerParameterFn::Value(t) => value = t(value),
                        }
                    }
                    (name, value)
                })
                .collect::<HashMap<Name, Value>>()
                .into(),
        }
    }
}

pub trait Transform {
    fn transform_block(&mut self, block: Block) -> Block;
    fn transform(&mut self, entity: CssEntity) -> CssEntity {
        match entity {
            CssEntity::Block(block) => CssEntity::Block(self.transform_block(block)),
            CssEntity::Media(Media { screen, blocks }) => CssEntity::Media(Media {
                screen,
                blocks: blocks
                    .0
                    .into_iter()
                    .map(|block| self.transform_block(block))
                    .collect::<Vec<_>>()
                    .into(),
            }),
            e => e,
        }
    }
    fn transform_many(&mut self, blocks: CssEntities) -> CssEntities {
        CssEntities(blocks.0.into_iter().map(|b| self.transform(b)).collect())
    }
}
