use gs_rs::optimizer::optimize;
use gs_rs::parser::json::JsonParser;
use gs_rs::parser::Parser;

fn main() {
    // parse json file containing 3D variables and odometries to internal factor graph representation
    let factor_graph = JsonParser::parse_file("examples/io_files/Sphere_3D.json").unwrap();

    // optimize the factor graph's variables with 10 iterations
    optimize(&factor_graph, 10);

    // compose json file containing optimized 3D variables and unchanged odometries
    JsonParser::compose_file(&factor_graph, "examples/io_files/Sphere_3D_optimized.json").unwrap();
}
