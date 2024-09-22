use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, multispace0, space1};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;

use std::collections::HashMap;
use std::hash::Hash;

#[path = "./common.rs"]
mod common;

#[derive(Debug)]
pub struct FoamFile {
    pub version: String,
    pub format: String,
    pub class: String,
    pub object: String,
}

pub fn parse_foamfile(input: &str) -> IResult<&str, FoamFile> {
    let mut parser = tuple((
        multispace0,
        tag("FoamFile"),
        multispace0,
        delimited(tag("{"), take_until("}"), tag("}")),
    ));
    let (input, (_, _, _, content)) = parser(input)?;

    let (_, key_values) = many0(common::parse_key_value)(input)?;

    let map: HashMap<&str, &str> = key_values.into_iter().collect();
    let foamfile = FoamFile {
        version: map.get("version").map_or("".to_string(), |s| s.to_string()),
        format: map.get("format").map_or("".to_string(), |s| s.to_string()),
        class: map.get("class").map_or("".to_string(), |s| s.to_string()),
        object: map.get("object").map_or("".to_string(), |s| s.to_string()),
    };

    Ok((input, foamfile))
}
