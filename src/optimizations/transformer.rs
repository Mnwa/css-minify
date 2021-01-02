use crate::structure::{Block, Blocks, Name, Selector, Value};
use std::collections::HashMap;

#[derive(Default)]
pub struct Transformer {
    selectors: Vec<TransformerSelectorFn>,
    parameters: Vec<TransformerParameterFn>,
}

pub struct TransformerSelectorFn(Box<dyn FnMut(Selector) -> Selector>);

pub enum TransformerParameterFn {
    Name(Box<dyn FnMut(Name) -> Name>),
    Value(Box<dyn FnMut(Value) -> Value>),
}

impl Transformer {
    pub fn register_selector(&mut self, transformer: TransformerSelectorFn) {
        self.selectors.push(transformer)
    }
    pub fn register_parameter(&mut self, transformer: TransformerParameterFn) {
        self.parameters.push(transformer)
    }

    pub fn transform(
        &mut self,
        Block {
            selectors,
            parameters,
        }: Block,
    ) -> Block {
        Block {
            selectors: selectors
                .0
                .into_iter()
                .map(|mut selector| {
                    for transformer in self.selectors.iter_mut() {
                        selector = transformer.0(selector)
                    }
                    selector
                })
                .collect::<Vec<Selector>>()
                .into(),
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

    pub fn transform_many(&mut self, blocks: Blocks) -> Blocks {
        Blocks(blocks.0.into_iter().map(|b| self.transform(b)).collect())
    }
}
