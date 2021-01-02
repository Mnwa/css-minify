use crate::optimizations::transformer::Transform;
use crate::structure::{Block, Name, Value};
use nom::lib::std::fmt::Formatter;
use std::fmt::Display;

#[derive(Clone, Debug, Default)]
pub struct Merge;

#[derive(Debug, Default, Clone)]
pub struct Margin(Option<Value>, Option<Value>, Option<Value>, Option<Value>);
#[derive(Debug, Default, Clone)]
pub struct Padding(Option<Value>, Option<Value>, Option<Value>, Option<Value>);

impl Transform for Merge {
    fn transform(&mut self, mut block: Block) -> Block {
        let mut margin = Margin::default();
        let mut padding = Padding::default();
        block.parameters.0.iter().for_each(|(name, val)| {
            if !block.parameters.0.contains_key("margin") {
                margin.add(name, val.clone());
            }
            if !block.parameters.0.contains_key("padding") {
                padding.add(name, val.clone());
            }
        });

        if margin.is_may_be_merged() {
            block
                .parameters
                .insert(String::from("margin"), margin.to_string());
            block.parameters.0.remove("margin-top");
            block.parameters.0.remove("margin-bottom");
            block.parameters.0.remove("margin-left");
            block.parameters.0.remove("margin-right");
        }
        if padding.is_may_be_merged() {
            block
                .parameters
                .insert(String::from("padding"), padding.to_string());
            block.parameters.0.remove("padding-top");
            block.parameters.0.remove("padding-bottom");
            block.parameters.0.remove("padding-left");
            block.parameters.0.remove("padding-right");
        }

        block
    }
}

impl Margin {
    fn add(&mut self, name: &Name, value: Value) -> bool {
        match name.as_str() {
            "margin-top" => {
                self.0 = Some(value);
                true
            }
            "margin-right" => {
                self.1 = Some(value);
                true
            }
            "margin-bottom" => {
                self.2 = Some(value);
                true
            }
            "margin-left" => {
                self.3 = Some(value);
                true
            }
            _ => false,
        }
    }

    fn is_may_be_merged(&self) -> bool {
        self.0.is_some() && self.1.is_some() && self.2.is_some() && self.3.is_some()
    }
}

impl Padding {
    fn add(&mut self, name: &Name, value: Value) -> bool {
        match name.as_str() {
            "padding-top" => {
                self.0 = Some(value);
                true
            }
            "padding-right" => {
                self.1 = Some(value);
                true
            }
            "padding-bottom" => {
                self.2 = Some(value);
                true
            }
            "padding-left" => {
                self.3 = Some(value);
                true
            }
            _ => false,
        }
    }

    fn is_may_be_merged(&self) -> bool {
        self.0.is_some() && self.1.is_some() && self.2.is_some() && self.3.is_some()
    }
}

impl Display for Margin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (top, right, bottom, left) = (
            self.0.clone().unwrap_or_else(|| String::from("auto")),
            self.1.clone().unwrap_or_else(|| String::from("auto")),
            self.2.clone().unwrap_or_else(|| String::from("auto")),
            self.3.clone().unwrap_or_else(|| String::from("auto")),
        );

        if top == bottom && right == left && top == right {
            return write!(f, "{}", top);
        }
        if top == bottom && right == left {
            return write!(f, "{} {}", top, right);
        }
        if right == left {
            return write!(f, "{} {} {}", top, right, bottom);
        }

        write!(f, "{} {} {} {}", top, right, bottom, left)
    }
}

impl Display for Padding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (top, right, bottom, left) = (
            self.0.clone().unwrap_or_else(|| String::from("auto")),
            self.1.clone().unwrap_or_else(|| String::from("auto")),
            self.2.clone().unwrap_or_else(|| String::from("auto")),
            self.3.clone().unwrap_or_else(|| String::from("auto")),
        );

        if top == bottom && right == left {
            return write!(f, "{} {}", top, right);
        }
        if right == left {
            return write!(f, "{} {} {}", top, right, bottom);
        }

        write!(f, "{} {} {} {}", top, right, bottom, left)
    }
}

mod test {
    use crate::optimizations::merge_m_n_p::Merge;
    use crate::optimizations::transformer::Transform;
    use crate::structure::{Block, Parameters, Selectors};
    use std::collections::HashMap;

    #[test]
    fn test_full_compress() {
        assert_eq!(
            Merge::default().transform(Block {
                selectors: Selectors::default(),
                parameters: {
                    let mut map = HashMap::new();
                    map.insert("margin-top".into(), "3px".into());
                    map.insert("margin-bottom".into(), "3px".into());
                    map.insert("margin-left".into(), "3px".into());
                    map.insert("margin-right".into(), "3px".into());
                    Parameters(map)
                },
            }),
            Block {
                selectors: Selectors::default(),
                parameters: {
                    let mut map = HashMap::new();
                    map.insert("margin".into(), "3px".into());
                    Parameters(map)
                },
            }
        )
    }

    #[test]
    fn test_compress_2() {
        assert_eq!(
            Merge::default().transform(Block {
                selectors: Selectors::default(),
                parameters: {
                    let mut map = HashMap::new();
                    map.insert("margin-top".into(), "3px".into());
                    map.insert("margin-bottom".into(), "3px".into());
                    map.insert("margin-left".into(), "4px".into());
                    map.insert("margin-right".into(), "4px".into());
                    Parameters(map)
                },
            }),
            Block {
                selectors: Selectors::default(),
                parameters: {
                    let mut map = HashMap::new();
                    map.insert("margin".into(), "3px 4px".into());
                    Parameters(map)
                },
            }
        )
    }

    #[test]
    fn test_compress_3() {
        assert_eq!(
            Merge::default().transform(Block {
                selectors: Selectors::default(),
                parameters: {
                    let mut map = HashMap::new();
                    map.insert("margin-top".into(), "3px".into());
                    map.insert("margin-bottom".into(), "1px".into());
                    map.insert("margin-left".into(), "4px".into());
                    map.insert("margin-right".into(), "4px".into());
                    Parameters(map)
                },
            }),
            Block {
                selectors: Selectors::default(),
                parameters: {
                    let mut map = HashMap::new();
                    map.insert("margin".into(), "3px 4px 1px".into());
                    Parameters(map)
                },
            }
        )
    }

    #[test]
    fn test_compress_4() {
        assert_eq!(
            Merge::default().transform(Block {
                selectors: Selectors::default(),
                parameters: {
                    let mut map = HashMap::new();
                    map.insert("margin-top".into(), "3px".into());
                    map.insert("margin-bottom".into(), "1px".into());
                    map.insert("margin-left".into(), "2px".into());
                    map.insert("margin-right".into(), "4px".into());
                    Parameters(map)
                },
            }),
            Block {
                selectors: Selectors::default(),
                parameters: {
                    let mut map = HashMap::new();
                    map.insert("margin".into(), "3px 4px 1px 2px".into());
                    Parameters(map)
                },
            }
        )
    }
}
