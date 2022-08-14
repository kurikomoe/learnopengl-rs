#![allow(unreachable_code, unused_variables, unused_imports)]

use std::f64::consts::PI;
use std::ffi::CString;
use std::io::Cursor;
use std::mem::{size_of, size_of_val};
use std::ops::Add;
use std::thread;
use std::time::{Duration, Instant};

use chrono::{Timelike, TimeZone};
use gl::*;
use gl::types::*;
use glutin::{ContextWrapper, PossiblyCurrent};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use image::io::Reader as ImgReader;

use anyhow::Result;
use image::DynamicImage;
use partial_application::partial;

use learnopengl_utils as utils;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() -> Result<()> {
    let (windowed_context, el) = utils::init(WIDTH, HEIGHT, true);


    let tex_coords: &[f32] = &[
        0.0, 0.0,
        1.0, 0.0,
        0.5, 1.0,
    ];
    /*
        GL_REPEAT: Repeats the texture image
        GL_MIRRORED_REPEAT: Same as GL_REPEAT but mirrors the texture image
        GL_CLAMP_TO_EDGE: Clamps the coordinates between 0 and 1.
        GL_CLAMP_TO_BORDER: Coordinates outside the range are now given
            a user-specified border color.
     */
    // unsafe {
    //     gl::TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, MIRRORED_REPEAT as _);
    //     // minifying, scale down
    //     gl::TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as _);
    //     // magnifying, scale up
    //     gl::TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as _);
    //
    //     //mipmaps
    //     gl::TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR_MIPMAP_LINEAR as _);
    //     // No use, because mipmaps not used in scale up
    //     // gl::TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR_MIPMAP_LINEAR as _);
    // }

    let stride = 8;
    let vertices: &[f32] = &[
        // position       // colors        // texture coords
        0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0,  // top right
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,  // bottom right
        -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,  // bottom left
        -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0,  // top left
    ];
    let mut vbo = 0;
    unsafe {
        GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        BindBuffer(ARRAY_BUFFER, vbo);
        BufferData(
            ARRAY_BUFFER,
            size_of_val(vertices) as _,
            vertices.as_ptr() as *const _,
            STATIC_DRAW);
    }

    let elements: &[u32] = &[
        0, 1, 3,
        1, 2, 3,
    ];
    let mut ebo = 0;
    unsafe {
        GenBuffers(1, &mut ebo);
        assert_ne!(ebo, 0);
        BindBuffer(ELEMENT_ARRAY_BUFFER, ebo);
        BufferData(
            ELEMENT_ARRAY_BUFFER,
            size_of_val(elements) as _,
            elements.as_ptr() as *const _,
            STATIC_DRAW);
    }

    let mut vao = 0;
    unsafe {
        GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        BindVertexArray(vao);

        BindBuffer(ARRAY_BUFFER, vbo);
        BindBuffer(ELEMENT_ARRAY_BUFFER, ebo);

        VertexAttribPointer(
            0, 3, FLOAT, FALSE,
            stride * size_of::<f32>() as i32,
            std::ptr::null(),
        );
        EnableVertexAttribArray(0);
        VertexAttribPointer(
            1, 3, FLOAT, FALSE,
            stride * size_of::<f32>() as i32,
            (3 * size_of::<f32>()) as *const _,
        );
        EnableVertexAttribArray(1);
        VertexAttribPointer(
            2, 2, FLOAT, FALSE,
            stride * size_of::<f32>() as i32,
            (6 * size_of::<f32>()) as *const _,
        );
        EnableVertexAttribArray(2);
    }

    fn load_texture(data: &[u8]) -> Result<GLuint> {
        let img = ImgReader::new(Cursor::new(data))
            .with_guessed_format()?
            .decode()?;
        let w = img.width();
        let h = img.height();
        let format = match img {
            DynamicImage::ImageRgb8(_) => { dbg!("rgb"); RGB }
            DynamicImage::ImageRgba8(_) => { dbg!("rgba"); RGBA }
            _ => { unimplemented!() }
        };


        let mut tex = 0;
        unsafe {
            GenTextures(1, &mut tex);
            assert_ne!(tex, 0);

            BindTexture(TEXTURE_2D, tex);

            TexImage2D(
                // generate a texture_2d target
                TEXTURE_2D,
                // mipmap level, base level is 0
                0,
                // store the image in RGB format
                format as _,
                // image size
                w as _, h as _,
                // always be 0
                0,
                // source image type in RGB with unsigned bytes
                format, UNSIGNED_BYTE,
                // source pointer
                img.as_bytes().as_ptr() as *const _,
            );
            GenerateMipmap(TEXTURE_2D);
        }

        Ok(tex)
    }

    let tex1 = load_texture(include_bytes!("textures/wall.jpg"))?;
    let tex2 = load_texture(include_bytes!("textures/awesomeface.png"))?;

    unsafe {
        ActiveTexture(TEXTURE0);
        BindTexture(TEXTURE_2D, tex1);

        ActiveTexture(TEXTURE1);
        BindTexture(TEXTURE_2D, tex2);
    }


    let shader = utils::Shader::new(
        include_str!("shaders/vertex.glsl"),
        include_str!("shaders/fragment.glsl"),
    );


    shader.activate().ok();
    shader.set_i32("texture1", (0, )).ok();
    shader.set_i32("texture2", (1, )).ok();


    let mut frames = 0;
    let mut events = 0;
    let mut timer  = chrono::Utc::now().timestamp_millis();

    el.run(move |event, _, control_flow| {
        // dbg!(&event);
        *control_flow = ControlFlow::Wait;

        events += 1;

        let now2 = chrono::Utc::now();

        if now2.timestamp_millis() - timer >= 1000 {
            timer = now2.timestamp_millis();

            println!("frames:{frames}/s  events:{events}/s");

            frames = 0;
            events = 0;
        }


        match event {
            Event::RedrawEventsCleared => {
                frames += 1;

                unsafe {
                    ClearColor(0.3, 0.3, 0.3, 1.0);
                    Clear(COLOR_BUFFER_BIT);

                    DrawElements(TRIANGLES, 6, UNSIGNED_INT, std::ptr::null());
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

    Ok(())
}

