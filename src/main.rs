use sdl2::event::{Event, WindowEvent};
use sdl2::video::SwapInterval;

mod objects;
use objects::*;

use std::f32::consts::PI;
use std::ptr::null;
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
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();

    let _gl = gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    window
        .subsystem()
        .gl_set_swap_interval(SwapInterval::VSync)
        .unwrap();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::Enable(gl::CULL_FACE);
    }

    let mut model_matrix: nalgebra_glm::Mat4 = nalgebra_glm::one();
    model_matrix = nalgebra_glm::rotate_y(&model_matrix, 1.5);
    let mut view_matrix: nalgebra_glm::Mat4 = nalgebra_glm::look_at(
        &nalgebra_glm::vec3(0.0,  0.0,  0.0), 
        &nalgebra_glm::vec3(0.0,  0.0,  -0.0), 
        &nalgebra_glm::vec3(0.0,  1.0,  0.0)
    );

    let mut proj_matrix: nalgebra_glm::Mat4 = nalgebra_glm::perspective(1.0, 1.5, 0.01, 100.0);

    let program = create_program().unwrap();
    program.set();

    let vertices = cube();
    let colors = cube_colors();

    // These Uniforms allow us to pass data (ex: window size, elapsed time) to the GPU shaders
    let u_resolution = Uniform::new(program.id(), "u_resolution").unwrap();
    // let u_time = Uniform::new(program.id(), "u_time").unwrap();
    let u_model_matrix = Uniform::new(program.id(), "u_model_matrix").unwrap();
    let u_view_matrix = Uniform::new(program.id(), "u_view_matrix").unwrap();
    let u_proj_matrix = Uniform::new(program.id(), "u_proj_matrix").unwrap();

    unsafe { 
        gl::Uniform2f(u_resolution.id, 600., 600.);
        // gl::Uniform1f(u_time.id, 0.0);

        gl::UniformMatrix4fv(u_model_matrix.id, 1, gl::FALSE, &model_matrix.columns(0, 4)[0]);
        gl::UniformMatrix4fv(u_view_matrix.id, 1, gl::FALSE, &view_matrix.columns(0, 4)[0]);
        gl::UniformMatrix4fv(u_proj_matrix.id, 1, gl::FALSE, &proj_matrix.columns(0, 4)[0]);
    }

    let vao = Vao::gen(); // glGenVertexArrays
    vao.set(0); // glBindVertexArray
    let vbo = Vbo::gen(); // genBuffers
    vbo.set(&vertices); // bindBuffer, bufferData
    let colors_vbo = Vbo::gen();// genBuffers
    colors_vbo.set(&colors); // bindBuffer, bufferData

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

                Event::Window {win_event, ..} => {
                    if let WindowEvent::Resized(new_width, new_height) = win_event {
                        unsafe {
                            gl::Viewport(0, 0, new_width, new_height);
                            gl::Uniform2f(u_resolution.id, new_width as f32, new_height as f32);
                        }
                    }
                }

                _ => {},
            }
        }

        unsafe {
            gl::ClearColor(0./255., 0./255., 20./255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            model_matrix = nalgebra_glm::one();
            model_matrix = nalgebra_glm::rotate_x(&model_matrix, start.elapsed().as_secs_f32());
            view_matrix = nalgebra_glm::look_at(
                &nalgebra_glm::vec3(start.elapsed().as_secs_f32().sin() * 5.0, 5.0 * start.elapsed().as_secs_f32().cos(),  5.0 * start.elapsed().as_secs_f32().cos()), 
                &nalgebra_glm::vec3(0.0,  0.0,  0.0), 
                &nalgebra_glm::vec3(0.0,  1.0,  0.0)
            );
            proj_matrix = nalgebra_glm::perspective(1.0, 1.5, 0.01, 100.0);

            // gl::Uniform1f(u_time.id, start.elapsed().as_secs_f32());
            gl::UniformMatrix4fv(u_model_matrix.id, 1, gl::FALSE, &model_matrix.columns(0, 4)[0]);
            gl::UniformMatrix4fv(u_view_matrix.id, 1, gl::FALSE, &view_matrix.columns(0, 4)[0]);
            gl::UniformMatrix4fv(u_proj_matrix.id, 1, gl::FALSE, &proj_matrix.columns(0, 4)[0]);
            
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo.id);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                0,
                null(),
            );

            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, colors_vbo.id);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                0,
                null(),
            );

            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                12*3,
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

fn cube() -> Vec<f32> {
    vec![
        -1.0,-1.0,-1.0, // triangle 1 : begin
        -1.0,-1.0, 1.0,
        -1.0, 1.0, 1.0, // triangle 1 : end
        1.0, 1.0,-1.0, // triangle 2 : begin
        -1.0,-1.0,-1.0,
        -1.0, 1.0,-1.0, // triangle 2 : end
        1.0,-1.0, 1.0,
        -1.0,-1.0,-1.0,
        1.0,-1.0,-1.0,
        1.0, 1.0,-1.0,
        1.0,-1.0,-1.0,
        -1.0,-1.0,-1.0,
        -1.0,-1.0,-1.0,
        -1.0, 1.0, 1.0,
        -1.0, 1.0,-1.0,
        1.0,-1.0, 1.0,
        -1.0,-1.0, 1.0,
        -1.0,-1.0,-1.0,
        -1.0, 1.0, 1.0,
        -1.0,-1.0, 1.0,
        1.0,-1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0,-1.0,-1.0,
        1.0, 1.0,-1.0,
        1.0,-1.0,-1.0,
        1.0, 1.0, 1.0,
        1.0,-1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0,-1.0,
        -1.0, 1.0,-1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0,-1.0,
        -1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0,
        1.0,-1.0, 1.0
    ]
}

fn cube_colors() -> Vec<f32> {
    vec![
        0.583,  0.771,  0.014,
        0.609,  0.115,  0.436,
        0.327,  0.483,  0.844,
        0.822,  0.569,  0.201,
        0.435,  0.602,  0.223,
        0.310,  0.747,  0.185,
        0.597,  0.770,  0.761,
        0.559,  0.436,  0.730,
        0.359,  0.583,  0.152,
        0.483,  0.596,  0.789,
        0.559,  0.861,  0.639,
        0.195,  0.548,  0.859,
        0.014,  0.184,  0.576,
        0.771,  0.328,  0.970,
        0.406,  0.615,  0.116,
        0.676,  0.977,  0.133,
        0.971,  0.572,  0.833,
        0.140,  0.616,  0.489,
        0.997,  0.513,  0.064,
        0.945,  0.719,  0.592,
        0.543,  0.021,  0.978,
        0.279,  0.317,  0.505,
        0.167,  0.620,  0.077,
        0.347,  0.857,  0.137,
        0.055,  0.953,  0.042,
        0.714,  0.505,  0.345,
        0.783,  0.290,  0.734,
        0.722,  0.645,  0.174,
        0.302,  0.455,  0.848,
        0.225,  0.587,  0.040,
        0.517,  0.713,  0.338,
        0.053,  0.959,  0.120,
        0.393,  0.621,  0.362,
        0.673,  0.211,  0.457,
        0.820,  0.883,  0.371,
        0.982,  0.099,  0.879
    ]
}