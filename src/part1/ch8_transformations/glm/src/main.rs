#![allow(unreachable_code, unused_variables, unused_imports)]

use std::io::Cursor;
use std::mem::{size_of, size_of_val};
use std::sync::Arc;

use anyhow::Result;
use chrono::TimeZone;
use gl::{Clear, ClearColor, COLOR_BUFFER_BIT};
use gl::*;
use gl::types::*;
use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;
use image::DynamicImage;
use image::io::Reader as ImgReader;
use nalgebra::{Matrix4, Point3, Rotation3, Scale3, Translation3, Vector3};

use learnopengl_utils as utils;

fn main() -> Result<()> {
    // glm::vec4 vec(1.0f, 0.0f, 0.0f, 1.0f);
    // Here, vec4 is (1, 0, 0) with homogeneous coordinate 1.0
    // let vec = Point3::<f32>::new(1.0, 0.0, 0.0);
    // let vec = vec.to_homogeneous();
    // glm::mat4 trans = glm::mat4(1.0f);
    // trans = glm::translate(trans, glm::vec3(1.0f, 1.0f, 0.0f));
    // let trans = Matrix4::identity();
    // let trans = Translation3::<f32>::new(1.0, 1.0, 0.0).to_homogeneous();
    let trans = Translation3::<f32>::new(0.5, -0.5, 0.0).to_homogeneous();
    // vec = trans * vec;
    // let vec = trans * vec;
    // dbg!(vec);

    let axisangle = Vector3::z() * std::f32::consts::FRAC_PI_2;
    let trans = Rotation3::new(axisangle).to_homogeneous();
    let trans = trans * Scale3::new(0.5, 0.5, 0.5).to_homogeneous();

    // let vec = trans * vec;
    // dbg!(vec);

    // Normal opengl rendering procedure
    let (windowed_context, el) = utils::init(800, 600, true);


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
    unsafe {
        gl::TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, MIRRORED_REPEAT as _);
        // minifying, scale down
        gl::TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as _);
        // magnifying, scale up
        gl::TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as _);

        //mipmaps
        gl::TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR_MIPMAP_LINEAR as _);
        // No use, because mipmaps not used in scale up
        // gl::TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR_MIPMAP_LINEAR as _);
    }

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
            .decode()?
            .flipv();  // flip the image so that fit into the opengl coordination.
        let w = img.width();
        let h = img.height();
        let format = match img {
            DynamicImage::ImageRgb8(_) => {
                dbg!("rgb");
                RGB
            }
            DynamicImage::ImageRgba8(_) => {
                dbg!("rgba");
                RGBA
            }
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
        include_str!("shaders/shader1.vs.glsl"),
        include_str!("shaders/shader1.fg.glsl"),
    );


    shader.activate().ok();
    shader.set_i32("texture1", (0, )).ok();
    shader.set_i32("texture2", (1, )).ok();

    // shader.set_mat4fv("transform", Arc::clone(&mat)).unwrap();


    let mut frames = 0;
    let mut events = 0;
    let mut timer = chrono::Utc::now().timestamp_millis();

    let mut mix_rate = 0.5f32;

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
            Event::RedrawEventsCleared => unsafe {
                frames += 1;

                ClearColor(0.3, 0.3, 0.3, 1.0);
                Clear(COLOR_BUFFER_BIT);

                let trans = Translation3::<f32>::new(0.5, -0.5, 0.0).to_homogeneous();
                let axisangle = Vector3::z() * (frames as f32) / 30.0 * std::f32::consts::PI;
                let trans = Rotation3::new(axisangle).to_homogeneous();
                let mat = Arc::new(trans);

                shader.set_mat4fv("transform", Arc::clone(&mat)).unwrap();
                shader.set_f32("mix_rate", (mix_rate, )).ok();

                DrawElements(TRIANGLES, 6, UNSIGNED_INT, std::ptr::null());

                windowed_context.swap_buffers().ok();
            }
            Event::MainEventsCleared => {}
            Event::RedrawRequested(..) => unsafe {
                ClearColor(0.3, 0.3, 0.3, 1.0);
                Clear(COLOR_BUFFER_BIT);

                windowed_context.swap_buffers().ok();
            }
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::KeyboardInput { input: KeyboardInput { state, virtual_keycode: Some(key), .. }, .. } => {
                        match (key, state) {
                            (VirtualKeyCode::Escape, ElementState::Released) => {
                                *control_flow = ControlFlow::ExitWithCode(0);
                            }
                            (VirtualKeyCode::Up, ElementState::Pressed) => {
                                mix_rate = f32::min(mix_rate + 0.1, 1.0);
                            }
                            (VirtualKeyCode::Down, ElementState::Pressed) => {
                                mix_rate = f32::max(mix_rate - 0.1, 0.0);
                            }
                            _ => {}
                        };
                    }
                    WindowEvent::Resized(size) => unsafe {
                        gl::Viewport(0, 0, size.width as _, size.height as _);
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

        if *control_flow == ControlFlow::Wait {
            *control_flow = ControlFlow::Poll;
        }
    });

    Ok(())
}

