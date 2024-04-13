use sdl2::event::{Event, WindowEvent};
use sdl2::video::SwapInterval;

mod objects;
use objects::*;

use std::f32::consts::PI;
use std::ptr::null;
use std::time::Instant;

use obj::{load_obj, Obj, Vertex};

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
        // gl::Enable(gl::CULL_FACE);
    }

    // MATRICES
    let mut model_matrix: nalgebra_glm::Mat4 = nalgebra_glm::one();
    model_matrix = nalgebra_glm::rotate_y(&model_matrix, 1.5);
    let mut view_matrix: nalgebra_glm::Mat4 = nalgebra_glm::look_at(
        &nalgebra_glm::vec3(0.0,  0.0,  0.0), 
        &nalgebra_glm::vec3(0.0,  0.0,  -0.0), 
        &nalgebra_glm::vec3(0.0,  1.0,  0.0)
    );
    let mut proj_matrix: nalgebra_glm::Mat4 = nalgebra_glm::perspective(1.0, 1.5, 0.01, 100.0);

    // OPEN GL SHADERS 'N SHIET!
    let program = create_program().unwrap();
    program.set();

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

    let input = include_bytes!("../res/ico-sphere.obj");
    let obj: Obj = load_obj(&input[..]).unwrap();
    let vb = obj.vertices;
    let mut indices = obj.indices;
    let mut vertices = flatten(vb);

    let vbo = Vbo::gen();
    let vao = Vao::gen();
    let ibo = Ibo::gen();

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();
    let start = Instant::now();
    let mut seconds_elapsed: u16 = 0;

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

            
        model_matrix = nalgebra_glm::one();
        // model_matrix = nalgebra_glm::translate(&model_matrix, &nalgebra_glm::vec3(0.0, start.elapsed().as_secs_f32().sin() * 5.0, 0.0));
        model_matrix = nalgebra_glm::rotate_y(&model_matrix, start.elapsed().as_secs_f32() * 10.0);
        view_matrix = nalgebra_glm::look_at(
            &nalgebra_glm::vec3(5.0, 0.0,  0.0), 
            &nalgebra_glm::vec3(0.0,  0.0,  0.0), 
            &nalgebra_glm::vec3(0.0,  1.0,  0.0)
        );
        proj_matrix = nalgebra_glm::perspective(1.0, 1.0, 0.01, 100.0);
            

        unsafe {
            gl::ClearColor(0./255., 0./255., 20./255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // gl::Uniform1f(u_time.id, start.elapsed().as_secs_f32());
            gl::UniformMatrix4fv(u_model_matrix.id, 1, gl::FALSE, &model_matrix.columns(0, 4)[0]);
            gl::UniformMatrix4fv(u_view_matrix.id, 1, gl::FALSE, &view_matrix.columns(0, 4)[0]);
            gl::UniformMatrix4fv(u_proj_matrix.id, 1, gl::FALSE, &proj_matrix.columns(0, 4)[0]);

            // (vertices, indices) = triangle_fan(5);
            vbo.set(&vertices);
            vao.set();
            ibo.set(&vec_u32_from_vec_u16(indices.clone()));

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

fn flatten(vertices: Vec<Vertex>) -> Vec<f32> {
    let mut retval = vec![];
    for vertex in vertices {
        retval.push(vertex.position[0]);
        retval.push(vertex.position[1]);
        retval.push(vertex.position[2]);
    };
    retval
}

fn vec_u32_from_vec_u16(input: Vec<u16>) -> Vec<u32> {
    let mut retval = vec![];
    for x in input {
        retval.push(x as u32);
    }
    retval
}

fn triangle_fan(n: u32) -> (Vec<f32>, Vec<u32>) {
    let mut vertices: Vec<f32> = vec![
        0.0, 0.0, 0.0,
        0.5, 0.0, 0.0
    ];
    let mut indices: Vec<u32> = vec![];

    let mut angle: f32;
    for m in 1..n {
        angle = 2. * PI * m as f32 / n as f32;
        vertices.push(angle.cos() * 0.5);
        vertices.push(angle.sin() * 0.5);
        vertices.push(0.0);

        indices.push(0);
        indices.push(m);
        indices.push(m+1);
    }
    indices.push(0);
    indices.push(n);
    indices.push(1);

    (vertices, indices)
}