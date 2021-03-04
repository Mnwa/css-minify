use crate::optimizations::transformer::Transform;
use crate::optimizations::{if_some_has_important, none_or_has_important};
use crate::structure::{Name, Parameters, Value};
use nom::lib::std::fmt::Formatter;
use std::fmt::Display;

#[derive(Default, Debug, Clone)]
pub struct MergeShortHand;

impl Transform for MergeShortHand {
    fn transform_parameters(&mut self, mut parameters: Parameters) -> Parameters {
        let mut font = FontShortHand::default();
        let mut list = ListShortHand::default();
        let mut background = BackgroundShortHand::default();
        let mut border = BorderShortHand::default();
        let mut outline = OutlineShortHand::default();
        let mut transition = TransitionShortHand::default();

        parameters
            .0
            .iter()
            .for_each(|(name, value): (&Name, &Value)| {
                if !parameters.0.contains_key("font") {
                    font.add(name, value.clone());
                }
                if !parameters.0.contains_key("list-style") {
                    list.add(name, value.clone());
                }
                if !parameters.0.contains_key("background") {
                    background.add(name, value.clone());
                }
                if !parameters.0.contains_key("border") {
                    border.add(name, value.clone());
                }
                if !parameters.0.contains_key("outline") {
                    outline.add(name, value.clone());
                }
                if !parameters.0.contains_key("transition") {
                    transition.add(name, value.clone());
                }
            });

        if font.is_maybe_shorted() {
            parameters
                .0
                .insert(String::from("font"), font.to_string().trim().to_string());
            parameters.0.remove("font-style");
            parameters.0.remove("font-variant");
            parameters.0.remove("font-weight");
            parameters.0.remove("font-size");
            parameters.0.remove("line-height");
            parameters.0.remove("font-family");
        }

        if list.is_maybe_shorted() {
            parameters.0.insert(
                String::from("list-style"),
                list.to_string().trim().to_string(),
            );
            parameters.0.remove("list-style-type");
            parameters.0.remove("list-style-position");
            parameters.0.remove("list-style-image");
        }

        if background.is_maybe_shorted() {
            parameters.0.insert(
                String::from("background"),
                background.to_string().trim().to_string(),
            );
            parameters.0.remove("background-attachment");
            parameters.0.remove("background-color");
            parameters.0.remove("background-position");
            parameters.0.remove("background-repeat");
            parameters.0.remove("background-image");
        }

        if border.is_maybe_shorted() {
            parameters.0.insert(
                String::from("border"),
                border.to_string().trim().to_string(),
            );
            parameters.0.remove("border-width");
            parameters.0.remove("border-style");
            parameters.0.remove("border-color");
        }

        if outline.is_maybe_shorted() {
            parameters.0.insert(
                String::from("outline"),
                outline.to_string().trim().to_string(),
            );
            parameters.0.remove("outline-width");
            parameters.0.remove("outline-style");
            parameters.0.remove("outline-color");
        }

        if transition.is_maybe_shorted() {
            parameters.0.insert(
                String::from("transition"),
                transition.to_string().trim().to_string(),
            );
            parameters.0.remove("transition-property");
            parameters.0.remove("transition-duration");
            parameters.0.remove("transition-delay");
            parameters.0.remove("transition-timing-function");
        }

        parameters
    }
}

#[derive(Debug, Default)]
struct FontShortHand {
    font_style: Option<Value>,
    font_variant: Option<Value>,
    font_weight: Option<Value>,
    font_size: Option<Value>,
    line_height: Option<Value>,
    font_family: Option<Value>,
}

impl FontShortHand {
    fn is_maybe_shorted(&self) -> bool {
        self.font_size.is_some()
            && self.font_family.is_some()
            && (self.all_elements_has_important() || self.no_one_element_has_no_important())
    }

    fn all_elements_has_important(&self) -> bool {
        if_some_has_important(self.font_style.as_ref())
            && if_some_has_important(self.font_variant.as_ref())
            && if_some_has_important(self.font_weight.as_ref())
            && if_some_has_important(self.font_size.as_ref())
            && if_some_has_important(self.line_height.as_ref())
            && if_some_has_important(self.font_family.as_ref())
    }

    fn no_one_element_has_no_important(&self) -> bool {
        !(none_or_has_important(self.font_style.as_ref())
            || none_or_has_important(self.font_variant.as_ref())
            || none_or_has_important(self.font_weight.as_ref())
            || none_or_has_important(self.font_size.as_ref())
            || none_or_has_important(self.line_height.as_ref())
            || none_or_has_important(self.font_family.as_ref()))
    }

    fn add(&mut self, name: &Name, value: Value) {
        match name.as_str() {
            "font-style" => self.font_style = Some(value),
            "font-variant" => self.font_variant = Some(value),
            "font-weight" => self.font_weight = Some(value),
            "font-size" => self.font_size = Some(value),
            "line-height" => self.line_height = Some(value),
            "font-family" => self.font_family = Some(value),
            _ => {}
        }
    }
}

impl Display for FontShortHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = &self.font_style {
            write!(f, "{}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.font_variant {
            write!(f, " {}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.font_weight {
            write!(f, " {}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.font_size {
            write!(f, " {}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.line_height {
            write!(f, "/{}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.font_family {
            write!(f, " {}", v.trim_end_matches("!important").trim())?;
        }
        if self.all_elements_has_important() {
            write!(f, "!important")?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
struct ListShortHand {
    list_style_type: Option<Value>,
    list_style_position: Option<Value>,
    list_style_image: Option<Value>,
}

impl ListShortHand {
    fn is_maybe_shorted(&self) -> bool {
        (self.list_style_type.is_some()
            || self.list_style_position.is_some()
            || self.list_style_image.is_some())
            && (self.all_elements_has_important() || self.no_one_element_has_no_important())
    }

    fn all_elements_has_important(&self) -> bool {
        if_some_has_important(self.list_style_type.as_ref())
            && if_some_has_important(self.list_style_position.as_ref())
            && if_some_has_important(self.list_style_image.as_ref())
    }

    fn no_one_element_has_no_important(&self) -> bool {
        !(none_or_has_important(self.list_style_type.as_ref())
            || none_or_has_important(self.list_style_position.as_ref())
            || none_or_has_important(self.list_style_image.as_ref()))
    }

    fn add(&mut self, name: &Name, value: Value) {
        match name.as_str() {
            "list-style-type" => self.list_style_type = Some(value),
            "list-style-position" => self.list_style_position = Some(value),
            "list-style-image" => self.list_style_image = Some(value),
            _ => {}
        }
    }
}

impl Display for ListShortHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = &self.list_style_type {
            write!(f, "{}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.list_style_position {
            write!(f, " {}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.list_style_image {
            write!(f, " {}", v.trim_end_matches("!important").trim())?;
        }
        if self.all_elements_has_important() {
            write!(f, "!important")?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
struct BackgroundShortHand {
    background_color: Option<Value>,
    background_image: Option<Value>,
    background_repeat: Option<Value>,
    background_attachment: Option<Value>,
    background_position: Option<Value>,
}

impl BackgroundShortHand {
    fn is_maybe_shorted(&self) -> bool {
        (self.background_color.is_some()
            || self.background_image.is_some()
            || self.background_repeat.is_some()
            || self.background_attachment.is_some()
            || self.background_position.is_some())
            && (self.all_elements_has_important() || self.no_one_element_has_no_important())
    }

    fn all_elements_has_important(&self) -> bool {
        if_some_has_important(self.background_color.as_ref())
            && if_some_has_important(self.background_image.as_ref())
            && if_some_has_important(self.background_repeat.as_ref())
            && if_some_has_important(self.background_attachment.as_ref())
            && if_some_has_important(self.background_position.as_ref())
    }

    fn no_one_element_has_no_important(&self) -> bool {
        !(none_or_has_important(self.background_color.as_ref())
            || none_or_has_important(self.background_image.as_ref())
            || none_or_has_important(self.background_repeat.as_ref())
            || none_or_has_important(self.background_attachment.as_ref())
            || none_or_has_important(self.background_position.as_ref()))
    }

    fn add(&mut self, name: &Name, value: Value) {
        match name.as_str() {
            "background-color" => self.background_color = Some(value),
            "background-image" => self.background_image = Some(value),
            "background-repeat" => self.background_repeat = Some(value),
            "background-attachment" => self.background_attachment = Some(value),
            "background-position" => self.background_position = Some(value),
            _ => {}
        }
    }
}

impl Display for BackgroundShortHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = &self.background_color {
            write!(f, " {}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.background_image {
            write!(f, " {}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.background_repeat {
            write!(f, " {}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.background_attachment {
            write!(f, "{}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.background_position {
            write!(f, " {}", v.trim_end_matches("!important").trim())?;
        }
        if self.all_elements_has_important() {
            write!(f, "!important")?;
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
struct BorderShortHand {
    border_width: Option<Value>,
    border_style: Option<Value>,
    border_color: Option<Value>,
}

impl BorderShortHand {
    fn is_maybe_shorted(&self) -> bool {
        (self.border_width.is_some() || self.border_style.is_some() || self.border_color.is_some())
            && (self.all_elements_has_important() || self.no_one_element_has_no_important())
    }

    fn all_elements_has_important(&self) -> bool {
        if_some_has_important(self.border_width.as_ref())
            && if_some_has_important(self.border_style.as_ref())
            && if_some_has_important(self.border_color.as_ref())
    }

    fn no_one_element_has_no_important(&self) -> bool {
        !(none_or_has_important(self.border_width.as_ref())
            || none_or_has_important(self.border_style.as_ref())
            || none_or_has_important(self.border_color.as_ref()))
    }

    fn add(&mut self, name: &Name, value: Value) {
        match name.as_str() {
            "border-width" => self.border_width = Some(value),
            "border-style" => self.border_style = Some(value),
            "border-color" => self.border_color = Some(value),
            _ => {}
        }
    }
}

impl Display for BorderShortHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = &self.border_width {
            write!(f, "{}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.border_style {
            write!(f, " {}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.border_color {
            write!(f, " {}", v.trim_end_matches("!important").trim())?;
        }
        if self.all_elements_has_important() {
            write!(f, "!important")?;
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
struct OutlineShortHand {
    outline_width: Option<Value>,
    outline_style: Option<Value>,
    outline_color: Option<Value>,
}

impl OutlineShortHand {
    fn is_maybe_shorted(&self) -> bool {
        (self.outline_width.is_some()
            || self.outline_style.is_some()
            || self.outline_color.is_some())
            && (self.all_elements_has_important() || self.no_one_element_has_no_important())
    }

    fn all_elements_has_important(&self) -> bool {
        if_some_has_important(self.outline_width.as_ref())
            && if_some_has_important(self.outline_style.as_ref())
            && if_some_has_important(self.outline_color.as_ref())
    }

    fn no_one_element_has_no_important(&self) -> bool {
        !(none_or_has_important(self.outline_width.as_ref())
            || none_or_has_important(self.outline_style.as_ref())
            || none_or_has_important(self.outline_color.as_ref()))
    }

    fn add(&mut self, name: &Name, value: Value) {
        match name.as_str() {
            "outline-width" => self.outline_width = Some(value),
            "outline-style" => self.outline_style = Some(value),
            "outline-color" => self.outline_color = Some(value),
            _ => {}
        }
    }
}

impl Display for OutlineShortHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = &self.outline_width {
            write!(f, "{}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.outline_style {
            write!(f, " {}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.outline_color {
            write!(f, " {}", v.trim_end_matches("!important").trim())?;
        }
        if self.all_elements_has_important() {
            write!(f, "!important")?;
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
struct TransitionShortHand {
    transition_property: Option<Value>,
    transition_duration: Option<Value>,
    transition_delay: Option<Value>,
    transition_timing_function: Option<Value>,
}

impl TransitionShortHand {
    fn is_maybe_shorted(&self) -> bool {
        (self.transition_property.is_some()
            || self.transition_duration.is_some()
            || self.transition_delay.is_some()
            || self.transition_timing_function.is_some())
            && (self.all_elements_has_important() || self.no_one_element_has_no_important())
    }

    fn all_elements_has_important(&self) -> bool {
        if_some_has_important(self.transition_property.as_ref())
            && if_some_has_important(self.transition_duration.as_ref())
            && if_some_has_important(self.transition_delay.as_ref())
            && if_some_has_important(self.transition_timing_function.as_ref())
    }

    fn no_one_element_has_no_important(&self) -> bool {
        !(none_or_has_important(self.transition_property.as_ref())
            || none_or_has_important(self.transition_duration.as_ref())
            || none_or_has_important(self.transition_delay.as_ref())
            || none_or_has_important(self.transition_timing_function.as_ref()))
    }

    fn add(&mut self, name: &Name, value: Value) {
        match name.as_str() {
            "transition-property" => self.transition_property = Some(value),
            "transition-duration" => self.transition_duration = Some(value),
            "transition-delay" => self.transition_delay = Some(value),
            "transition-timing-function" => self.transition_timing_function = Some(value),
            _ => {}
        }
    }
}

impl Display for TransitionShortHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = &self.transition_property {
            write!(f, "{}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.transition_duration {
            write!(f, " {}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.transition_delay {
            write!(f, " {}", v.trim_end_matches("!important").trim())?;
        }
        if let Some(v) = &self.transition_timing_function {
            write!(f, " {}", v.trim_end_matches("!important").trim())?;
        }
        if self.all_elements_has_important() {
            write!(f, "!important")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::optimizations::merge_shorthand::MergeShortHand;
    use crate::optimizations::transformer::Transform;
    use crate::structure::{Block, Parameters, Selectors};
    use indexmap::map::IndexMap;

    #[test]
    fn test_compress_font() {
        assert_eq!(
            MergeShortHand::default().transform(
                Block {
                    selectors: Selectors::default(),
                    parameters: {
                        let mut map = IndexMap::new();
                        map.insert("font-style".into(), "italic".into());
                        map.insert("font-weight".into(), "bold".into());
                        map.insert("font-size".into(), ".8em".into());
                        map.insert("line-height".into(), "1.2".into());
                        map.insert("font-family".into(), "Arial, sans-serif".into());
                        Parameters(map)
                    },
                }
                .into()
            ),
            Block {
                selectors: Selectors::default(),
                parameters: {
                    let mut map = IndexMap::new();
                    map.insert(
                        "font".into(),
                        "italic bold .8em/1.2 Arial, sans-serif".into(),
                    );
                    Parameters(map)
                },
            }
            .into()
        )
    }

    #[test]
    fn test_compress_background() {
        assert_eq!(
            MergeShortHand::default().transform(
                Block {
                    selectors: Selectors::default(),
                    parameters: {
                        let mut map = IndexMap::new();
                        map.insert("background-color".into(), "#000".into());
                        map.insert("background-image".into(), "url(images/bg.gif)".into());
                        map.insert("background-repeat".into(), "no-repeat".into());
                        map.insert("background-position".into(), "left top".into());
                        Parameters(map)
                    },
                }
                .into()
            ),
            Block {
                selectors: Selectors::default(),
                parameters: {
                    let mut map = IndexMap::new();
                    map.insert(
                        "background".into(),
                        "#000 url(images/bg.gif) no-repeat left top".into(),
                    );
                    Parameters(map)
                },
            }
            .into()
        )
    }

    #[test]
    fn test_background_important() {
        assert_eq!(
            MergeShortHand::default().transform(
                Block {
                    selectors: Selectors::default(),
                    parameters: {
                        let mut map = IndexMap::new();
                        map.insert("background-color".into(), "#000 !important".into());
                        Parameters(map)
                    },
                }
                .into()
            ),
            Block {
                selectors: Selectors::default(),
                parameters: {
                    let mut map = IndexMap::new();
                    map.insert("background".into(), "#000!important".into());
                    Parameters(map)
                },
            }
            .into()
        )
    }

    #[test]
    fn test_compress_border() {
        assert_eq!(
            MergeShortHand::default().transform(
                Block {
                    selectors: Selectors::default(),
                    parameters: {
                        let mut map = IndexMap::new();
                        map.insert("border-width".into(), "1px".into());
                        map.insert("border-style".into(), "solid".into());
                        map.insert("border-color".into(), "#000".into());
                        Parameters(map)
                    },
                }
                .into()
            ),
            Block {
                selectors: Selectors::default(),
                parameters: {
                    let mut map = IndexMap::new();
                    map.insert("border".into(), "1px solid #000".into());
                    Parameters(map)
                },
            }
            .into()
        )
    }

    #[test]
    fn test_compress_outline() {
        assert_eq!(
            MergeShortHand::default().transform(
                Block {
                    selectors: Selectors::default(),
                    parameters: {
                        let mut map = IndexMap::new();
                        map.insert("outline-width".into(), "1px".into());
                        map.insert("outline-style".into(), "solid".into());
                        map.insert("outline-color".into(), "#000".into());
                        Parameters(map)
                    },
                }
                .into()
            ),
            Block {
                selectors: Selectors::default(),
                parameters: {
                    let mut map = IndexMap::new();
                    map.insert("outline".into(), "1px solid #000".into());
                    Parameters(map)
                },
            }
            .into()
        )
    }
}
