use sdl2::event::{Event, WindowEvent};
use sdl2::video::SwapInterval;

mod objects;
use objects::*;

mod mesh;
use mesh::*;

mod camera;
use camera::*;

use std::time::Instant;

use gl::types::GLuint;

fn main() -> Result <(), String> {
    let mut screen_width: i32 = 600;
    let mut screen_height: i32 = 600;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Window!", screen_width as u32, screen_height as u32)
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

    // OPEN GL SHADERS 'N SHIET!
    let program = create_program().unwrap();
    program.set();
    let u_resolution = Uniform::new(program.id(), "u_resolution").unwrap();
    unsafe { gl::Uniform2f(u_resolution.id, screen_width as f32, screen_height as f32) }

    let mesh = Mesh::new();
    let mut camera = Camera::new(
        nalgebra_glm::vec3(0.0, 0.0, 70.0),
        nalgebra_glm::vec3(0.0, 0.0, 0.0),
        nalgebra_glm::vec3(0.0, 0.0, 1.0),
        0.94, // 50mm focal length
    );

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();
    let start = Instant::now();
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
                            screen_width = new_width;
                            screen_height = new_height;
                            gl::Viewport(0, 0, new_width, new_height);
                            gl::Uniform2f(u_resolution.id, new_width as f32, new_height as f32);
                        }
                    }
                }

                _ => {},
            }
        }
        unsafe {
            gl::ClearColor(0./255., 5./255., 5./255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let t = start.elapsed().as_secs_f32();
        let planet1_pos = nalgebra_glm::vec3(
            (0.3 * t).cos() * 60.0,
            (0.3 * t).sin() * 60.0,
            0.0);
        let planet2_pos = nalgebra_glm::vec3(
            (0.02 * t + 3.1415).cos() * 30.0,
            (0.02 * t + 3.1415).sin() * 30.0,
            0.0);
        camera.position = nalgebra_glm::vec3(
            (t).cos() * 10.0 + planet1_pos.x,
            (t).sin() * 10.0 + planet1_pos.y,
            0.0,
        ); 
        camera.lookat = planet1_pos;
        draw_planet(planet1_pos, program.id(), &mesh, &camera);
        draw_planet(planet2_pos, program.id(), &mesh, &camera);

        window.gl_swap_window();
    }

    println!("Hello, world!");

    Ok(())
}

fn draw_planet(
    planet_pos: nalgebra_glm::Vec3,
    program_id: GLuint,
    mesh: &Mesh,
    camera: &Camera,
) {
    // (x, y) plane is planetary plane
    // z is up off the plane
    let mut model_matrix = nalgebra_glm::one();
    model_matrix = nalgebra_glm::translate(&model_matrix, &planet_pos);
    let (view_matrix, proj_matrix) = camera.gen_view_proj_matrices();
        
    unsafe {
        // These Uniforms allow us to pass data (ex: window size, elapsed time) to the GPU shaders
        let u_model_matrix = Uniform::new(program_id, "u_model_matrix").unwrap();
        let u_view_matrix = Uniform::new(program_id, "u_view_matrix").unwrap();
        let u_proj_matrix = Uniform::new(program_id, "u_proj_matrix").unwrap();
        let u_sun_pos = Uniform::new(program_id, "u_sun_pos_vec3").unwrap();
        gl::UniformMatrix4fv(u_model_matrix.id, 1, gl::FALSE, &model_matrix.columns(0, 4)[0]);
        gl::UniformMatrix4fv(u_view_matrix.id, 1, gl::FALSE, &view_matrix.columns(0, 4)[0]);
        gl::UniformMatrix4fv(u_proj_matrix.id, 1, gl::FALSE, &proj_matrix.columns(0, 4)[0]);
        gl::Uniform3f(u_sun_pos.id, 0., 0., 0.);

        mesh.set();

        gl::DrawElements(
            gl::TRIANGLES,
            mesh.indices_len(),
            gl::UNSIGNED_INT,
            0 as *const _,
        );
    }
}