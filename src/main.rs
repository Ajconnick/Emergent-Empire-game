mod app;
mod camera;
mod mesh;
mod objects;
mod planet;

fn main() -> Result<(), String> {
    println!("Setting up app...");
    let mut app = app::App::new()?;

    app.run();

    Ok(())
}
