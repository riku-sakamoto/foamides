use foamides;

#[test]
fn test_u_boundary() {
    let path = "data/openfoam/v2012/tutorials/incompressible/simpleFoam/pitzDaily/0/U";
    let contents = foamides::utils::read_file_contents(path);

    let file_content = contents.unwrap();
    let file_content = foamides::parsers::common::trim_comments(&file_content);

    let result = foamides::boundary_file::parse_boundary_file(&file_content, "U");

    assert_eq!(result.target, "U");

    assert_eq!(
        result
            .boundary_field
            .boundaries
            .get("inlet")
            .unwrap()
            .boundary_type,
        "fixedValue"
    );
    assert_eq!(
        result.boundary_field.boundaries.get("inlet").unwrap().value,
        Some("10 0 0".to_string())
    );
    assert_eq!(
        result
            .boundary_field
            .boundaries
            .get("lowerWall")
            .unwrap()
            .boundary_type,
        "noSlip"
    )
}

// #[test]
// fn test_p_boundary() {
//     let path = "data/openfoam/v2012/tutorials/incompressible/simpleFoam/pitzDaily/0/p";
//     let contents = foamides::utils::read_file_contents(path);

//     let file_content = contents.unwrap();
//     let file_content = foamides::parsers::common::trim_comments(&file_content);

//     let result = foamides::boundary_file::parse_boundary_file(&file_content, "p");

//     assert_eq!(result.target, "p");

//     assert_eq!(
//         result
//             .boundary_field
//             .boundaries
//             .get("outlet")
//             .unwrap()
//             .boundary_type,
//         "fixedValue"
//     );
//     // assert_eq!(
//     //     result
//     //         .boundary_field
//     //         .boundaries
//     //         .get("outlet")
//     //         .unwrap()
//     //         .value,
//     //     Some("0".to_string())
//     // );

//     assert_eq!(
//         result
//             .boundary_field
//             .boundaries
//             .get("inlet")
//             .unwrap()
//             .boundary_type,
//         "zeroGradient"
//     );
// }
