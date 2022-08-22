use nalgebra::{Point, Point3};
use crate::data;

pub struct Triangle<const N: usize> {




    a: Point<f32, N>,
    b: Point<f32, N>,
    c: Point<f32, N>,
}

impl<const N: usize> Triangle<N> {
    pub fn new(a: Point<f32, N>, b: Point<f32, N>, c: Point<f32, N>) -> Self {
        // The vertex of the triangle.

        // let vbo = data::VertexBuffer::new(data, data::UsageType::StaticDraw);
        // let ebo = data::ElementBuffer::new(data, data::UsageType::StaticDraw);
        // let vao = data::VertexArray::new(vbo, ebo);

        Self {
            a, b, c
        }
    }
}
