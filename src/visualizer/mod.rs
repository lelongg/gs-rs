//! Handles the graphical user interface.

use std::ops::Index;

use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use kiss3d::camera::ArcBall;
use nalgebra::{Point3, Rotation3, Translation3, UnitQuaternion, Vector3};
use petgraph::visit::EdgeRef;

use crate::factor_graph::factor::{Factor, FactorType::*};
use crate::factor_graph::variable::VariableType::*;
use crate::factor_graph::FactorGraph;

struct VisualFactorGraph {
    scene_node: SceneNode,
    lines: Vec<[Point3<f32>; 3]>,
}

// TODO does not work multiple times in a single execution of the program
// Things tried so far:
// - Google -> nobody seems to have mentioned this problem before
// - make window static -> causes problems as it has to be mutable and can not be moved
// Possible solutions:
// - do not tackle this problem, but instead support being able to visualize several factor graphs and jump between them
/// Displays the visualization of the given factor graph in a new window.
/// Does not work multiple times in a single execution of the program.
pub fn visualize(factor_graph: &FactorGraph) {
    let mut window = Window::new("gs-rs");
    let visual_factor_graph = add_factor_graph_to_window(&mut window, &factor_graph);

    let mut cam = ArcBall::new(Point3::new(0.0, 0.0, 50.0), Point3::new(0.0, 0.0, 0.0));
    while window.render_with_camera(&mut cam) {
        visual_factor_graph
            .lines
            .iter()
            .for_each(|line| window.draw_line(&line[0], &line[1], &line[2]));
    }
}

fn add_factor_graph_to_window(
    window: &mut Window,
    factor_graph: &FactorGraph,
) -> VisualFactorGraph {
    let mut visual_factor_graph = VisualFactorGraph {
        scene_node: window.add_group(),
        lines: vec![],
    };
    add_variables_and_factors(&mut visual_factor_graph, factor_graph);
    visual_factor_graph
}

// TODO split up into multiple functions
fn add_variables_and_factors(visual_factor_graph: &mut VisualFactorGraph, factor_graph: &FactorGraph) {
    for variable_index in &factor_graph.node_indices {
        let variable = factor_graph.csr.index(*variable_index);
        let var_point = Point3::new(
            variable.get_content()[0] as f32,
            variable.get_content()[1] as f32,
            0.0 as f32,
        );

        let mut variable_object = visual_factor_graph.scene_node.add_sphere(0.1);
        variable_object.set_local_translation(var_point.coords.into());

        if variable.get_type() == Vehicle2D {
            let mut var_rotation_object = variable_object.add_capsule(0.02, 2.0);
            let var_rotation = variable.get_content()[2] as f32;
            var_rotation_object.set_local_rotation(UnitQuaternion::from_axis_angle(
                &Vector3::z_axis(),
                var_rotation,
            ));
            var_rotation_object.prepend_to_local_translation(&Translation3::new(0.0, 0.20, 0.0));
        }

        match variable.get_type() {
            Vehicle2D => variable_object.set_color(1.0, 0.0, 0.0),
            Landmark2D => variable_object.set_color(0.0, 1.0, 0.0),
        };

        for edge in factor_graph.csr.edges(*variable_index) {
            let factor: &Factor = edge.weight();
            let factor_point = Point3::new(
                factor.constraint[0] as f32,
                factor.constraint[1] as f32,
                0.0 as f32,
            );
            let meas_point = match factor.factor_type {
                Position2D => factor_point,
                Odometry2D | Observation2D => {
                    let var_rotation = variable.get_content()[2] as f32;
                    let local_point = Rotation3::new(Vector3::z() * var_rotation) * factor_point;
                    (var_point.coords + local_point.coords).into()
                }
            };

            let (r, g, b) = match factor.factor_type {
                Position2D => (1.0, 0.5, 0.5),
                Odometry2D => (0.5, 0.5, 1.0),
                Observation2D => (0.5, 1.0, 0.5),
            };

            let mut measurement_object = visual_factor_graph.scene_node.add_cube(0.16, 0.16, 0.16);
            measurement_object.set_local_translation(meas_point.coords.into());

            if factor.factor_type == Position2D || factor.factor_type == Odometry2D {
                let factor_rotation = factor.constraint[2] as f32;
                let meas_rotation = match factor.factor_type {
                    Position2D => factor_rotation,
                    Odometry2D => {
                        let var_rotation = variable.get_content()[2] as f32;
                        var_rotation + factor_rotation
                    },
                    _ => panic!("Internal Error at visualization of unsupported rotation."),
                };
                let mut measured_rotation_object = measurement_object.add_capsule(0.04, 1.5);
                measured_rotation_object.set_local_rotation(UnitQuaternion::from_axis_angle(
                    &Vector3::z_axis(),
                    meas_rotation,
                ));
                measured_rotation_object
                    .prepend_to_local_translation(&Translation3::new(0.0, 0.15, 0.0));
            }

            measurement_object.set_color(r, g, b);

            visual_factor_graph
                .lines
                .push([meas_point, var_point, Point3::new(r, g, b)]);
            if factor.factor_type == Observation2D {
                let observed_variable = factor_graph.csr.index(edge.target());
                let observed_point = Point3::new(
                    observed_variable.get_content()[0] as f32,
                    observed_variable.get_content()[1] as f32,
                    0.0 as f32,
                );
                visual_factor_graph
                    .lines
                    .push([meas_point, observed_point, Point3::new(r, g, b)]);
            } else if factor.factor_type == Odometry2D {
                let observed_variable = factor_graph.csr.index(edge.target());
                let observed_point = Point3::new(
                    observed_variable.get_content()[0] as f32,
                    observed_variable.get_content()[1] as f32,
                    0.0 as f32,
                );
                visual_factor_graph.lines.push([
                    var_point,
                    observed_point,
                    Point3::new(1.0, 1.0, 1.0),
                ]);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use log::LevelFilter;

    use crate::parser::json::JsonParser;
    use crate::parser::Parser;

    use super::*;

    fn init() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(LevelFilter::Debug)
            .try_init();
    }

    #[test]
    #[ignore]
    fn test_visualize() {
        init();

        let factor_graph = JsonParser::parse_file("data_files/fullTestGraph.json").unwrap();
        visualize(&factor_graph);
    }
}
