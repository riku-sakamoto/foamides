use std::collections::HashMap;

use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

#[path = "./parsers"]
mod parsers {
    pub mod boundary;
    pub mod common;
    pub mod dimensions;
    pub mod foamfile;
    pub mod header;
}

#[path = "./utils.rs"]
mod utils;

use self::parsers::boundary::BoundaryField;
use self::parsers::foamfile::FoamFile;
use itertools::Itertools;

pub struct BoundaryHolder {
    pub target: String,
    pub boundary_field: BoundaryField,
    pub dimensions: parsers::dimensions::Dimensions,
    pub others: HashMap<String, String>,
}

fn _parse_boundary_file(
    input: &str,
) -> IResult<&str, (&str, FoamFile, Vec<(&str, &str)>, BoundaryField)> {
    let (input, (header, foamfile, others, boundary_field)) = tuple((
        parsers::header::parse_header,
        parsers::foamfile::parse_foamfile,
        many0(parsers::common::parse_key_value),
        parsers::boundary::parse_boundary_field,
    ))(input)?;

    Ok((input, (header, foamfile, others, boundary_field)))
}

pub fn parse_boundary_file(input: &str, target_name: &str) -> BoundaryHolder {
    let result = _parse_boundary_file(input).unwrap().1;

    let strdim = result
        .2
        .iter()
        .filter(|v| v.0 == "dimensions")
        .exactly_one()
        .unwrap();
    let dims = parsers::dimensions::parse_dimensions(strdim.1);

    BoundaryHolder {
        target: target_name.to_string(),
        boundary_field: result.3,
        dimensions: dims,
        others: result
            .2
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
    }
}
