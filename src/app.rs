use std::time::Instant;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Scancode;
use sdl2::sys::{SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency};
use sdl2::video::{SwapInterval, Window};
use sdl2::Sdl;

use crate::objects::*;

pub struct App {
    screen_width: i32,
    screen_height: i32,
    sdl_context: Sdl,
    window: Window,
    program_id: u32,
    running: bool,
    keys: [bool; 256],
}

impl App {
    pub fn new() -> Result<Self, String> {
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

        let _gl = gl::load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        window
            .subsystem()
            .gl_set_swap_interval(SwapInterval::VSync)
            .unwrap();

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
            running: false,
            keys: [false; 256],
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
            current = time.elapsed().as_millis();
            elapsed = current - previous;

            previous = current;
            lag += elapsed;

            let scene_stale = false;
            while lag >= DELTA_T {
                self.poll_input();

                // scene.update()

                if !scene_stale {
                    // if scene isn't stale, purge the scene
                    lag -= DELTA_T;
                } else {
                    break;
                }
            }

            if !scene_stale {
                // scene.render()
                self.window.gl_swap_window();
            }

            let end = unsafe { SDL_GetPerformanceCounter() };
            let freq = unsafe { SDL_GetPerformanceFrequency() };
            let seconds = (end as f64 - (start as f64)) / (freq as f64);
            if seconds > 5.0 {
                println!("Tick!");
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
}
