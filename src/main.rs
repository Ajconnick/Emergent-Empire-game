mod components;
mod scenes;

use std::cell::RefCell;

use apricot::app::run;
use scenes::gameplay::Gameplay;

fn main() -> Result<(), String> {
    // Start Apricot's game loop
    run(
        nalgebra_glm::I32Vec2::new(800, 600),
        "Emergent Empire", // singular, empire
        &|app| RefCell::new(Box::new(Gameplay::new(app))),
    )
}
