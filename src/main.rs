mod parsers {
    pub mod boundary;
    pub mod common;
}

mod boundary_file;
mod utils;

fn main() {
    // let path = "data/demo.txt";
    // let path = "data/openfoam-OpenFOAM-v2012-tutorials-incompressible-simpleFoam-pitzDaily/tutorials/incompressible/simpleFoam/pitzDaily/0/U";
    let path = "data/openfoam/v2012/tutorials/incompressible/simpleFoam/pitzDaily/0/p";
    let contents = utils::read_file_contents(path);

    let file_content = contents.unwrap();
    let file_content = parsers::common::trim_comments(&file_content);

    let result = boundary_file::parse_boundary_file(&file_content, "p");

    println!("target: {}", result.target);

    for (key, value) in result.boundary_field.boundaries {
        println!("{}, {:?}", key, value);
    }

    println!("{:?}", result.dimensions);

    // let result = parsers::boundary::parse_boundary_field(&file_content).unwrap();

    // for (key, value) in result.1.boundaries{
    //     println!("{}, {:?}", key, value);
    // }
}
