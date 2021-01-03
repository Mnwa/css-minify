use crate::parsers::block::parse_block;
use crate::parsers::media::parse_media;
use crate::parsers::useless::non_useless;
use crate::structure::{CssEntities, CssEntity, Value};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{preceded, terminated};
use nom::IResult;

pub fn parse_css(input: &str) -> IResult<&str, CssEntities> {
    map(non_useless(many0(parse_entity)), |css| css.into())(input)
}

pub fn parse_entity(input: &str) -> IResult<&str, CssEntity> {
    alt((
        map(parse_media, |media| CssEntity::Media(media)),
        map(parse_charset, |charset| CssEntity::Charset(charset)),
        map(parse_block, |block| CssEntity::Block(block)),
    ))(input)
}

pub fn parse_charset(input: &str) -> IResult<&str, Value> {
    map(
        preceded(
            tag("@charset"),
            terminated(non_useless(is_not(";")), tag(";")),
        ),
        |s| Value::from(s),
    )(input)
}

mod test {
    use crate::parsers::css_entity::parse_charset;

    #[test]
    fn test_charset() {
        assert_eq!(
            parse_charset("@charset \"UTF-8\";"),
            Ok(("", "\"UTF-8\"".into()))
        )
    }
}
