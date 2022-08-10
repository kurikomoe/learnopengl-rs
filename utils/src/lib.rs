use std::ffi::{CStr, CString};

use gl::types::*;
use glutin::{Api, ContextBuilder, ContextWrapper, GlProfile, GlRequest, PossiblyCurrent};
use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use glutin::window::{Window, WindowBuilder};

pub fn init(width: u32, height: u32) -> (ContextWrapper<PossiblyCurrent, Window>, EventLoop<()>) {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(width, height))
        .with_title("Test Window");
    let windowed_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_gl_profile(GlProfile::Core)
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    gl::load_with(|s| windowed_context.get_proc_address(s));

    (windowed_context, el)
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
