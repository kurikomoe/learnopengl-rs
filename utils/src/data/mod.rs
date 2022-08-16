use std::mem::size_of_val;
use gl::*;

pub mod triangle;

pub mod types {
    use gl::types::*;

    pub type Vec4 = (f32, f32, f32, f32);
    pub type Vec3 = (f32, f32, f32);
    pub type Vec2 = (f32, f32);

    // re-exports the opengl types to actual name
    type Uid = GLuint;
    type Id = GLuint;
    type Type = GLenum;
    pub type ProgramId = Uid;
    pub type VAOId = Uid;
    pub type VBOId = Uid;
    pub type EBOId = Uid;
}
pub use types::*;


pub enum UsageType {
    StaticDraw = STATIC_DRAW as _,
    StreamDraw = STREAM_DRAW as _,
    DynamicDraw = DYNAMIC_DRAW as _,
}


pub(crate) struct VertexBuffer {
    id: VBOId,
}

impl VertexBuffer {
    pub(crate) fn new<T>(data: &[T], usage_type: UsageType) -> Self {
        let mut id = 0;

        unsafe {
            GenBuffers(1, &mut id);
            assert_ne!(id, 0);
            BindBuffer(ARRAY_BUFFER, id);
            BufferData(
                ARRAY_BUFFER,
                size_of_val(data) as _,
                data.as_ptr() as *const _,
                usage_type as _);
        }

        Self {
            id,
        }
    }
}

pub(crate) struct ElementBuffer {
    id: EBOId,
}

impl ElementBuffer {
    pub(crate) fn new<T>(data: &[T], usage_type: UsageType) -> Self {
        let mut id = 0;
        unsafe {
            GenBuffers(1, &mut id);
            assert_ne!(id, 0);
            BindBuffer(ELEMENT_ARRAY_BUFFER, id);
            BufferData(
                ELEMENT_ARRAY_BUFFER,
                size_of_val(data) as _,
                data.as_ptr() as *const _,
                usage_type as _);
        }

        Self {
            id
        }
    }
}

pub(crate) struct VertexArray {
    id: VAOId,

    vbo: VertexBuffer,
    ebo: ElementBuffer,
}

impl VertexArray {
    pub(crate) fn new(vbo: VertexBuffer, ebo: ElementBuffer) -> Self {
        let mut id = 0;
        unsafe {
            GenBuffers(1, &mut id);
            assert_ne!(id, 0);
            BindVertexArray(id);
            BindBuffer(ARRAY_BUFFER, vbo.id);
            BindBuffer(ELEMENT_ARRAY_BUFFER, ebo.id);
        }
        Self {
            id,
            vbo,
            ebo,
        }
    }
}

