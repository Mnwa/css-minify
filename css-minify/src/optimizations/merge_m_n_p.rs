use crate::optimizations::transformer::Transform;
use crate::optimizations::{if_some_has_important, none_or_has_important};
use crate::structure::{Name, Parameters, Value};
use nom::lib::std::fmt::Formatter;
use std::fmt::Display;

#[derive(Clone, Debug, Default)]
pub struct Merge;

#[derive(Debug, Default, Clone)]
pub struct Margin(Option<Value>, Option<Value>, Option<Value>, Option<Value>);
#[derive(Debug, Default, Clone)]
pub struct Padding(Option<Value>, Option<Value>, Option<Value>, Option<Value>);

impl Transform for Merge {
    fn transform_parameters(&self, mut parameters: Parameters) -> Parameters {
        let mut margin = Margin::default();
        let mut padding = Padding::default();
        parameters.0.iter().for_each(|(name, val)| {
            if !parameters.0.contains_key("margin") {
                margin.add(name, val.clone());
            }
            if !parameters.0.contains_key("padding") {
                padding.add(name, val.clone());
            }
        });

        if margin.is_may_be_merged() {
            parameters.insert(String::from("margin"), margin.to_string());
            parameters.0.swap_remove("margin-top");
            parameters.0.swap_remove("margin-bottom");
            parameters.0.swap_remove("margin-left");
            parameters.0.swap_remove("margin-right");
        }
        if padding.is_may_be_merged() {
            parameters.insert(String::from("padding"), padding.to_string());
            parameters.0.swap_remove("padding-top");
            parameters.0.swap_remove("padding-bottom");
            parameters.0.swap_remove("padding-left");
            parameters.0.swap_remove("padding-right");
        }

        parameters
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
        self.0.is_some()
            && self.1.is_some()
            && self.2.is_some()
            && self.3.is_some()
            && (self.all_elements_has_important() || self.no_one_element_has_no_important())
    }

    fn all_elements_has_important(&self) -> bool {
        if_some_has_important(self.0.as_ref())
            && if_some_has_important(self.1.as_ref())
            && if_some_has_important(self.2.as_ref())
            && if_some_has_important(self.3.as_ref())
    }

    fn no_one_element_has_no_important(&self) -> bool {
        !(none_or_has_important(self.0.as_ref())
            || none_or_has_important(self.1.as_ref())
            || none_or_has_important(self.2.as_ref())
            || none_or_has_important(self.3.as_ref()))
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
        self.0.is_some()
            && self.1.is_some()
            && self.2.is_some()
            && self.3.is_some()
            && (self.all_elements_has_important() || self.no_one_element_has_no_important())
    }

    fn all_elements_has_important(&self) -> bool {
        if_some_has_important(self.0.as_ref())
            && if_some_has_important(self.1.as_ref())
            && if_some_has_important(self.2.as_ref())
            && if_some_has_important(self.3.as_ref())
    }

    fn no_one_element_has_no_important(&self) -> bool {
        !(none_or_has_important(self.0.as_ref())
            || none_or_has_important(self.1.as_ref())
            || none_or_has_important(self.2.as_ref())
            || none_or_has_important(self.3.as_ref()))
    }
}

impl Display for Margin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (top, right, bottom, left) = (
            self.0
                .clone()
                .map(|m| m.trim_end_matches("!important").trim().to_string())
                .unwrap_or_else(|| String::from("auto")),
            self.1
                .clone()
                .map(|m| m.trim_end_matches("!important").trim().to_string())
                .unwrap_or_else(|| String::from("auto")),
            self.2
                .clone()
                .map(|m| m.trim_end_matches("!important").trim().to_string())
                .unwrap_or_else(|| String::from("auto")),
            self.3
                .clone()
                .map(|m| m.trim_end_matches("!important").trim().to_string())
                .unwrap_or_else(|| String::from("auto")),
        );

        if top == bottom && right == left && top == right {
            write!(f, "{}", top)?;
        } else if top == bottom && right == left {
            write!(f, "{} {}", top, right)?;
        } else if right == left {
            write!(f, "{} {} {}", top, right, bottom)?;
        } else {
            write!(f, "{} {} {} {}", top, right, bottom, left)?;
        }

        if self.all_elements_has_important() {
            write!(f, "!important")?;
        }

        Ok(())
    }
}

impl Display for Padding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (top, right, bottom, left) = (
            self.0
                .clone()
                .map(|m| m.trim_end_matches("!important").trim().to_string())
                .unwrap_or_else(|| String::from("auto")),
            self.1
                .clone()
                .map(|m| m.trim_end_matches("!important").trim().to_string())
                .unwrap_or_else(|| String::from("auto")),
            self.2
                .clone()
                .map(|m| m.trim_end_matches("!important").trim().to_string())
                .unwrap_or_else(|| String::from("auto")),
            self.3
                .clone()
                .map(|m| m.trim_end_matches("!important").trim().to_string())
                .unwrap_or_else(|| String::from("auto")),
        );

        if top == bottom && right == left && top == right {
            write!(f, "{}", top)?;
        } else if top == bottom && right == left {
            write!(f, "{} {}", top, right)?;
        } else if right == left {
            write!(f, "{} {} {}", top, right, bottom)?;
        } else {
            write!(f, "{} {} {} {}", top, right, bottom, left)?;
        }

        if self.all_elements_has_important() {
            write!(f, "!important")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::optimizations::merge_m_n_p::Merge;
    use crate::optimizations::transformer::Transform;
    use crate::structure::{Block, Parameters, Selectors};
    use indexmap::map::IndexMap;

    #[test]
    fn test_full_compress() {
        assert_eq!(
            Merge::default().transform(
                Block {
                    selectors: Selectors::default(),
                    parameters: {
                        let mut map = IndexMap::new();
                        map.insert("margin-top".into(), "3px".into());
                        map.insert("margin-bottom".into(), "3px".into());
                        map.insert("margin-left".into(), "3px".into());
                        map.insert("margin-right".into(), "3px".into());
                        Parameters(map)
                    },
                }
                .into()
            ),
            Block {
                selectors: Selectors::default(),
                parameters: {
                    let mut map = IndexMap::new();
                    map.insert("margin".into(), "3px".into());
                    Parameters(map)
                },
            }
            .into()
        )
    }

    #[test]
    fn test_compress_2() {
        assert_eq!(
            Merge::default().transform(
                Block {
                    selectors: Selectors::default(),
                    parameters: {
                        let mut map = IndexMap::new();
                        map.insert("margin-top".into(), "3px".into());
                        map.insert("margin-bottom".into(), "3px".into());
                        map.insert("margin-left".into(), "4px".into());
                        map.insert("margin-right".into(), "4px".into());
                        Parameters(map)
                    },
                }
                .into()
            ),
            Block {
                selectors: Selectors::default(),
                parameters: {
                    let mut map = IndexMap::new();
                    map.insert("margin".into(), "3px 4px".into());
                    Parameters(map)
                },
            }
            .into()
        )
    }

    #[test]
    fn test_compress_3() {
        assert_eq!(
            Merge::default().transform(
                Block {
                    selectors: Selectors::default(),
                    parameters: {
                        let mut map = IndexMap::new();
                        map.insert("margin-top".into(), "3px".into());
                        map.insert("margin-bottom".into(), "1px".into());
                        map.insert("margin-left".into(), "4px".into());
                        map.insert("margin-right".into(), "4px".into());
                        Parameters(map)
                    },
                }
                .into()
            ),
            Block {
                selectors: Selectors::default(),
                parameters: {
                    let mut map = IndexMap::new();
                    map.insert("margin".into(), "3px 4px 1px".into());
                    Parameters(map)
                },
            }
            .into()
        )
    }

    #[test]
    fn test_compress_4() {
        assert_eq!(
            Merge::default().transform(
                Block {
                    selectors: Selectors::default(),
                    parameters: {
                        let mut map = IndexMap::new();
                        map.insert("margin-top".into(), "3px".into());
                        map.insert("margin-bottom".into(), "1px".into());
                        map.insert("margin-left".into(), "2px".into());
                        map.insert("margin-right".into(), "4px".into());
                        Parameters(map)
                    },
                }
                .into()
            ),
            Block {
                selectors: Selectors::default(),
                parameters: {
                    let mut map = IndexMap::new();
                    map.insert("margin".into(), "3px 4px 1px 2px".into());
                    Parameters(map)
                },
            }
            .into()
        )
    }
}
