use crate::parsers::at::{
    parse_at, parse_font_face, parse_keyframes, parse_media, parse_page, parse_supports,
    parse_viewport,
};
use crate::parsers::block::parse_block;
use crate::parsers::utils::non_useless;
use crate::structure::{CssEntities, CssEntity};
use nom::branch::alt;
use nom::combinator::{all_consuming, into, map};
use nom::multi::many0;
use nom::IResult;

pub fn parse_css(input: &str) -> IResult<&str, CssEntities> {
    all_consuming(parse_entities)(input)
}

pub fn parse_entities(input: &str) -> IResult<&str, CssEntities> {
    map(non_useless(many0(parse_entity)), |css| css.into())(input)
}

pub fn parse_entity(input: &str) -> IResult<&str, CssEntity> {
    alt((
        into(parse_media),
        into(parse_at),
        into(parse_keyframes),
        into(parse_font_face),
        into(parse_viewport),
        into(parse_supports),
        into(parse_page),
        into(parse_block),
    ))(input)
}
