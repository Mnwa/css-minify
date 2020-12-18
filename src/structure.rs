use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Block {
    pub selectors: Vec<Selector>,
    pub parameters: HashMap<Name, Value>,
}

pub type Name = String;
pub type Value = String;

pub type Id = String;
pub type Class = String;
pub type Tag = String;
pub type Others = String;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Selector {
    Id(Id),
    Class(Class),
    Tag(Tag),
}
