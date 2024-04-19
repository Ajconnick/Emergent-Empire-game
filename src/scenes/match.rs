use crate::{
    components::planet::Planet,
    engine::{app::*, camera::Camera, world::*},
};

pub struct Match {
    world: World,

    camera: Camera,
}

impl Scene for Match {
    fn update(&mut self, app: &App) {
        self.planet_system(app);
    }

    fn render(&mut self, app: &App) {
        self.planet_render_system(app);
    }
}

impl Match {
    pub fn new(program_id: u32) -> Self {
        let mut world = World::new();

        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            program_id,
            1.,
            27.0,
            "res/earth.png",
            nalgebra_glm::vec3(0.8, 0.9, 1.),
        );

        let camera = Camera::new(
            nalgebra_glm::vec3(0.0, 30.0, 00.0),
            nalgebra_glm::vec3(0.0, 0.0, 0.0),
            nalgebra_glm::vec3(0.0, 0.0, 1.0),
            0.94, // 50mm focal length (iPhone 13 camera)
        );
        world.add_component_to_entity(planet_entity, planet_component);

        Self { world, camera }
    }

    /// Goes through each planet and updates it's position
    fn planet_system(&mut self, app: &App) {
        let mut planets = self.world.borrow_component_vec::<Planet>().unwrap();
        let iter = planets.iter_mut().filter_map(|p| Some(p.as_mut()?));
        for planet in iter {
            planet.update(app.seconds);
            if planet.body_radius == 1.0 {
                self.camera.lookat = planet.position;
            }
        }
    }

    fn planet_render_system(&mut self, app: &App) {
        let mut planets = self.world.borrow_component_vec::<Planet>().unwrap();
        let iter = planets.iter_mut().filter_map(|p| Some(p.as_mut()?));
        for planet in iter {
            planet.draw(app.program_id, &self.camera);
        }
    }
}
