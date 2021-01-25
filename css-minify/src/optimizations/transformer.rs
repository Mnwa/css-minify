use crate::structure::{CssEntities, CssEntity, Media, Name, Parameters, Supports, Value};
use indexmap::map::IndexMap;

#[derive(Default)]
pub struct Transformer {
    parameters: Vec<TransformerParameterFn>,
}

pub enum TransformerParameterFn {
    Name(Box<dyn FnMut(Name) -> Name>),
    Value(Box<dyn FnMut(Value) -> Value>),
}

unsafe impl Send for TransformerParameterFn {}

impl Transformer {
    pub fn register_parameter(&mut self, transformer: TransformerParameterFn) {
        self.parameters.push(transformer)
    }
}

impl Transform for Transformer {
    fn transform_parameters(&mut self, parameters: Parameters) -> Parameters {
        parameters
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
            .collect::<IndexMap<Name, Value>>()
            .into()
    }
}

pub trait Transform {
    fn transform_parameters(&mut self, parameters: Parameters) -> Parameters;
    fn transform(&mut self, entity: CssEntity) -> CssEntity {
        match entity {
            CssEntity::Block(mut block) => {
                block.parameters = self.transform_parameters(block.parameters);
                CssEntity::Block(block)
            }
            CssEntity::Media(Media { screen, entities }) => CssEntity::Media(Media {
                screen,
                entities: entities
                    .0
                    .into_iter()
                    .map(|block| self.transform(block))
                    .collect::<Vec<_>>()
                    .into(),
            }),
            CssEntity::Supports(Supports {
                conditions,
                entities,
            }) => CssEntity::Supports(Supports {
                conditions,
                entities: entities
                    .0
                    .into_iter()
                    .map(|block| self.transform(block))
                    .collect::<Vec<_>>()
                    .into(),
            }),
            CssEntity::FontFace(mut font_face) => {
                font_face.parameters = self.transform_parameters(font_face.parameters);
                CssEntity::FontFace(font_face)
            }
            CssEntity::Page(mut page) => {
                page.parameters = self.transform_parameters(page.parameters);
                CssEntity::Page(page)
            }
            CssEntity::Viewport(mut viewport) => {
                viewport.parameters = self.transform_parameters(viewport.parameters);
                CssEntity::Viewport(viewport)
            }
            CssEntity::Keyframes(mut kf) => {
                kf.blocks.0 = kf
                    .blocks
                    .0
                    .into_iter()
                    .map(|mut block| {
                        block.parameters = self.transform_parameters(block.parameters);
                        block
                    })
                    .collect();
                CssEntity::Keyframes(kf)
            }
            CssEntity::At(at) => CssEntity::At(at),
        }
    }
    fn transform_many(&mut self, blocks: CssEntities) -> CssEntities {
        CssEntities(blocks.0.into_iter().map(|b| self.transform(b)).collect())
    }
}
