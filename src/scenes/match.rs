use std::f32::consts::PI;

use sdl2::keyboard::Scancode;

use crate::{
    components::planet::{Planet, YEAR_SPEED},
    engine::{app::*, camera::Camera, world::*},
};

pub struct Match {
    world: World,

    camera: Camera,

    selection: u32,
    selected_pos: nalgebra_glm::Vec3,
    selected_body_radius: f32,

    phi: f32,
    theta: f32,
    distance: f32,

    prev_enter_state: bool,
    number_planets: u32,
}

impl Scene for Match {
    fn update(&mut self, app: &App) {
        self.planet_system(app);
        self.camera_update(app);
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
            89.38,
            0.01,
            "res/sun.png",
            nalgebra_glm::vec3(0., 0., 0.),
            nalgebra_glm::vec3(1., 1., 1.),
        );
        world.add_component_to_entity(planet_entity, planet_component);
        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            program_id,
            0.38,
            90.910,
            "res/mercury.png",
            nalgebra_glm::vec3(0., 0., 0.),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(planet_entity, planet_component);
        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            program_id,
            0.9499,
            169.878,
            "res/venus.png",
            nalgebra_glm::vec3(1., 0.9, 0.7),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(planet_entity, planet_component);
        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            program_id,
            1.,
            234.866,
            "res/earth.png",
            nalgebra_glm::vec3(0.8, 0.9, 1.),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(planet_entity, planet_component);
        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            program_id,
            0.533,
            352.198,
            "res/mars.png",
            nalgebra_glm::vec3(1., 0.45, 0.25),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(planet_entity, planet_component);
        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            program_id,
            10.973,
            1222.14,
            "res/jupiter.png",
            nalgebra_glm::vec3(1.5, 1.3, 0.88),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(planet_entity, planet_component);

        let camera = Camera::new(
            nalgebra_glm::vec3(0.0, 0.0, 0.0),
            nalgebra_glm::vec3(0.0, 0.0, 0.0),
            nalgebra_glm::vec3(0.0, 0.0, 1.0),
            0.94, // 50mm focal length (iPhone 13 camera)
        );

        Self {
            world,
            camera,
            selection: 0,
            selected_pos: nalgebra_glm::vec3(0.0, 0.0, 0.0),
            selected_body_radius: 0.0,
            phi: 2.5,
            theta: 0.0,
            distance: 0.02,
            prev_enter_state: false,
            number_planets: 0,
        }
    }

    /// Goes through each planet and updates it's position
    fn planet_system(&mut self, app: &App) {
        let mut planets = self.world.borrow_component_vec::<Planet>().unwrap();
        let iter = planets
            .iter_mut()
            .filter_map(|p| Some(p.as_mut()?))
            .zip(0..);
        self.number_planets = 0;
        for (planet, i) in iter {
            planet.update(app.seconds);
            if i == self.selection {
                self.selected_pos = planet.position;
                self.selected_body_radius = planet.body_radius;
            }
            self.number_planets += 1;
        }
    }

    fn camera_update(&mut self, app: &App) {
        let control_speed = 0.05;
        if app.keys[Scancode::Left as usize] {
            self.phi += control_speed;
        }
        if app.keys[Scancode::Right as usize] {
            self.phi -= control_speed;
        }
        if app.keys[Scancode::Up as usize] {
            self.theta = (self.theta - control_speed).max(control_speed - PI / 2.0);
        }
        if app.keys[Scancode::Down as usize] {
            self.theta = (self.theta + control_speed).min(PI / 2.0 - control_speed);
        }
        let curr_enter_state = app.keys[Scancode::Return as usize];
        if curr_enter_state && !self.prev_enter_state {
            self.selection += 1;
            if self.selection >= self.number_planets {
                self.selection = 0;
            }
        }
        self.prev_enter_state = curr_enter_state;

        let orbit_height = 100.0 / 3958.0;
        let orbit_pos = self.selected_pos
            + nalgebra_glm::vec3(
                (0.001 * YEAR_SPEED * app.seconds + 3.5).cos()
                    * (self.selected_body_radius + orbit_height),
                (0.001 * YEAR_SPEED * app.seconds + 3.5).sin()
                    * (self.selected_body_radius + orbit_height),
                0.,
            );
        let rot_matrix = nalgebra_glm::rotate_y(
            &nalgebra_glm::rotate_z(&nalgebra_glm::one(), self.phi),
            self.theta,
        );
        self.camera.position =
            orbit_pos + (rot_matrix * nalgebra_glm::vec4(self.distance, 0., 0., 0.)).xyz();
        self.camera.lookat = orbit_pos; // And look at it!
    }

    fn planet_render_system(&mut self, app: &App) {
        let mut planets = self.world.borrow_component_vec::<Planet>().unwrap();
        let iter = planets.iter_mut().filter_map(|p| Some(p.as_mut()?));
        for planet in iter {
            planet.draw(app.program_id, &self.camera);
        }
    }
}
