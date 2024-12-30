mod components;
mod scenes;

use std::cell::RefCell;

use apricot::app::run;
use scenes::gameplay::Gameplay;

fn main() -> Result<(), String> {
    run(
        nalgebra_glm::I32Vec2::new(800, 600),
        "Survival Prototype",
        &|app| RefCell::new(Box::new(Gameplay::new(app))),
    )
}
