mod color;
mod font;
mod merge_blocks;
mod merge_m_n_p;
mod merge_media;
mod merge_shorthand;
mod transformer;

use crate::optimizations::color::optimize_color;
use crate::optimizations::font::FontTransformer;
use crate::optimizations::merge_blocks::MergeBlocks;
use crate::optimizations::merge_m_n_p::Merge;
use crate::optimizations::merge_media::MergeMedia;
use crate::optimizations::merge_shorthand::MergeShortHand;
use crate::optimizations::transformer::{Transform, Transformer, TransformerParameterFn};
use crate::parsers::css_entity::parse_css;
use crate::structure::Value;
use derive_more::{From, Into};
use nom::lib::std::fmt::Debug;
use nom::lib::std::str::FromStr;
use nom::{Err, Needed};
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

/// Struct which stores all optimizations from css minify lib
pub struct Minifier {
    transformer: Transformer,
    merge_m_n_p: Merge,
    merge_shorthand: MergeShortHand,
    media: MergeMedia,
    blocks: MergeBlocks,
    font: FontTransformer,
}

impl Minifier {
    /// Minify css input and return result with minified css string
    pub fn minify<'a>(&mut self, input: &'a str, level: Level) -> MResult<'a> {
        let mut result = parse_css(input)
            .map(|(_, blocks)| blocks)
            .map_err(|e| MError(input, e));

        if level == Level::Three {
            result = result
                .map(|blocks| self.blocks.transform_many(blocks))
                .map(|blocks| self.media.transform_many(blocks))
        }

        if level >= Level::Two {
            result = result
                .map(|blocks| self.merge_m_n_p.transform_many(blocks))
                .map(|blocks| self.merge_shorthand.transform_many(blocks))
        }

        if level >= Level::One {
            result = result
                .map(|blocks| self.transformer.transform_many(blocks))
                .map(|blocks| self.font.transform_many(blocks))
        }

        result.map(|blocks| blocks.to_string())
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
        let media = MergeMedia::default();
        let blocks = MergeBlocks::default();
        let font = FontTransformer::default();

        Minifier {
            merge_m_n_p,
            merge_shorthand,
            transformer,
            media,
            blocks,
            font,
        }
    }
}

/// Transforming level
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Level {
    /// Disable transformer
    Zero = 0,
    /// Remove whitespaces, replace `0.` to `.` and others non dangerous optimizations
    /// It's default level
    One = 1,
    /// Level One + shortcuts (margins, paddings, backgrounds and etc)
    /// In mostly cases it's non dangerous optimizations, but be careful
    Two = 2,
    /// Level Two + merge @media and css blocks with equal screen/selectors
    /// It is a danger optimizations, because ordering of your css code may be changed
    Three = 3,
}

impl Default for Level {
    fn default() -> Self {
        Self::One
    }
}

impl FromStr for Level {
    type Err = ParseLevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Level::Zero),
            "1" => Ok(Level::One),
            "2" => Ok(Level::Two),
            "3" => Ok(Level::Three),
            _ => Err(ParseLevelError),
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct ParseLevelError;

impl Display for ParseLevelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Input must be number from 0 to 3 values")
    }
}

impl Debug for ParseLevelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParseLevelError")
            .field(
                "message",
                &"Input must be number from 0 to 3 values".to_string(),
            )
            .finish()
    }
}

impl Error for ParseLevelError {}

pub type MResult<'a> = Result<String, MError<'a>>;

#[derive(From, Into, PartialEq)]
pub struct MError<'a>(&'a str, nom::Err<nom::error::Error<&'a str>>);

impl Debug for MError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MError")
            .field("message", &format!("{}", self))
            .field("error", &format!("{:?}", self.0))
            .finish()
    }
}

impl Display for MError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let unparsed_size = match &self.1 {
            Err::Incomplete(n) => match n {
                Needed::Unknown => None,
                Needed::Size(s) => Some(s.get()),
            },
            Err::Error(e) => Some(e.input.len()),
            Err::Failure(e) => Some(e.input.len()),
        };
        if let Some(size) = unparsed_size {
            let parsed_css = &self.0[..self.0.len() - size];

            write!(
                f,
                "Invalid block at line {}",
                parsed_css.lines().count() + 1
            )
        } else {
            write!(f, "Invalid css")
        }
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
    use crate::optimizations::{Level, Minifier};

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
                    font-weight: bold;
                }
            "#,
                    Level::Three
            ),
            Ok("#some_id,input{padding:5px 3px;color:white}#some_id_2,.class{padding:5px 4px;color:#fff;font-weight:700}".into())
        )
    }

    #[test]
    fn test_minify_invalid_css() {
        assert_eq!(
            Minifier::default()
                .minify(
                    r#"
                #some_id, input {
                    padding: 5px 3px; /* Mega comment */
                    color: white;
                }} /* sasd */
                
                
                /* this is are test id */
                #some_id_2, .class {
                    padding: 5px 4px; /* Mega comment */
                    Color: rgb(255, 255, 255);
                }
            "#,
                    Level::Three
                )
                .unwrap_err()
                .to_string(),
            "Invalid block at line 6"
        )
    }
}
