use nom::bytes::complete::{is_not, tag, take_until};
use nom::character::complete::{alphanumeric1, multispace0};
use nom::combinator::{opt, rest};
use nom::multi::many0;
use nom::sequence::{terminated, tuple};
use nom::{Err, IResult};

fn parse_line_without_comment(input: &str) -> IResult<&str, &str> {
    // extract line
    let (input, (content, _)) = tuple((take_until("\n"), tag("\n")))(input)?;

    let (content, with_comment) = opt(take_until("//"))(content)?;

    match with_comment {
        Some(value) => {
            return Ok((input, value));
        }
        None => {
            return Ok((input, content));
        }
    }
}

pub fn trim_comments(input: &str) -> String {
    let mut parser = many0(parse_line_without_comment);
    let (_, parsed) = parser(input).unwrap();

    parsed.join("\n")
}

pub fn parse_key_value(input: &str) -> IResult<&str, (&str, &str)> {
    let mut parser = tuple((multispace0, terminated(is_not(";\n"), tag(";"))));
    let (input, (_, result)) = parser(input)?;

    let (_, (_, key, value)) = tuple((multispace0, alphanumeric1, rest))(result)?;
    let value = value.trim();

    Ok((input, (key, value)))
}
