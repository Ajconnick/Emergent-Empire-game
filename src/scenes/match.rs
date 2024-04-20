use std::f32::consts::PI;

use sdl2::keyboard::Scancode;

use crate::{
    components::planet::Planet,
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
    pub fn new() -> Self {
        let mut world = World::new();

        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            true,
            109.17,
            0.01,
            0.0,
            0.0,
            0.0,
            "res/sun.png",
            nalgebra_glm::vec3(0., 0., 0.),
            nalgebra_glm::vec3(1., 1., 1.),
        );
        world.add_component_to_entity(planet_entity, planet_component);
        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            false,
            0.38,
            9084.20,
            0.0005,
            0.24,
            0.16,
            "res/mercury.png",
            nalgebra_glm::vec3(0., 0., 0.),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(planet_entity, planet_component);
        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            false,
            0.9499,
            16990.86,
            3.096,
            0.62,
            0.67,
            "res/venus.png",
            nalgebra_glm::vec3(1., 0.78, 0.62),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(planet_entity, planet_component);
        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            false,
            1.,
            23486.60,
            0.4091,
            1.0,
            0.0027,
            "res/earth.png",
            nalgebra_glm::vec3(0.64, 0.83, 1.),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(planet_entity, planet_component);
        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            false,
            0.533,
            35219.80,
            0.4392,
            1.88,
            0.0027,
            "res/mars.png",
            nalgebra_glm::vec3(1., 0.29, 0.23),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(planet_entity, planet_component);
        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            true,
            10.973,
            122273.60,
            0.0546,
            11.86,
            0.00038,
            "res/jupiter.png",
            nalgebra_glm::vec3(1., 0.8, 0.73),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(planet_entity, planet_component);
        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            true,
            9.14,
            224025.82,
            0.466,
            29.46,
            0.00041,
            "res/saturn.png",
            nalgebra_glm::vec3(1., 0.92, 0.8),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(planet_entity, planet_component);
        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            true,
            3.98,
            449310.14,
            1.707,
            84.02,
            0.00062,
            "res/uranus.png",
            nalgebra_glm::vec3(0.64, 0.83, 1.),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(planet_entity, planet_component);
        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            true,
            3.86,
            706000.76,
            0.494,
            164.79,
            0.00058,
            "res/neptune.png",
            nalgebra_glm::vec3(0.27, 0.51, 1.),
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
            distance: 20.0,
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
        let curr_enter_state = app.keys[Scancode::Return as usize];
        if curr_enter_state && !self.prev_enter_state {
            self.selection += 1;
            self.selected_body_radius *= 10000000.0;
            if self.selection >= self.number_planets {
                self.selection = 0;
            }
        }
        self.prev_enter_state = curr_enter_state;

        let control_speed = 0.005;
        let zoom_control_speed = 0.5 * self.selected_body_radius;
        if app.mouse_left_down {
            self.phi -= control_speed * (app.mouse_rel_x as f32);
            self.theta = (self.theta - control_speed * (app.mouse_rel_y as f32))
                .max(control_speed - PI / 2.0)
                .min(PI / 2.0 - control_speed);
        }
        self.distance = (self.distance - zoom_control_speed * (app.mouse_wheel as f32))
            .max(self.selected_body_radius + 1.0)
            .min(self.selected_body_radius * 10.0);

        let rot_matrix = nalgebra_glm::rotate_y(
            &nalgebra_glm::rotate_z(&nalgebra_glm::one(), self.phi),
            self.theta,
        );
        self.camera.position =
            self.selected_pos + (rot_matrix * nalgebra_glm::vec4(self.distance, 0., 0., 0.)).xyz();
        self.camera.lookat = self.selected_pos; // And look at it!
    }

    fn planet_render_system(&mut self, app: &App) {
        let mut planets = self.world.borrow_component_vec::<Planet>().unwrap();
        let iter = planets.iter_mut().filter_map(|p| Some(p.as_mut()?));
        for planet in iter {
            planet.draw(app.program_id, &self.camera);
        }
    }
}
