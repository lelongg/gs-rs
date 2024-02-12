// -----------------------------------------------------------------------------------------------------
//                                      gs-rs - Graph SLAM in Rust
// -----------------------------------------------------------------------------------------------------
//
// SPDX-FileCopyrightText:      © 2020 Samuel Valenzuela (samuel.valenzuela@tngtech.com)
//                              © 2020 Florian Rohm (florian.rohm@tngtech.com)
//                              © 2020 Daniel Pape (daniel.pape@tngtech.com)
// SPDX-License-Identifier:     MIT OR Apache-2.0
//
// This product includes software developed at TNG Technology Consulting GmbH (https://www.tngtech.com/).
//


#![allow(non_snake_case)]

use crate::factor_graph::factor::Factor;
use crate::factor_graph::variable::{FixedType, VehicleVariable2D};
use nalgebra::{
    ArrayStorage, DMatrix, DVector, Matrix, Matrix3, Rotation2, Rotation3, RowVector3, Vector, Vector2, Vector3, U3,
};
use std::{f64::consts::PI, ops::Range};



pub fn update_H_b(H: &mut DMatrix<f64>, b: &mut DVector<f64>, factor: &Factor, var: &VehicleVariable2D) {
    let range = if let FixedType::NonFixed(range) = &var.fixed_type {
        range
    } else {
        return;
    };

    let (pos_v, rot_v) = get_pos_and_rot(&*var.pose.borrow());
    let (pos_m, rot_m) = get_pos_and_rot(&factor.constraint);
    let (jacobi, jacobi_T) = calc_jacobians(rot_m);
    let right_mult = &factor.information_matrix.content * jacobi;

    let H_update = jacobi_T * &right_mult;
    update_H_submatrix(H, &H_update, range.to_owned());

    let err_pos = Rotation2::new(-rot_m) * (pos_v - pos_m);
    let mut err_rot = rot_v - rot_m;
    if err_rot > PI {
        err_rot -= 2.0 * PI;
    } else if err_rot < -PI {
        err_rot += 2.0 * PI;
    }
    let mut err_vec = err_pos.data.as_slice().to_vec();
    err_vec.push(err_rot);
    let b_update = (RowVector3::from_vec(err_vec) * &right_mult).transpose();
    update_b_subvector(b, &b_update, range.to_owned());
}

fn calc_jacobians(rot_m: f64) -> (Matrix3<f64>, Matrix3<f64>) {
    let jacobian = *Rotation3::from_axis_angle(&Vector3::z_axis(), -rot_m).matrix();
    (jacobian, jacobian.transpose())
}

fn update_H_submatrix(
    H: &mut DMatrix<f64>,
    added_matrix: &Matrix<f64, U3, U3, ArrayStorage<f64, 3, 3>>,
    range: Range<usize>,
) {
    let updated_submatrix = &(H.index((range.clone(), range.clone())) + added_matrix);
    H.index_mut((range.clone(), range)).copy_from(updated_submatrix);
}

fn update_b_subvector(
    b: &mut DVector<f64>,
    added_vector: &Vector<f64, U3, ArrayStorage<f64, 3, 1 >>,
    range: Range<usize>,
) {
    let updated_subvector = &(b.index((range.clone(), ..)) + added_vector);
    b.index_mut((range, ..)).copy_from(updated_subvector);
}

fn get_pos_and_rot(pose: &[f64]) -> (Vector2<f64>, f64) {
    (Vector2::new(pose[0], pose[1]), pose[2])
}
