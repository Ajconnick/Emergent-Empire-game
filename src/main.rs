use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Scancode;
use sdl2::video::SwapInterval;

mod objects;
use objects::*;

mod mesh;

mod camera;
use camera::*;

mod planet;
use planet::*;

use std::f32::consts::PI;
use std::time::Instant;

fn main() -> Result<(), String> {
    let mut screen_width: i32 = 800;
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

    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

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

    let merucry = Planet::new(
        program.id(),
        0.38,
        90.910,
        "res/mercury.png",
        nalgebra_glm::vec3(0., 0., 0.),
    );
    let venus = Planet::new(
        program.id(),
        0.9499,
        169.878,
        "res/venus.png",
        nalgebra_glm::vec3(1., 0.9, 0.7),
    );
    let earth = Planet::new(
        program.id(),
        1.,
        234.866,
        "res/earth.png",
        nalgebra_glm::vec3(0.8, 0.9, 1.),
    );
    let mars = Planet::new(
        program.id(),
        0.533,
        352.198,
        "res/mars.png",
        nalgebra_glm::vec3(1., 0.45, 0.25),
    );
    let jupiter = Planet::new(
        program.id(),
        10.973,
        1222.14,
        "res/jupiter.png",
        nalgebra_glm::vec3(1.5, 1.3, 0.88),
    );
    let mut planets = vec![merucry, venus, earth, mars, jupiter];
    let mut camera = Camera::new(
        nalgebra_glm::vec3(0.0, 0.0, 70.0),
        nalgebra_glm::vec3(0.0, 0.0, 0.0),
        nalgebra_glm::vec3(0.0, 0.0, 1.0),
        0.94, // 50mm focal length (iPhone 13 camera)
    );

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();
    let start = Instant::now();
    // User input
    let mut keys: [bool; 256] = [false; 256];
    let mut prev_enter_state = keys[Scancode::Return as usize];
    let mut angle: f32 = 2.5;
    let mut phi: f32 = 0.0;
    let mut distance: f32 = 0.06;
    let mut selection: usize = 0;
    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }

                Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    println!("Mouse x: {}, y: {}", x, y);
                    println!("Relative x: {}, y: {}", xrel, yrel);
                }

                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(new_width, new_height) = win_event {
                        screen_width = new_width;
                        screen_height = new_height;
                    }
                }

                Event::KeyDown { scancode, .. } => match scancode {
                    Some(sc) => {
                        keys[sc as usize] = true;
                        println!("{}!", sc);
                    }
                    None => {}
                },

                Event::KeyUp { scancode, .. } => match scancode {
                    Some(sc) => keys[sc as usize] = false,
                    None => {}
                },

                _ => {}
            }
        }
        unsafe {
            gl::Viewport(0, 0, screen_width, screen_height);
            gl::Uniform2f(u_resolution.id, screen_width as f32, screen_height as f32);
            gl::ClearColor(0. / 255., 0. / 255., 0. / 255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let control_speed = 0.05;
        if keys[Scancode::Left as usize] {
            angle += control_speed;
        }
        if keys[Scancode::Right as usize] {
            angle -= control_speed;
        }
        if keys[Scancode::Up as usize] {
            phi = (phi - control_speed).max(control_speed - PI / 2.0);
        }
        if keys[Scancode::Down as usize] {
            phi = (phi + control_speed).min(PI / 2.0 - control_speed);
        }
        let curr_enter_state = keys[Scancode::Return as usize];
        if curr_enter_state && !prev_enter_state {
            selection += 1;
            if selection >= planets.len() {
                selection = 0;
            }
        }
        prev_enter_state = curr_enter_state;

        // Update planet motion
        let t = start.elapsed().as_secs_f32();
        for planet in &mut planets.iter_mut() {
            planet.update(t);
        }

        // Update camera position and look-at
        let orbit_height = 200.0 / 3958.0;
        let orbit_pos = planets[selection].pos()
            + nalgebra_glm::vec3(
                (YEAR_SPEED * t + 3.5).cos() * (planets[selection].body_radius + orbit_height),
                (YEAR_SPEED * t + 3.5).sin() * (planets[selection].body_radius + orbit_height),
                0.,
            );
        let rot_matrix =
            nalgebra_glm::rotate_y(&nalgebra_glm::rotate_z(&nalgebra_glm::one(), angle), phi);
        camera.position = orbit_pos + (rot_matrix * nalgebra_glm::vec4(distance, 0., 0., 0.)).xyz();
        camera.lookat = orbit_pos; // And look at it!

        // Draw planets
        for planet in &planets {
            planet.draw(program.id(), &camera);
        }

        window.gl_swap_window();
    }

    println!("Hello, world!");

    Ok(())
}
