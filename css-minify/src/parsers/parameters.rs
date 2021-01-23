use crate::parsers::useless::non_useless;
use crate::structure::{Name, Parameters, Value};
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::char;
use nom::combinator::{map, not};
use nom::lib::std::collections::HashMap;
use nom::multi::many0;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

pub fn parse_parameters(input: &str) -> IResult<&str, Parameters> {
    map(
        many0(non_useless(preceded(not(char('}')), parse_parameter))),
        |p| p.into_iter().collect::<HashMap<_, _>>().into(),
    )(input)
}

pub fn parse_parameter(input: &str) -> IResult<&str, (Name, Value)> {
    map(
        terminated(
            separated_pair(non_useless(is_not(":")), tag(":"), non_useless(is_not(";"))),
            tag(";"),
        ),
        |(name, value)| (name.trim().into(), value.trim().into()),
    )(input)
}

#[cfg(test)]
mod test {
    use crate::parsers::parameters::{parse_parameter, parse_parameters};
    use nom::lib::std::collections::HashMap;

    #[test]
    fn test_parameter() {
        assert_eq!(
            parse_parameter("margin-top: 32px;"),
            Ok(("", ("margin-top".into(), "32px".into())))
        )
    }

    #[test]
    fn test_parameters() {
        assert_eq!(
            parse_parameters(
                "
                margin-top: 32px;
                margin-bottom: 32px;
                margin-left: 32px; /* lazyefix */

                float: left; /* lazyefix */
                "
            ),
            Ok(("", {
                let mut tmp = HashMap::new();
                tmp.insert("margin-top".into(), "32px".into());
                tmp.insert("margin-bottom".into(), "32px".into());
                tmp.insert("margin-left".into(), "32px".into());
                tmp.insert("float".into(), "left".into());
                tmp.into()
            }))
        )
    }

    #[test]
    fn test_parameters_important() {
        assert_eq!(
            parse_parameters(
                "
                background-color: #f64e60 !important; 
                "
            ),
            Ok(("", {
                let mut tmp = HashMap::new();
                tmp.insert("background-color".into(), "#f64e60 !important".into());
                tmp.into()
            }))
        )
    }
}