#![feature(trait_alias)]
#![allow(dead_code)]

use glutin::{Api, ContextBuilder, ContextWrapper, GlProfile, GlRequest, PossiblyCurrent};
use glutin::dpi::PhysicalSize;
use glutin::event_loop::{EventLoop, EventLoopBuilder};
use glutin::platform::unix::EventLoopBuilderExtUnix;
use glutin::window::{Window, WindowBuilder};

pub use shader::compile_shader;
pub use shader::get_status_and_output;

pub mod shader;

pub fn init(width: u32, height: u32) -> (ContextWrapper<PossiblyCurrent, Window>, EventLoop<()>) {
    let el = if cfg!(test) {
        EventLoopBuilder::new()
            .with_any_thread(true)
            .build()
    } else {
        EventLoop::new()
    };

    // #[cfg(test)]
    // let el = EventLoopBuilder::new()
    //     .with_any_thread(true)
    //     .build();
    //
    // #[cfg(not(test))]
    // let el = EventLoop::new();

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

    unsafe { gl::Viewport(0, 0, width as _, height as _); }

    (windowed_context, el)
}
