#![allow(unreachable_code, unused_variables, unused_imports)]

use std::f64::consts::PI;
use std::ffi::CString;
use std::mem::{size_of, size_of_val};
use std::ops::Add;
use std::thread;
use std::time::{Duration, Instant};

use chrono::{Timelike, TimeZone};
use gl::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::{ContextWrapper, PossiblyCurrent};

use learnopengl_utils as utils;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let (windowed_context, el) = utils::init(WIDTH, HEIGHT, true);

    let mut nr_attributes = 0;
    unsafe {
        gl::GetIntegerv(MAX_VERTEX_ATTRIBS, &mut nr_attributes);

        // Draw in debug line mode
        #[cfg(debug_assertions)] {
            // gl::PolygonMode(FRONT_AND_BACK, LINE);
        }
    }
    dbg!(nr_attributes);

    let vertices: &[f32] = &[
        // position      // colors
        0.0, 0.5, 0.0, 1.0, 0.0, 0.0,
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
        -0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
    ];
    let mut vbo = 0;
    unsafe {
        GenBuffers(1, &mut vbo);
        BindBuffer(ARRAY_BUFFER, vbo);
        BufferData(
            ARRAY_BUFFER,
            size_of_val(vertices) as _,
            vertices.as_ptr() as *const _,
            STATIC_DRAW);
    }
    assert_ne!(vbo, 0);

    let elements: &[u32] = &[
        0, 1, 2,
    ];
    let mut ebo = 0;
    unsafe {
        GenBuffers(1, &mut ebo);
        BindBuffer(ELEMENT_ARRAY_BUFFER, ebo);
        BufferData(
            ELEMENT_ARRAY_BUFFER,
            size_of_val(elements) as _,
            elements.as_ptr() as *const _,
            STATIC_DRAW);
    }
    assert_ne!(ebo, 0);

    let mut vao = 0;
    unsafe {
        GenVertexArrays(1, &mut vao);
        BindVertexArray(vao);

        BindBuffer(ARRAY_BUFFER, vbo);
        BindBuffer(ELEMENT_ARRAY_BUFFER, ebo);

        VertexAttribPointer(
            0, 3, FLOAT, FALSE,
            6 * size_of::<f32>() as i32,
            std::ptr::null(),
        );
        VertexAttribPointer(
            1, 3, FLOAT, FALSE,
            6 * size_of::<f32>() as i32,
            (3 * size_of::<f32>()) as *const _,
        );
        EnableVertexAttribArray(0);
        EnableVertexAttribArray(1);
    }
    assert_ne!(vao, 0);


    let vertex_shader = utils::compile_shader(
        include_str!("shaders/vertex.glsl"), VERTEX_SHADER)
        .expect("Shader compile failed");

    let frag_shader = utils::compile_shader(
        include_str!("shaders/fragment.glsl"), FRAGMENT_SHADER)
        .expect("Shader compile failed");

    let shader_program = unsafe { CreateProgram() };
    unsafe {
        AttachShader(shader_program, vertex_shader);
        AttachShader(shader_program, frag_shader);

        LinkProgram(shader_program);

        utils::get_status_and_output(
            gl::GetProgramiv,
            gl::GetProgramInfoLog,
            shader_program,
            LINK_STATUS).unwrap();
    }


    let mut frames = 0;
    let mut events = 0;
    let base = chrono::Utc::now().timestamp_millis();
    let mut timer = base;

    el.run(move |event, _, control_flow| {
        // dbg!(&event);
        *control_flow = ControlFlow::Wait;

        events += 1;

        let now2 = chrono::Utc::now();

        if now2.timestamp_millis() - timer >= 1000 {
            timer = now2.timestamp_millis();

            println!("{frames} {events}");

            frames = 0;
            events = 0;
        }

        match event {
            Event::RedrawEventsCleared => {
                frames += 1;

                unsafe {
                    ClearColor(0.3, 0.3, 0.3, 1.0);
                    Clear(COLOR_BUFFER_BIT);

                    UseProgram(shader_program);

                    gl::BindVertexArray(vao);
                    // DrawArrays(TRIANGLES, 0, 3);
                    DrawElements(TRIANGLES, 3, UNSIGNED_INT, std::ptr::null());
                }
                windowed_context.swap_buffers().ok();
            }
            Event::MainEventsCleared => {}
            Event::RedrawRequested(..) => {
                unsafe {
                    ClearColor(0.3, 0.3, 0.3, 1.0);
                    Clear(COLOR_BUFFER_BIT);
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
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    _ => (),
                }
            }
            _ => (),
        }

        *control_flow = ControlFlow::Poll;
    });
}

