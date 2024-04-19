mod components;
mod engine;
mod scenes;

use std::cell::RefCell;

use engine::app::*;
use scenes::r#match::Match;

fn main() -> Result<(), String> {
    run(&|app| RefCell::new(Box::new(Match::new(app.program_id))))
}
