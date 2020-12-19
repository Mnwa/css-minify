use derive_more::{Deref, DerefMut, From, Into};
use nom::lib::std::fmt::Formatter;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Block {
    pub selectors: Selectors,
    pub parameters: Parameters,
}

#[derive(Clone, Eq, PartialEq, Default, Debug, Deref, DerefMut, From, Into)]
pub struct Selectors(Vec<Selector>);

#[derive(Clone, Eq, PartialEq, Default, Debug, Deref, DerefMut, From, Into)]
pub struct Parameters(HashMap<Name, Value>);

pub type Name = String;
pub type Value = String;

pub type Id = String;
pub type Class = String;
pub type Tag = String;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Selector {
    Id(Id),
    Class(Class),
    Tag(Tag),
}

impl Display for Selector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Selector::Id(id) => {
                write!(f, "#{}", id)
            }
            Selector::Class(class) => {
                write!(f, ".{}", class)
            }
            Selector::Tag(tag) => {
                write!(f, "{}", tag)
            }
        }
    }
}
