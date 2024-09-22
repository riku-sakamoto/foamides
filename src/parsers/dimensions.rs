use nom::sequence::delimited;
use nom::IResult;
use nom::{
    bytes::complete::{tag, take_until},
    sequence::tuple,
};

#[derive(Debug)]
pub struct Dimensions {
    pub value: [f32; 7],
}

fn parse_dimension_list(value: &str) -> IResult<&str, &str> {
    let mut parser = delimited(tag("["), take_until("]"), tag("]"));
    let (left, parsed) = parser(value)?;

    Ok((left, parsed))
}

pub fn parse_dimensions(value: &str) -> Dimensions {
    let result = parse_dimension_list(value);
    let parsed = result.unwrap().1;

    let v: Vec<_> = parsed.split(" ").collect();
    let v: Vec<f32> = v.into_iter().map(|k| k.parse().unwrap()).collect();

    let v: [f32; 7] = v
        .try_into()
        .map_err(|_| "Dimension size is not equal to 7.")
        .unwrap();

    Dimensions { value: v }
}
