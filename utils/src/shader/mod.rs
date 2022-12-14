use std::ffi::{CStr, CString};
use std::sync::Arc;

use anyhow::Result;
use gl::*;
use gl::types::*;
use nalgebra::{Matrix4, OMatrix};

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
        COMPILE_STATUS)
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

macro_rules! build_uniform_setter {
    (@setter 1, $setter:tt, $loc:ident, $d:ident) => { $setter($loc, $d.0) };
    (@setter 2, $setter:tt, $loc:ident, $d:ident) => { $setter($loc, $d.0, $d.1) };
    (@setter 3, $setter:tt, $loc:ident, $d:ident) => { $setter($loc, $d.0, $d.1, $d.2) };
    (@setter 4, $setter:tt, $loc:ident, $d:ident) => { $setter($loc, $d.0, $d.1, $d.2, $d.3) };

    (@typer 1, $t:tt) => { ($t,) };
    (@typer 2, $t:tt) => { ($t, $t, ) };
    (@typer 3, $t:tt) => { ($t, $t, $t, ) };
    (@typer 4, $t:tt) => { ($t, $t, $t, $t, ) };

    ($n:tt, $func_name:ident, $t:ty, $setter:expr  ) => {
        pub fn $func_name (&self, varname: &str, data: build_uniform_setter!(@typer $n, $t)) -> Result<()> {
            let varname = CString::new(varname).unwrap();
            let loc = unsafe { gl::GetUniformLocation( self.prog, varname.as_ptr() as *const _, ) };
            assert_ne!(loc, -1);

            unsafe { build_uniform_setter!(@setter $n, $setter, loc, data); }
            Ok(())
        }
    };
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
        unsafe { gl::UseProgram(self.prog); }
        Ok(())
    }

    build_uniform_setter!(1, set_u32, u32, Uniform1ui);
    build_uniform_setter!(1, set_i32, i32, Uniform1i);
    build_uniform_setter!(1, set_f32, f32, Uniform1f);

    build_uniform_setter!(2, set_vec2ui, u32, Uniform2ui);
    build_uniform_setter!(2, set_vec2i, i32, Uniform2i);
    build_uniform_setter!(2, set_vec2, f32, Uniform2f);

    build_uniform_setter!(3, set_vec3ui, u32, Uniform3ui);
    build_uniform_setter!(3, set_vec3i, i32, Uniform3i);
    build_uniform_setter!(3, set_vec3, f32, Uniform3f);

    build_uniform_setter!(4, set_vec4ui, u32, Uniform4ui);
    build_uniform_setter!(4, set_vec4i, i32, Uniform4i);
    build_uniform_setter!(4, set_vec4, f32, Uniform4f);

    // build_uniform_setter!(3, set_mat4fv, f32, UniformMatrix4fv);
    // pub fn $func_name (&self, varname: &str, data: build_uniform_setter!(@typer $n, $t)) -> Result<()> {
    pub fn set_mat4fv(&self, varname: &str, mat: Arc<Matrix4<f32>>) -> Result<()> {
        let varname = CString::new(varname).unwrap();
        let loc = unsafe { gl::GetUniformLocation(self.prog, varname.as_ptr() as *const _) };
        assert_ne!(loc, -1);

        unsafe {
            let ptr = mat.as_ptr();
            gl::UniformMatrix4fv(loc, 1, FALSE, ptr as *const _);
        }

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