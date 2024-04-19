use std::cell::RefCell;
use std::time::Instant;

use sdl2::event::{Event, WindowEvent};
use sdl2::sys::{SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency};
use sdl2::video::SwapInterval;
use sdl2::Sdl;

use crate::components::planet::Planet;
use crate::engine::camera::Camera;

use super::objects::{create_program, Uniform};

pub struct App {
    // Screen stuff
    pub screen_width: i32,
    pub screen_height: i32,
    pub sdl_context: Sdl,

    // OpenGL stuff
    pub program_id: u32,
    // u_resolution: Uniform,

    // Main loop stuff
    pub running: bool,
    pub seconds: f32, //< How many seconds the program has been up

    // User input state
    pub keys: [bool; 256],

    // Scene stack stuff
    scene_stack: Vec<RefCell<Box<dyn Scene>>>,
}

pub fn run(init: &dyn Fn(&App) -> RefCell<Box<dyn Scene>>) -> Result<(), String> {
    let screen_width: i32 = 800;
    let screen_height: i32 = 600;

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

    let program = create_program().unwrap();
    program.set();
    // let u_resolution = Uniform::new(program.id(), "u_resolution").unwrap();
    // unsafe { gl::Uniform2f(u_resolution.id, screen_width as f32, screen_height as f32) }

    let mut app = App {
        screen_width,
        screen_height,
        sdl_context,
        program_id: program.id(),
        // u_resolution,
        running: true,
        keys: [false; 256],
        seconds: 0.0,
        scene_stack: Vec::new(),
    };

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
        nalgebra_glm::vec3(0.0, 0.0, 00.0),
        planets[1].position,
        nalgebra_glm::vec3(0.0, 0.0, 1.0),
        0.94, // 50mm focal length (iPhone 13 camera)
    );

    let initial_scene = init(&app);
    app.scene_stack.push(initial_scene);

    let time = Instant::now();
    let mut start = time.elapsed().as_millis();
    let mut current;
    let mut previous = 0;
    let mut lag = 0;
    let mut elapsed;
    const DELTA_T: u128 = 16;
    while app.running {
        app.seconds = time.elapsed().as_secs_f32();
        current = time.elapsed().as_millis();
        elapsed = current - previous;

        previous = current;
        lag += elapsed;

        let scene_stale = false;
        while lag >= DELTA_T {
            app.poll_input();

            if let Some(scene_ref) = app.scene_stack.last() {
                scene_ref.borrow_mut().update(&app);
            }

            if !scene_stale {
                // if scene isn't stale, purge the scene
                lag -= DELTA_T;
            } else {
                break;
            }
        }

        if !scene_stale {
            unsafe {
                // gl::Viewport(0, 0, app.screen_width, app.screen_height);
                // gl::Uniform2f(
                //     app.u_resolution.id,
                //     app.screen_width as f32,
                //     app.screen_height as f32,
                // );
                gl::ClearColor(0. / 255., 0. / 255., 20. / 255., 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }
            if let Some(scene_ref) = app.scene_stack.last() {
                scene_ref.borrow_mut().render(&app);
            }
            window.gl_swap_window();
        }

        // Draw planets
        for planet in &planets {
            planet.draw(program.id(), &camera);
        }

        let end = unsafe { SDL_GetPerformanceCounter() };
        let freq = unsafe { SDL_GetPerformanceFrequency() };
        let seconds = (end as f64 - (start as f64)) / (freq as f64);
        if seconds > 5.0 {
            println!("5 seconds");
            start = end as u128;
        }
    }
    Ok(())
}

impl App {
    fn poll_input(&mut self) {
        let mut event_queue = self.sdl_context.event_pump().unwrap();
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.running = false;
                }

                Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    _ = (x, y, xrel, yrel);
                }

                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(new_width, new_height) = win_event {
                        self.screen_width = new_width;
                        self.screen_height = new_height;
                    }
                }

                Event::KeyDown { scancode, .. } => match scancode {
                    Some(sc) => {
                        self.keys[sc as usize] = true;
                    }
                    None => {}
                },

                Event::KeyUp { scancode, .. } => match scancode {
                    Some(sc) => self.keys[sc as usize] = false,
                    None => {}
                },

                _ => {}
            }
        }
    }
}

pub trait Scene {
    // TODO: Return a "command" enum so that scene's can affect App state
    fn update(&mut self, app: &App);
    fn render(&mut self, app: &App);
}
