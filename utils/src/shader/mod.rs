use std::ffi::{CStr, CString};

use gl::*;
use gl::types::*;

use anyhow::Result;

pub fn compile_shader(source: &str, shader_type: GLenum) -> Result<GLuint, String> {
    let src = CString::new(source).unwrap();

    let shader = unsafe {
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &src.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
        shader
    };

    get_status_and_output(
        gl::GetShaderiv,
        gl::GetShaderInfoLog,
        shader,
        gl::COMPILE_STATUS)
}

pub type IVFunc = unsafe fn(GLuint, GLenum, *mut GLint);
pub type InfoLogFunc = unsafe fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar);

pub fn get_status_and_output(
    get_xx_iv: IVFunc,
    get_xx_info_log: InfoLogFunc,
    uid: GLuint,
    pname: GLenum) -> Result<GLuint, String>
{
    const MAX_LOG_LEN: usize = 512;

    let mut succ = 0;
    let mut len = 0;
    let mut info = [0i8; MAX_LOG_LEN];

    unsafe {
        get_xx_iv(uid, pname, &mut succ);

        match succ {
            0 => {
                get_xx_info_log(uid, MAX_LOG_LEN as i32, &mut len, info.as_mut_ptr());
                let msg = CStr::from_ptr(info.as_ptr()).to_str().unwrap().to_string();

                Err(dbg!(msg))
            }
            _ => Ok(uid)
        }
    }
}

pub struct Shader<'shader_src_life> {
    prog: GLuint,

    vertex_shader_src: &'shader_src_life str,
    fragment_shader_src: &'shader_src_life str,
}

impl<'a> Shader<'a> {
    pub fn new(vertex_shader_src: &'a str, fragment_shader_src: &'a str) -> Self {
        let vertex_shader = compile_shader(
            vertex_shader_src, VERTEX_SHADER)
            .expect("Vertex shader compile failed");

        let fragment_shader = compile_shader(
            fragment_shader_src, FRAGMENT_SHADER)
            .expect("fragment shader compile failed");

        let prog = unsafe { gl::CreateProgram() };

        unsafe {
            gl::AttachShader(prog, vertex_shader);
            gl::AttachShader(prog, fragment_shader);
            gl::LinkProgram(prog);

            get_status_and_output(
                gl::GetProgramiv,
                gl::GetProgramInfoLog,
                prog,
                LINK_STATUS).expect("Link Program Failed");

            // Shader can be deleted after linked
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(vertex_shader);
        }


        Self {
            prog,

            vertex_shader_src,
            fragment_shader_src,
        }
    }

    pub fn activate(&self) -> Result<()> {
        unsafe {
            gl::UseProgram(self.prog);
        }

        Ok(())
    }

    // Uniform1d(location: types::GLint, x: types::GLdouble) -> ()
    pub fn set_var<F: Fn(GLint)>(&self, varname: &str, setter: F) -> Result<()> {
        let varname = CString::new(varname).unwrap();
        let loc = unsafe {
            let ret = gl::GetUniformLocation(
                self.prog, varname.as_ptr() as *const _,
            );
            assert_ne!(ret, -1);
            ret
        };

        setter(loc);

        Ok(())
    }
}

impl<'a> Drop for Shader<'a> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.prog);
        }
    }
}

#[cfg(test)]
mod test {
    use glutin::event_loop::ControlFlow;
    use super::*;

    #[test]
    fn test_shader() {
        let (_window, ev) = crate::init(800, 600);


        let shader = Shader::new(
            include_str!("test/vertex.glsl"),
            include_str!("test/fragment.glsl"),
        );

        shader.activate().ok();

        shader.set_var(
            "uni",
            |loc| unsafe { gl::Uniform4f(loc, 0.1, 0.1, 0.1, 1.0); },
        ).ok();

        ev.run(|_event, _, control_flow| {
            *control_flow = ControlFlow::Exit;
        })
    }
}