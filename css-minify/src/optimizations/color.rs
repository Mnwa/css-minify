use crate::parsers::utils::non_useless;
use derive_more::{Deref, DerefMut, From, Into};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, hex_digit1};
use nom::combinator::{map, recognize};
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

#[derive(Clone, Eq, PartialEq, Default, Debug, Deref, DerefMut, From, Into)]
pub struct Color(String);

pub fn optimize_color(input: &str) -> Color {
    let color = alt((parse_hex, parse_rgb))(input)
        .map(|(postfix, mut color)| {
            if postfix.trim() == "!important" {
                color.0 += "!important"
            }
            color
        })
        .unwrap_or_else(|_| Color(input.into()));
    color
}

pub fn parse_hex(input: &str) -> IResult<&str, Color> {
    map(recognize(preceded(tag("#"), hex_digit1)), |color: &str| {
        if color[1..4] == color[4..] {
            return Color((&color[..4]).to_lowercase());
        }
        Color(color.to_lowercase())
    })(input)
}

pub fn parse_rgb(input: &str) -> IResult<&str, Color> {
    terminated(
        preceded(
            terminated(non_useless(tag("rgb")), char('(')),
            map(
                tuple((
                    terminated(non_useless(digit1), char(',')),
                    terminated(non_useless(digit1), char(',')),
                    non_useless(digit1),
                )),
                |(red, green, blue): (&str, &str, &str)| {
                    let color = format!(
                        "#{:02X}{:02X}{:02X}",
                        u8::from_str_radix(red, 10).unwrap(),
                        u8::from_str_radix(green, 10).unwrap(),
                        u8::from_str_radix(blue, 10).unwrap(),
                    );

                    if color[1..4] == color[4..] {
                        return Color((&color[..4]).to_lowercase());
                    }
                    Color(color.to_lowercase())
                },
            ),
        ),
        tag(")"),
    )(input)
}

#[cfg(test)]
mod test {
    use crate::optimizations::color::{parse_hex, parse_rgb, Color};

    #[test]
    fn test_rgb() {
        assert_eq!(
            parse_rgb("rgb(255, 255, /* lol */ 255 )"),
            Ok(("", Color("#fff".into())))
        )
    }
    #[test]
    fn test_small_rgb() {
        assert_eq!(
            parse_rgb("rgb(4, 120, 87)"),
            Ok(("", Color("#047857".into())))
        )
    }
    #[test]
    fn test_hex() {
        assert_eq!(parse_hex("#fff"), Ok(("", Color("#fff".into()))))
    }
}
