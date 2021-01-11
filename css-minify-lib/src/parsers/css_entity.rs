use crate::parsers::at::{parse_at, parse_media};
use crate::parsers::block::parse_block;
use crate::parsers::useless::non_useless;
use crate::structure::{CssEntities, CssEntity};
use nom::branch::alt;
use nom::combinator::{into, map};
use nom::multi::many0;
use nom::IResult;

pub fn parse_css(input: &str) -> IResult<&str, CssEntities> {
    map(non_useless(many0(parse_entity)), |css| css.into())(input)
}

pub fn parse_entity(input: &str) -> IResult<&str, CssEntity> {
    alt((into(parse_media), into(parse_at), into(parse_block)))(input)
}
