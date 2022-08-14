#![feature(trait_alias)]
#![feature(format_args_nl)]
#![allow(dead_code)]

use std::ffi::{c_void, CStr};

use gl::DEBUG_TYPE_ERROR;
use gl::types::*;
use glutin::{Api, Context, ContextBuilder, ContextWrapper, GlProfile, GlRequest, PossiblyCurrent};
use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use glutin::window::{Window, WindowBuilder};

pub use shader::compile_shader;
pub use shader::get_status_and_output;
pub use shader::Shader;

pub mod shader;

/// # Safety
/// Please ensure that gl is loaded.
pub unsafe fn init_error_callback() {
    extern "system" fn callback(_source: GLenum,
                                gltype: GLenum,
                                _id: GLuint,
                                severity: GLenum,
                                _length: GLsizei,
                                message: *const GLchar,
                                _user_param: *mut c_void)
    {
        let msg = unsafe {
            CStr::from_ptr(message as *mut _).to_str().unwrap().to_owned()
        };
        let tt = if gltype == DEBUG_TYPE_ERROR { " ** GL ERROR **" } else { "" };
        eprintln!("GL CALLBACK: {tt} type = {gltype:#x}, severity = {severity}\n {msg}");
    }
    gl::DebugMessageCallback(Some(callback), std::ptr::null());
}

pub fn init_headless(width: u32, height: u32, debug: bool) -> (Context<PossiblyCurrent>, EventLoop<()>) {
    let el = EventLoop::new();

    let context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_gl_profile(GlProfile::Core)
        .with_gl_debug_flag(debug)
        .with_vsync(true)
        .build_headless(&el, PhysicalSize::new(width, height))
        .unwrap();


    let windowed_context = unsafe { context.make_current().unwrap() };

    gl::load_with(|s| windowed_context.get_proc_address(s));

    if debug {
        unsafe { init_error_callback() }
    }

    unsafe { gl::Viewport(0, 0, width as _, height as _); }

    (windowed_context, el)
}

pub fn init(width: u32, height: u32, debug: bool) -> (ContextWrapper<PossiblyCurrent, Window>, EventLoop<()>) {
    let el = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(width, height))
        .with_title("Test Window");

    let windowed_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_gl_profile(GlProfile::Core)
        .with_vsync(debug)
        .build_windowed(wb, &el)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    gl::load_with(|s| windowed_context.get_proc_address(s));

    if debug {
        unsafe { init_error_callback() }
    }

    unsafe { gl::Viewport(0, 0, width as _, height as _); }

    (windowed_context, el)
}
