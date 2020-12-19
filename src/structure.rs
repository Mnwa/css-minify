use derive_more::{Deref, DerefMut, From, Into};
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Block {
    pub selectors: Selectors,
    pub parameters: Parameters,
}

#[derive(Clone, Eq, PartialEq, Default, Debug, Deref, DerefMut, From, Into)]
pub struct Selectors(Vec<Selector>);

#[derive(Clone, Eq, PartialEq, Default, Debug, Deref, DerefMut, From, Into)]
pub struct Parameters(HashMap<Name, Value>);

#[derive(Clone, Eq, PartialEq, Default, Debug, Deref, DerefMut, From, Into)]
pub struct Blocks(Vec<Block>);

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

impl Display for Selectors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut selectors = self
            .0
            .iter()
            .map(|selector| format!("{}", selector))
            .collect::<Vec<String>>();
        selectors.sort();
        write!(f, "{}", selectors.join(","))
    }
}

impl Display for Parameters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut parameters = self
            .0
            .iter()
            .map(|(name, value)| format!("{}:{}", name, value))
            .collect::<Vec<String>>();
        parameters.sort();
        write!(f, "{}", parameters.join(";"))
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{{{}}}", self.selectors, self.parameters)
    }
}

impl Display for Blocks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|block| format!("{}", block))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

#[cfg(test)]
mod test {
    use crate::structure::{Block, Blocks, Selector};
    use std::collections::HashMap;

    #[test]
    fn write_block() {
        let blocks: Blocks = vec![
            Block {
                selectors: vec![
                    Selector::Id("some_id".into()),
                    Selector::Tag("input".into()),
                ]
                .into(),
                parameters: {
                    let mut tmp = HashMap::new();
                    tmp.insert("padding".into(), "5px 3px".into());
                    tmp.insert("color".into(), "white".into());
                    tmp.into()
                },
            },
            Block {
                selectors: vec![
                    Selector::Id("some_id_2".into()),
                    Selector::Class("class".into()),
                ]
                .into(),
                parameters: {
                    let mut tmp = HashMap::new();
                    tmp.insert("padding".into(), "5px 4px".into());
                    tmp.insert("color".into(), "black".into());
                    tmp.into()
                },
            },
        ]
        .into();
        assert_eq!(format!("{}", blocks), "#some_id,input{color:white;padding:5px 3px;}#some_id_2,.class{color:black;padding:5px 4px;}")
    }
}
