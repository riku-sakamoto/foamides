use std::collections::HashMap;

use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

use itertools::Itertools;

use crate::parsers::boundary;
use crate::parsers::common;
use crate::parsers::dimensions;
use crate::parsers::foamfile;
use crate::parsers::header;

pub struct BoundaryHolder {
    pub target: String,
    pub boundary_field: boundary::BoundaryField,
    pub dimensions: dimensions::Dimensions,
    pub others: HashMap<String, String>,
}

fn _parse_boundary_file(
    input: &str,
) -> IResult<
    &str,
    (
        &str,
        foamfile::FoamFile,
        Vec<(&str, &str)>,
        boundary::BoundaryField,
    ),
> {
    let (input, (header, foamfile, others, boundary_field)) = tuple((
        header::parse_header,
        foamfile::parse_foamfile,
        many0(common::parse_key_value),
        boundary::parse_boundary_field,
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
    let dims = dimensions::parse_dimensions(strdim.1);

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
