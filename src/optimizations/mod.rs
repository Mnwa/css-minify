mod color;
mod merge_m_n_p;
mod transformer;

use crate::optimizations::color::optimize_color;
use crate::optimizations::transformer::{Transform, Transformer, TransformerParameterFn};
use crate::parsers::block::parse_blocks;
use derive_more::{From, Into};
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

pub struct Minifier {
    transformer: Transformer,
}

impl Minifier {
    pub fn minify<'a>(&mut self, input: &'a str) -> MResult<'a> {
        parse_blocks(input)
            .map_err(MError::from)
            .map(|(other, blocks)| (other, self.transformer.transform_many(blocks)))
            .map(|(other, blocks)| (other, blocks.to_string()))
    }
}

impl Default for Minifier {
    fn default() -> Self {
        let mut transformer = Transformer::default();
        transformer.register_parameter(TransformerParameterFn::Value(Box::new(|value| {
            optimize_color(&value).into()
        })));

        transformer.register_parameter(TransformerParameterFn::Name(Box::new(|name| {
            name.to_lowercase()
        })));

        Minifier { transformer }
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
            Ok(("", "#some_id,input{color:white;padding:5px 3px}#some_id_2,.class{color:#FFF;padding:5px 4px}".into()))
        )
    }
}
