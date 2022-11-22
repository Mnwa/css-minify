use crate::parsers::utils::{is_not_block_ending, non_useless};
use crate::structure::{PseudoClass, Selector, SelectorWithPseudoClasses, Selectors};
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::multi::{many0, many_m_n, separated_list1};
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;

pub fn parse_selectors(input: &str) -> IResult<&str, Selectors> {
    map(
        is_not_block_ending(separated_list1(
            char(','),
            non_useless(parse_selector_with_pseudo_class),
        )),
        |selectors| selectors.into(),
    )(input)
}

pub fn parse_selector_with_pseudo_class(input: &str) -> IResult<&str, SelectorWithPseudoClasses> {
    map(
        pair(
            non_useless(opt(parse_selector)),
            non_useless(parse_pseudo_classes),
        ),
        |(selector, pc)| SelectorWithPseudoClasses(selector, pc),
    )(input)
}

pub fn parse_pseudo_classes(input: &str) -> IResult<&str, Vec<PseudoClass>> {
    many0(parse_pseudo_class)(input)
}

pub fn parse_pseudo_class(input: &str) -> IResult<&str, PseudoClass> {
    map(
        pair(
            pair(
                non_useless(parse_pseudo_class_name),
                non_useless(opt(parse_pseudo_class_params)),
            ),
            map(non_useless(opt(is_not(",{:"))), |i: Option<&str>| {
                i.map(|i| i.to_string())
            }),
        ),
        |((name, params), next)| PseudoClass { name, params, next },
    )(input)
}

pub fn parse_pseudo_class_name(input: &str) -> IResult<&str, String> {
    map(
        preceded(many_m_n(1, 2, char(':')), is_not("(,{:")),
        |i: &str| i.to_string(),
    )(input)
}

pub fn parse_pseudo_class_params(input: &str) -> IResult<&str, String> {
    map(delimited(char('('), is_not(")"), char(')')), |i: &str| {
        i.to_string()
    })(input)
}

pub fn parse_selector(input: &str) -> IResult<&str, Selector> {
    alt((parse_id, parse_class, parse_tag))(input)
}

pub fn parse_id(input: &str) -> IResult<&str, Selector> {
    map(preceded(char('#'), is_not(",{:")), |i: &str| {
        Selector::Id(i.trim().into())
    })(input)
}

pub fn parse_class(input: &str) -> IResult<&str, Selector> {
    map(preceded(char('.'), is_not(",{:")), |i: &str| {
        Selector::Class(i.trim().into())
    })(input)
}

pub fn parse_tag(input: &str) -> IResult<&str, Selector> {
    map(is_not(",{:"), |i: &str| Selector::Tag(i.trim().into()))(input)
}

#[cfg(test)]
mod test {
    use crate::parsers::selector::{parse_selector, parse_selectors};
    use crate::structure::{PseudoClass, Selector, SelectorWithPseudoClasses};

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
                    SelectorWithPseudoClasses(Some(Selector::Id("some_id".into())), vec![]),
                    SelectorWithPseudoClasses(Some(Selector::Class("some_class".into())), vec![]),
                    SelectorWithPseudoClasses(Some(Selector::Tag("input".into())), vec![]),
                ]
                .into()
            ))
        );
    }

    #[test]
    fn test_selectors_with_pc_without_params() {
        assert_eq!(
            parse_selectors("#some_id:only-child"),
            Ok((
                "",
                vec![SelectorWithPseudoClasses(
                    Some(Selector::Id("some_id".into())),
                    vec![PseudoClass {
                        name: "only-child".to_string(),
                        params: None,
                        next: None,
                    }]
                ),]
                .into()
            ))
        );
    }

    #[test]
    fn test_selectors_with_pc_with_params() {
        assert_eq!(
            parse_selectors("#some_id:nth-child(4n)"),
            Ok((
                "",
                vec![SelectorWithPseudoClasses(
                    Some(Selector::Id("some_id".into())),
                    vec![PseudoClass {
                        name: "nth-child".to_string(),
                        params: Some("4n".to_string()),
                        next: None,
                    }]
                ),]
                .into()
            ))
        );
    }

    #[test]
    fn test_pc_without_selector() {
        assert_eq!(
            parse_selectors(":is(nav, .posts)"),
            Ok((
                "",
                vec![SelectorWithPseudoClasses(
                    None,
                    vec![PseudoClass {
                        name: "is".to_string(),
                        params: Some("nav, .posts".to_string()),
                        next: None,
                    }]
                ),]
                .into()
            ))
        );
    }

    #[test]
    fn test_pc_selector() {
        assert_eq!(
            parse_selectors(":is(.test) a"),
            Ok((
                "",
                vec![SelectorWithPseudoClasses(
                    None,
                    vec![PseudoClass {
                        name: "is".to_string(),
                        params: Some(".test".to_string()),
                        next: Some("a".to_string()),
                    }]
                ),]
                .into()
            ))
        );
    }

    #[test]
    fn test_pc_double_dots() {
        assert_eq!(
            parse_selectors("::is(.test) a"),
            Ok((
                "",
                vec![SelectorWithPseudoClasses(
                    None,
                    vec![PseudoClass {
                        name: "is".to_string(),
                        params: Some(".test".to_string()),
                        next: Some("a".to_string()),
                    }]
                ),]
                .into()
            ))
        );
    }

    #[test]
    fn test_pc_nulti() {
        assert_eq!(
            parse_selectors("a:not([href]):not([tabindex])"),
            Ok((
                "",
                vec![SelectorWithPseudoClasses(
                    Some(Selector::Tag("a".into())),
                    vec![
                        PseudoClass {
                            name: "not".to_string(),
                            params: Some("[href]".to_string()),
                            next: None,
                        },
                        PseudoClass {
                            name: "not".to_string(),
                            params: Some("[tabindex]".to_string()),
                            next: None,
                        }
                    ]
                ),]
                .into()
            ))
        );
    }
}
