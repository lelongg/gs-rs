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

//! The internal representation of a factor graph's optimizable variable.

use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
pub enum FixedType {
    Fixed,
    NonFixed(Range<usize>),
}

/// Representation of an optimizable vehicle variable.
#[derive(Debug)]
pub struct VehicleVariable2D {
    pub id: usize,
    pub pose: Rc<RefCell<[f64; 3]>>,
    pub fixed_type: FixedType,
}

/// Representation of an optimizable landmark variable.
#[derive(Debug)]
pub struct LandmarkVariable2D {
    pub id: usize,
    pub position: Rc<RefCell<[f64; 2]>>,
    pub fixed_type: FixedType,
}

/// Representation of an optimizable vehicle variable.
#[derive(Debug)]
pub struct VehicleVariable3D {
    pub id: usize,
    pub pose: Rc<RefCell<[f64; 7]>>,
    pub fixed_type: FixedType,
}

/// Representation of an optimizable landmark variable.
#[derive(Debug)]
pub struct LandmarkVariable3D {
    pub id: usize,
    pub position: Rc<RefCell<[f64; 3]>>,
    pub fixed_type: FixedType,
}

/// Enum representing a supported variable type.
#[derive(Debug)]
pub enum Variable {
    /// Vehicle pose (position and rotation) in 2D.
    Vehicle2D(VehicleVariable2D),
    /// Landmark position in 2D.
    Landmark2D(LandmarkVariable2D),
    /// Vehicle pose (position and rotation) in 3D.
    Vehicle3D(VehicleVariable3D),
    /// Landmark position in 3D.
    Landmark3D(LandmarkVariable3D),
}
impl VehicleVariable2D {
    /// Returns a new variable from a 2D pose, a given ID and whether the variable is fixed.
    pub fn new(id: usize, x: f64, y: f64, phi: f64, fixed_type: FixedType) -> Self {
        VehicleVariable2D {
            id,
            pose: Rc::new(RefCell::new([x, y, phi])),
            fixed_type,
        }
    }
}

impl LandmarkVariable2D {
    /// Returns a new variable from a 2D position, a given ID and whether the variable is fixed.
    pub fn new(id: usize, x: f64, y: f64, fixed_type: FixedType) -> Self {
        LandmarkVariable2D {
            id,
            position: Rc::new(RefCell::new([x, y])),
            fixed_type,
        }
    }
}

impl VehicleVariable3D {
    /// Returns a new variable from a 3D pose, a given ID and whether the variable is fixed.
    pub fn new(
        id: usize,
        x: f64,
        y: f64,
        z: f64,
        rot_x: f64,
        rot_y: f64,
        rot_z: f64,
        rot_w: f64,
        fixed_type: FixedType,
    ) -> Self {
        VehicleVariable3D {
            id,
            pose: Rc::new(RefCell::new([x, y, z, rot_x, rot_y, rot_z, rot_w])),
            fixed_type,
        }
    }
}

impl LandmarkVariable3D {
    /// Returns a new variable from a 3D position, a given ID and whether the variable is fixed.
    pub fn new(id: usize, x: f64, y: f64, z: f64, fixed_type: FixedType) -> Self {
        LandmarkVariable3D {
            id,
            position: Rc::new(RefCell::new([x, y, z])),
            fixed_type,
        }
    }
}

impl Variable {
    pub fn get_fixed_type(&self) -> &FixedType {
        match self {
            Variable::Vehicle2D(v) => &v.fixed_type,
            Variable::Landmark2D(v) => &v.fixed_type,
            Variable::Vehicle3D(v) => &v.fixed_type,
            Variable::Landmark3D(v) => &v.fixed_type,
        }
    }
    pub fn get_content(&self) -> Vec<f64> {
        match self {
            Variable::Vehicle2D(v) => v.pose.borrow().to_vec(),
            Variable::Landmark2D(v) => v.position.borrow().to_vec(),
            Variable::Vehicle3D(v) => v.pose.borrow().to_vec(),
            Variable::Landmark3D(v) => v.position.borrow().to_vec(),
        }
    }
    pub fn set_content(&self, update: Vec<f64>) {
        let u = update;
        match self {
            Variable::Vehicle2D(v) => *v.pose.borrow_mut() = [u[0], u[1], u[2]],
            Variable::Landmark2D(v) => *v.position.borrow_mut() = [u[0], u[1]],
            Variable::Vehicle3D(v) => *v.pose.borrow_mut() = [u[0], u[1], u[2], u[3], u[4], u[5], u[6]],
            Variable::Landmark3D(v) => *v.position.borrow_mut() = [u[0], u[1], u[2]],
        }
    }
    pub fn get_id(&self) -> usize {
        match self {
            Variable::Vehicle2D(v) => v.id,
            Variable::Landmark2D(v) => v.id,
            Variable::Vehicle3D(v) => v.id,
            Variable::Landmark3D(v) => v.id,
        }
    }
}
