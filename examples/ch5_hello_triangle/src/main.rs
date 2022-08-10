use std::ffi::{c_void, CStr};

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

use learnopengl_utils as utils;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const VERTEX_SHADER_SOURCE: &str = include_str!("shaders/shader1.glsl");
const FRAGMENT_SHADER_SOURCE: &str = include_str!("shaders/shader2.glsl");

fn main() {
    let (windowed_context, el) = utils::init(WIDTH, HEIGHT);

    unsafe {
        let msg = gl::GetString(gl::SHADING_LANGUAGE_VERSION);
        let msg = CStr::from_ptr(msg as *const _).to_owned();
        dbg!(msg);

        // Draw in debug line mode
        #[cfg(debug_assertions)] {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }
    }

    let vertex_shader =  utils::compile_shader(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER).unwrap() ;
    let fragment_shader =  utils::compile_shader(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER).unwrap() ;

    let shader_program = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        utils::get_status_and_output(
            gl::GetProgramiv,
            gl::GetProgramInfoLog,
            shader_program,
            gl::LINK_STATUS).unwrap();
    }

    let vertices: &[f32] = &[
        0.5, 0.5, 0.0,
        0.5, -0.5, 0.0,
        -0.5, -0.5, 0.0,
        -0.5, 0.5, 0.0,
    ];

    // Vertex Array Object
    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(vertices) as isize,
            vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW);
    }

    let indices: &[u32] = &[
        0, 1, 3,
        1, 2, 3,
    ];

    // Element Buffer Object
    let mut ebo = 0;
    unsafe {
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(indices) as _,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW);
    }

    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // The way to share the vbo, ebo in different vaos
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

        gl::VertexAttribPointer(
            0, 3, gl::FLOAT, gl::FALSE,
            3 * std::mem::size_of::<f32>() as i32,
            std::ptr::null());
        gl::EnableVertexAttribArray(0);
    }

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(..) => {
                unsafe {
                    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);

                    gl::UseProgram(shader_program);
                    gl::BindVertexArray(vao);
                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

                    // Unset the bindings
                    gl::BindVertexArray(0)
                }
                windowed_context.swap_buffers().ok();
            }
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::Resized(size) => {
                        unsafe {
                            gl::Viewport(0, 0, size.width as _, size.height as _);
                        }
                    }
                    WindowEvent::CloseRequested => {
                        dbg!(&event);
                        unsafe {
                            gl::DeleteShader(vertex_shader);
                            gl::DeleteShader(fragment_shader);
                        }
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    });
}
