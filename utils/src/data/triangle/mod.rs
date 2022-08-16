use gl::*;
use gl::types::*;

use crate::data;

pub struct Triangle {
    program: GLuint,
    vao: data::VertexArray,
}

impl Triangle {
    pub fn new(program: data::ProgramId, data: &[u8]) -> Self {
        let vbo = data::VertexBuffer::new(data, data::UsageType::StaticDraw);
        let ebo = data::ElementBuffer::new(data, data::UsageType::StaticDraw);
        let vao = data::VertexArray::new(vbo, ebo);
        Self {
            program,
            vao,
        }
    }
}
