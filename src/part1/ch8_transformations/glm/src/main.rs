#![allow(unreachable_code, unused_variables, unused_imports)]

use anyhow::Result;
use nalgebra::{Matrix3x4, Matrix4, SMatrix, SVector, Vector4};

use learnopengl_utils as utils;

fn main() -> Result<()> {
    let vec = SVector::<f32, 4>::new(1.0, 0.0, 0.0, 1.0);
    let trans = SMatrix::<f32, 4, 4>::identity();

    let vec2 = trans * vec;

    dbg!(vec2);

    Ok(())
}

