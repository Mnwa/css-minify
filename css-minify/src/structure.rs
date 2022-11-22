use derive_more::{Deref, DerefMut, Display, From, Into};
use indexmap::map::IndexMap;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Eq, PartialEq, Debug, From, Into)]
pub struct Block {
    pub selectors: Selectors,
    pub parameters: Parameters,
}

#[derive(Clone, Eq, PartialEq, Debug, From, Into)]
pub struct Media {
    pub screen: Name,
    pub entities: CssEntities,
}

#[derive(Clone, Eq, PartialEq, Debug, From, Into)]
pub struct Page {
    pub selectors: Option<Name>,
    pub parameters: Parameters,
}

#[derive(Clone, Eq, PartialEq, Debug, From, Into)]
pub struct Supports {
    pub conditions: Name,
    pub entities: CssEntities,
}

#[derive(Clone, Eq, PartialEq, Debug, From, Into)]
pub struct Keyframes {
    pub webkit_prefix: bool,
    pub name: Name,
    pub blocks: KeyframeBlocks,
}

#[derive(Clone, Eq, PartialEq, Debug, From, Into)]
pub struct KeyframeBlock {
    pub name: Name,
    pub parameters: Parameters,
}

#[derive(Clone, Eq, PartialEq, Debug, From, Into)]
pub struct FontFace {
    pub parameters: Parameters,
}

#[derive(Clone, Eq, PartialEq, Debug, From, Into)]
pub struct Viewport {
    pub parameters: Parameters,
}

#[derive(Clone, Eq, PartialEq, Debug, From, Into)]
pub struct MsViewport {
    pub parameters: Parameters,
}

#[derive(Clone, Eq, PartialEq, Debug, From, Into)]
pub struct NamespaceAt {
    prefix: Option<Value>,
    url: Value,
}

#[derive(Clone, Eq, PartialEq, Debug, From, Into)]
pub struct ImportAt {
    url: Value,
    media_queries: Option<Value>,
}

#[derive(Clone, Eq, PartialEq, Debug, From, Into)]
pub struct CharsetAt {
    charset: Value,
}

#[derive(Clone, Eq, PartialEq, Debug, From)]
pub enum At {
    Namespace(NamespaceAt),
    Import(ImportAt),
    Charset(CharsetAt),
}

#[derive(Clone, Eq, PartialEq, Debug, From, Display)]
pub enum CssEntity {
    Block(Block),
    Media(Media),
    Page(Page),
    Supports(Supports),
    FontFace(FontFace),
    Viewport(Viewport),
    MsViewport(MsViewport),
    Keyframes(Keyframes),
    At(At),
}

#[derive(Clone, Eq, PartialEq, Default, Debug, Deref, DerefMut, From, Into)]
pub struct Selectors(pub(crate) Vec<SelectorWithPseudoClasses>);

#[derive(Clone, Eq, PartialEq, Default, Debug, Deref, DerefMut, From, Into)]
pub struct Parameters(pub(crate) IndexMap<Name, Value>);

#[derive(Clone, Eq, PartialEq, Default, Debug, Deref, DerefMut, From, Into)]
pub struct Blocks(pub(crate) Vec<Block>);

#[derive(Clone, Eq, PartialEq, Default, Debug, Deref, DerefMut, From, Into)]
pub struct KeyframeBlocks(pub(crate) Vec<KeyframeBlock>);

#[derive(Clone, Eq, PartialEq, Default, Debug, Deref, DerefMut, From, Into)]
pub struct CssEntities(pub(crate) Vec<CssEntity>);

pub type Name = String;
pub type Value = String;

pub type Id = String;
pub type Class = String;
pub type Tag = String;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct SelectorWithPseudoClasses(pub Option<Selector>, pub Vec<PseudoClass>);
impl Display for SelectorWithPseudoClasses {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(s) = &self.0 {
            write!(f, "{}", s)?
        }
        for pc in self.1.iter() {
            write!(f, "{}", pc)?
        }

        Ok(())
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct PseudoClass {
    pub name: String,
    pub params: Option<String>,
    pub next: Option<String>,
}

impl Display for PseudoClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, ":{}", self.name)?;
        if let Some(params) = &self.params {
            write!(f, "({})", params)?;
        }
        if let Some(next) = &self.next {
            write!(f, " {}", next)?;
        }

        Ok(())
    }
}

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
        let parameters = self
            .0
            .iter()
            .map(|(name, value)| format!("{}:{}", name, value))
            .collect::<Vec<String>>();
        write!(f, "{}", parameters.join(";"))
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{{{}}}", self.selectors, self.parameters)
    }
}

impl Display for KeyframeBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{{{}}}", self.name, self.parameters)
    }
}

impl Display for Media {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@media {}{{{}}}", self.screen, self.entities)
    }
}

impl Display for Page {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@page")?;
        if let Some(selectors) = &self.selectors {
            write!(f, " {}", selectors)?
        }
        write!(f, " {{{}}}", self.parameters)?;
        Ok(())
    }
}

impl Display for Supports {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@supports {}{{{}}}", self.conditions, self.entities)
    }
}

impl Display for FontFace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@font-face {{{}}}", self.parameters)
    }
}

impl Display for Viewport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@viewport {{{}}}", self.parameters)
    }
}

impl Display for MsViewport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@-ms-viewport {{{}}}", self.parameters)
    }
}

impl Display for Keyframes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.webkit_prefix {
            write!(f, "@-webkit-keyframes {}{{{}}}", self.name, self.blocks)
        } else {
            write!(f, "@keyframes {}{{{}}}", self.name, self.blocks)
        }
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

impl Display for KeyframeBlocks {
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

impl Display for NamespaceAt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@namespace ")?;
        if let Some(prefix) = &self.prefix {
            write!(f, "{} ", prefix)?;
        }
        write!(f, "{};", self.url)?;
        Ok(())
    }
}

impl Display for CharsetAt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@charset {};", self.charset)
    }
}

impl Display for ImportAt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@import {};", self.url)?;
        if let Some(media_queries) = &self.media_queries {
            write!(f, " {}", media_queries)?
        }
        Ok(())
    }
}

impl Display for At {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            At::Namespace(n) => {
                write!(f, "{}", n)
            }
            At::Import(i) => {
                write!(f, "{}", i)
            }
            At::Charset(c) => write!(f, "{}", c),
        }
    }
}

impl Display for CssEntities {
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
    use crate::structure::{Block, Blocks, Selector, SelectorWithPseudoClasses};
    use indexmap::map::IndexMap;

    #[test]
    fn write_block() {
        let blocks: Blocks = vec![
            Block {
                selectors: vec![
                    SelectorWithPseudoClasses(Some(Selector::Id("some_id".into())), vec![]),
                    SelectorWithPseudoClasses(Some(Selector::Tag("input".into())), vec![]),
                ]
                .into(),
                parameters: {
                    let mut tmp = IndexMap::new();
                    tmp.insert("padding".into(), "5px 3px".into());
                    tmp.insert("color".into(), "white".into());
                    tmp.into()
                },
            },
            Block {
                selectors: vec![
                    SelectorWithPseudoClasses(Some(Selector::Id("some_id_2".into())), vec![]),
                    SelectorWithPseudoClasses(Some(Selector::Class("class".into())), vec![]),
                ]
                .into(),
                parameters: {
                    let mut tmp = IndexMap::new();
                    tmp.insert("padding".into(), "5px 4px".into());
                    tmp.insert("color".into(), "black".into());
                    tmp.into()
                },
            },
        ]
        .into();
        assert_eq!(format!("{}", blocks), "#some_id,input{padding:5px 3px;color:white}#some_id_2,.class{padding:5px 4px;color:black}")
    }
}
