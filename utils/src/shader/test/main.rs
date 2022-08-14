use learnopengl_utils::*;
use learnopengl_utils::shader::Shader;

use glutin::event_loop::ControlFlow;

use partial_application::partial;

fn main() {
    let (_window, ev) = crate::init_headless(800, 600);


    let shader = Shader::new(
        include_str!("vertex.glsl"),
        include_str!("fragment.glsl"),
    );


    shader.set_variable(
        "uni",
        unsafe { partial!(gl::Uniform4f => _, 0.1, 0.1, 0.1, 1.0) },
    ).ok();

    shader.activate().ok();

    ev.run(|_event, _, control_flow| {
        *control_flow = ControlFlow::Exit;
    })
}