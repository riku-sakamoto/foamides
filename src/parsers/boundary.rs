use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, multispace0, space1};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;

use std::collections::HashMap;

#[derive(Debug)]
pub struct BoundaryCondition {
    pub boundary_type: String,
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct BoundaryField {
    pub boundaries: HashMap<String, BoundaryCondition>,
}

fn parse_type(input: &str) -> IResult<&str, &str> {
    // type fixedValue;

    let parser = tuple((tag("type"), space1, alpha1, tag(";")));
    map(parser, |(_, _, type_name, _)| type_name)(input)
}

fn parse_uniform_value(input: &str) -> IResult<&str, &str> {
    // value uniform 1.0;

    if !(input.starts_with("value")) {
        return Ok((input, ""));
    }

    let (_, input) = delimited(tag("value"), take_until(";"), tag(";"))(input)?;
    let parser = tuple((
        multispace0,
        tag("uniform"),
        space1,
        delimited(tag("("), take_until(")"), tag(")")),
    ));
    map(parser, |(_, _, _, v)| v)(input)
}

fn parse_boundary_condition(input: &str) -> IResult<&str, (String, BoundaryCondition)> {
    let (input, (_, boundary_name, _, content)) = tuple((
        multispace0,
        alpha1,
        multispace0,
        delimited(tag("{"), take_until("}"), tag("}")),
    ))(input)?;

    let (_, (_, boundary_type, _, uniform_value)) =
        tuple((multispace0, parse_type, multispace0, parse_uniform_value))(content)?;

    Ok((
        input,
        (
            boundary_name.to_string(),
            BoundaryCondition {
                boundary_type: boundary_type.to_string(),
                value: Some(uniform_value.to_string()),
            },
        ),
    ))
}

pub fn parse_boundary_field(input: &str) -> IResult<&str, BoundaryField> {
    let (input, (_, _, _, _, boundaries)) = tuple((
        multispace0,
        tag("boundaryField"),
        multispace0,
        tag("{"),
        many0(parse_boundary_condition),
    ))(input)?;

    let boundaries_map = boundaries.into_iter().collect();
    Ok((
        input,
        BoundaryField {
            boundaries: boundaries_map,
        },
    ))
}
