use engine::app::App;

mod components;
mod engine;
mod scenes;

fn main() -> Result<(), String> {
    println!("Setting up app...");
    let mut app = App::new()?;

    // let match_scene = Match::new(app.program_id);
    // app.add_scene(RefCell::new(Box::new(match_scene)));
    app.run();

    Ok(())
}
