use std::{cell::RefCell, rc::Rc};

use app::{App, Scene};

mod app;
mod camera;
mod mesh;
mod objects;
mod planet;
mod world;

fn main() -> Result<(), String> {
    println!("Setting up app...");
    let mut app = App::new()?;

    app.add_scene(RefCell::new(Box::new(TestScene { i: 4 })));
    app.run();

    Ok(())
}

struct TestScene {
    i: i32,
}

impl Scene for TestScene {
    fn update(&mut self, app: &App) {
        println!("Updated!\n");
    }

    fn render(&mut self, app: &App) {
        println!("Rendered!\n");
    }
}
