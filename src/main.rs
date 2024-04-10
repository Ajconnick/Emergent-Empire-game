use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::video::{gl_attr, SwapInterval};

mod objects;
use objects::*;

use std::f32::consts::PI;
use std::time::Instant;

fn main() -> Result <(), String> {
    let screen_width = 600;
    let screen_height = 600;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Window!", screen_width, screen_height)
        .opengl()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();

    let gl = gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    window
        .subsystem()
        .gl_set_swap_interval(SwapInterval::VSync)
        .unwrap();

    let program = create_program().unwrap();
    program.set();

    let (mut vertices, mut indices) = triangle_fan(1000000);
    let vbo = Vbo::gen();
    let vao = Vao::gen();
    let ibo = Ibo::gen();

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();
    let start = Instant::now();
    let mut seconds_elapsed: u32 = 0;

    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit {..} => {
                    running = false;
                }

                Event::MouseMotion {x, y, xrel, yrel, ..} => {
                    println!("Mouse x: {}, y: {}", x, y);
                    println!("Relative x: {}, y: {}", xrel, yrel);
                },

                _ => {},
            }
        }

        unsafe {
            gl::ClearColor(54./255., 159./255., 219./255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            if start.elapsed().as_secs_f32().floor() as u32 > seconds_elapsed {
                seconds_elapsed += 1;
                (vertices, indices) = triangle_fan(seconds_elapsed);
                vbo.set(&vertices);
                vao.set();
                ibo.set(&indices);
            }

            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const _,
            );
        }

        window.gl_swap_window();
    }

    println!("Hello, world!");

    Ok(())
}

fn triangle_fan(n: u32) -> (Vec<f32>, Vec<u32>) {
    let mut vertices: Vec<f32> = vec![
        0.0, 0.0,
        0.5, 0.0,
    ];
    let mut indices: Vec<u32> = vec![];

    let mut angle: f32;
    for m in 1..n {
        angle = 2. * PI * m as f32 / n as f32;
        vertices.push(angle.cos() * 0.5);
        vertices.push(angle.sin() * 0.5);

        indices.push(0);
        indices.push(m);
        indices.push(m+1);
    }
    indices.push(0);
    indices.push(n);
    indices.push(1);

    (vertices, indices)
}