use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::sequence::delimited;
use nom::IResult;

pub fn parse_header(input: &str) -> IResult<&str, &str> {
    let mut parser = delimited(tag("/*"), take_until("*/"), tag("*/"));
    let (input, header) = parser(input)?;
    Ok((input, header))
}
