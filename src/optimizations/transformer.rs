use crate::structure::{Block, Blocks, Name, Value};
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
    fn transform(
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
    fn transform(&mut self, block: Block) -> Block;
    fn transform_many(&mut self, blocks: Blocks) -> Blocks {
        Blocks(blocks.0.into_iter().map(|b| self.transform(b)).collect())
    }
}
