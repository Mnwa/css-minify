use crate::parsers::block::parse_blocks;
use derive_more::{From, Into};
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

pub fn minify(input: &str) -> MResult {
    parse_blocks(input)
        .map_err(MError::from)
        .map(|(other, blocks)| (other, format!("{}", blocks)))
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
    use crate::optimizations::minify;

    #[test]
    fn test_minify() {
        assert_eq!(
            minify(
                r#"
                #some_id, input {
                    padding: 5px 3px; /* Mega comment */
                    color: white;
                }
                
                
                /* this is are test id */
                #some_id_2, .class {
                    padding: 5px 4px; /* Mega comment */
                    color: black;
                }
            "#
            ),
            Ok(("", "#some_id,input{color:white;padding:5px 3px;}#some_id_2,.class{color:black;padding:5px 4px;}".into()))
        )
    }
}
