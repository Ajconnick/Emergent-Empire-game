use std::cell::RefCell;
use std::time::Instant;

use sdl2::event::{Event, WindowEvent};
use sdl2::sys::{SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency};
use sdl2::video::{SwapInterval, Window};
use sdl2::Sdl;

use super::objects::{create_program, Uniform};

pub struct App {
    // Screen stuff
    pub screen_width: i32,
    pub screen_height: i32,
    pub sdl_context: Sdl,
    pub window: Window,

    // OpenGL stuff
    pub program_id: u32,
    u_resolution: Uniform,

    // Main loop stuff
    pub running: bool,
    pub seconds: f32, //< How many seconds the program has been up

    // User input state
    pub keys: [bool; 256],

    // Scene stack stuff
    scene_stack: Vec<RefCell<Box<dyn Scene>>>,
}

impl App {
    pub fn new() -> Result<Self, String> {
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

        let program = create_program().unwrap();
        program.set();

        let u_resolution = Uniform::new(program.id(), "u_resolution").unwrap();
        unsafe { gl::Uniform2f(u_resolution.id, screen_width as f32, screen_height as f32) }

        Ok(App {
            screen_width,
            screen_height,
            sdl_context,
            window,
            program_id: program.id(),
            u_resolution,
            running: false,
            seconds: 0.0,
            keys: [false; 256],
            scene_stack: vec![],
        })
    }

    pub fn run(&mut self) {
        self.running = true;
        let time = Instant::now();
        let mut start = time.elapsed().as_millis();
        let mut current;
        let mut previous = 0;
        let mut lag = 0;
        let mut elapsed;
        const DELTA_T: u128 = 16;

        while self.running {
            self.seconds = time.elapsed().as_secs_f32();
            current = time.elapsed().as_millis();
            elapsed = current - previous;

            previous = current;
            lag += elapsed;

            let scene_stale = false;
            while lag >= DELTA_T {
                self.poll_input();

                if let Some(scene_ref) = self.scene_stack.last() {
                    scene_ref.borrow_mut().update(&self);
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
                    gl::Viewport(0, 0, self.screen_width, self.screen_height);
                    gl::Uniform2f(
                        self.u_resolution.id,
                        self.screen_width as f32,
                        self.screen_height as f32,
                    );
                    gl::ClearColor(20. / 255., 20. / 255., 250. / 255., 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                }

                if let Some(scene_ref) = self.scene_stack.last() {
                    scene_ref.borrow_mut().render(&self);
                }
                self.window.gl_swap_window();
            }

            let end = unsafe { SDL_GetPerformanceCounter() };
            let freq = unsafe { SDL_GetPerformanceFrequency() };
            let seconds = (end as f64 - (start as f64)) / (freq as f64);
            if seconds > 5.0 {
                println!("5 seconds");
                start = end as u128;
            }
        }
    }

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

    pub fn add_scene(&mut self, scene: RefCell<Box<dyn Scene>>) {
        self.scene_stack.push(scene);
    }
}

pub trait Scene {
    // TODO: Return a "command" enum so that scene's can affect App state
    fn update(&mut self, app: &App);
    fn render(&mut self, app: &App);
}
