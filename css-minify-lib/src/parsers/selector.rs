use crate::parsers::useless::non_useless;
use crate::structure::{Selector, Selectors};
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::char;
use nom::combinator::{map, not};
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;

pub fn parse_selectors(input: &str) -> IResult<&str, Selectors> {
    map(
        preceded(
            not(alt((char('@'), char('{'), char('}')))),
            separated_list1(char(','), non_useless(parse_selector)),
        ),
        |selectors| selectors.into(),
    )(input)
}

pub fn parse_selector(input: &str) -> IResult<&str, Selector> {
    alt((parse_id, parse_class, parse_tag))(input)
}

pub fn parse_id(input: &str) -> IResult<&str, Selector> {
    map(preceded(char('#'), is_not(",{")), |i: &str| {
        Selector::Id(i.trim().into())
    })(input)
}

pub fn parse_class(input: &str) -> IResult<&str, Selector> {
    map(preceded(char('.'), is_not(",{")), |i: &str| {
        Selector::Class(i.trim().into())
    })(input)
}

pub fn parse_tag(input: &str) -> IResult<&str, Selector> {
    map(is_not(",{"), |i: &str| Selector::Tag(i.trim().into()))(input)
}

#[cfg(test)]
mod test {
    use crate::parsers::selector::{parse_selector, parse_selectors};
    use crate::structure::Selector;

    #[test]
    fn test_selector() {
        assert_eq!(
            parse_selector("#some_id"),
            Ok(("", Selector::Id("some_id".into())))
        );
        assert_eq!(
            parse_selector(".some_class"),
            Ok(("", Selector::Class("some_class".into())))
        );
        assert_eq!(
            parse_selector("input"),
            Ok(("", Selector::Tag("input".into())))
        );
    }

    #[test]
    fn test_selectors() {
        assert_eq!(
            parse_selectors("#some_id, .some_class, input"),
            Ok((
                "",
                vec![
                    Selector::Id("some_id".into()),
                    Selector::Class("some_class".into()),
                    Selector::Tag("input".into())
                ]
                .into()
            ))
        );
    }
}
