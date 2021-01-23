mod color;
mod media;
mod merge_m_n_p;
mod merge_shorthand;
mod transformer;

use crate::optimizations::color::optimize_color;
use crate::optimizations::media::MediaOptimizer;
use crate::optimizations::merge_m_n_p::Merge;
use crate::optimizations::merge_shorthand::MergeShortHand;
use crate::optimizations::transformer::{Transform, Transformer, TransformerParameterFn};
use crate::parsers::css_entity::parse_css;
use crate::structure::Value;
use derive_more::{From, Into};
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

pub struct Minifier {
    transformer: Transformer,
    merge_m_n_p: Merge,
    merge_shorthand: MergeShortHand,
    media: MediaOptimizer,
}

impl Minifier {
    pub fn minify<'a>(&mut self, input: &'a str) -> MResult<'a> {
        parse_css(input)
            .map_err(MError::from)
            .map(|(other, blocks)| (other, self.transformer.transform_many(blocks)))
            .map(|(other, blocks)| (other, self.merge_m_n_p.transform_many(blocks)))
            .map(|(other, blocks)| (other, self.merge_shorthand.transform_many(blocks)))
            .map(|(other, blocks)| (other, self.media.transform_many(blocks)))
            .map(|(other, blocks)| (other, blocks.to_string()))
    }
}

impl Default for Minifier {
    fn default() -> Self {
        let mut transformer = Transformer::default();
        transformer.register_parameter(TransformerParameterFn::Value(Box::new(|value| {
            optimize_color(&value).into()
        })));
        transformer.register_parameter(TransformerParameterFn::Value(Box::new(|mut value| {
            if value.starts_with("0px") {
                value = format!("0{}", value.trim_start_matches("0px"))
            }
            if value.starts_with("0rem") {
                value = format!("0{}", value.trim_start_matches("0rem"))
            }
            if value.starts_with("0.") {
                value = format!(".{}", value.trim_start_matches("0."))
            }
            value
                .replace(" 0px", " 0")
                .replace(" 0rem", " 0")
                .replace(" 0.", " .")
                .replace(", ", ",")
                .replace(" !important", "!important")
        })));

        transformer.register_parameter(TransformerParameterFn::Name(Box::new(|name| {
            name.to_lowercase()
        })));

        let merge_m_n_p = Merge::default();
        let merge_shorthand = MergeShortHand::default();
        let media = MediaOptimizer::default();

        Minifier {
            merge_m_n_p,
            merge_shorthand,
            transformer,
            media,
        }
    }
}

pub type MResult<'a> = Result<(&'a str, String), MError<'a>>;

#[derive(Debug, From, Into, PartialEq)]
pub struct MError<'a>(nom::Err<nom::error::Error<&'a str>>);

impl Display for MError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid css")
    }
}

impl<'a> Error for MError<'a> {}

#[inline]
pub(crate) fn if_some_has_important(input: Option<&Value>) -> bool {
    if let Some(input) = input {
        return input.ends_with("!important");
    }
    true
}

#[inline]
pub(crate) fn none_or_has_important(input: Option<&Value>) -> bool {
    if let Some(input) = input {
        return input.ends_with("!important");
    }
    false
}

#[cfg(test)]
mod test {
    use crate::optimizations::Minifier;

    #[test]
    fn test_minify() {
        assert_eq!(
            Minifier::default().minify(
                r#"
                #some_id, input {
                    padding: 5px 3px; /* Mega comment */
                    color: white;
                }
                
                
                /* this is are test id */
                #some_id_2, .class {
                    padding: 5px 4px; /* Mega comment */
                    Color: rgb(255, 255, 255);
                }
            "#
            ),
            Ok(("", "#some_id,input{color:white;padding:5px 3px}#some_id_2,.class{color:#fff;padding:5px 4px}".into()))
        )
    }
}
