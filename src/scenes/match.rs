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
    prev_selected_pos: nalgebra_glm::Vec3,
    selected_body_radius: f32,
    transition: f32,

    phi: f32,
    theta: f32,
    distance: f32,

    prev_enter_state: bool,
    number_planets: u32,
}

impl Scene for Match {
    fn update(&mut self, app: &App) {
        self.control(app);
        self.planet_system(app, 0);
        self.planet_system(app, 1);
        self.planet_system(app, 2);
        self.camera_update(app);
    }

    fn render(&mut self, app: &App) {
        self.planet_render_system(app);
    }
}

fn incr_num_planets(num: &mut u32) -> u32 {
    let tmp = *num;
    *num += 1;
    tmp
}

impl Match {
    pub fn new() -> Self {
        let mut world = World::new();

        let mut num_planets: u32 = 0;

        let sun_entity = world.new_entity();
        let planet_component = Planet::new(
            true,
            incr_num_planets(&mut num_planets),
            sun_entity,
            0,
            109.17,
            0.01,
            0.0,
            0.0,
            0.0,
            "res/sun.png",
            nalgebra_glm::vec3(0., 0., 0.),
            nalgebra_glm::vec3(1., 1., 1.),
        );
        world.add_component_to_entity(sun_entity, planet_component);

        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            false,
            incr_num_planets(&mut num_planets),
            sun_entity,
            1,
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
            incr_num_planets(&mut num_planets),
            sun_entity,
            1,
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
            incr_num_planets(&mut num_planets),
            sun_entity,
            1,
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
        let moon_entity = world.new_entity();
        let moon_component = Planet::new(
            false,
            incr_num_planets(&mut num_planets),
            planet_entity,
            2,
            0.272,
            60.34,
            0.0896,
            0.0749,
            0.0749,
            "res/moon.png",
            nalgebra_glm::vec3(0.0, 0.0, 0.0),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(moon_entity, moon_component);

        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            false,
            incr_num_planets(&mut num_planets),
            sun_entity,
            1,
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
            incr_num_planets(&mut num_planets),
            sun_entity,
            1,
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
        let moon_entity = world.new_entity();
        let moon_component = Planet::new(
            false,
            incr_num_planets(&mut num_planets),
            planet_entity,
            2,
            0.2858,
            66.17,
            0.0,
            0.00484,
            0.00484,
            "res/io.png",
            nalgebra_glm::vec3(0., 0., 0.),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(moon_entity, moon_component);
        let moon_entity = world.new_entity();
        let moon_component = Planet::new(
            false,
            incr_num_planets(&mut num_planets),
            planet_entity,
            2,
            0.245,
            105.01,
            0.0,
            0.00971,
            0.00971,
            "res/europa.png",
            nalgebra_glm::vec3(0., 0., 0.),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(moon_entity, moon_component);
        let moon_entity = world.new_entity();
        let moon_component = Planet::new(
            false,
            incr_num_planets(&mut num_planets),
            planet_entity,
            2,
            0.4125,
            167.85,
            0.0,
            0.01957,
            0.01957,
            "res/ganymede.png",
            nalgebra_glm::vec3(0., 0., 0.),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(moon_entity, moon_component);
        let moon_entity = world.new_entity();
        let moon_component = Planet::new(
            false,
            incr_num_planets(&mut num_planets),
            planet_entity,
            2,
            0.3798,
            295.39,
            0.0,
            0.04565,
            0.04565,
            "res/callisto.png",
            nalgebra_glm::vec3(0., 0., 0.),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(moon_entity, moon_component);

        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            true,
            incr_num_planets(&mut num_planets),
            sun_entity,
            1,
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
        let moon_entity = world.new_entity();
        let moon_component = Planet::new(
            false,
            incr_num_planets(&mut num_planets),
            planet_entity,
            2,
            0.2529,
            191.70,
            0.0,
            0.0436,
            0.0436,
            "res/titan.png",
            nalgebra_glm::vec3(0.82, 0.63, 0.82),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(moon_entity, moon_component);

        let planet_entity = world.new_entity();
        let planet_component = Planet::new(
            true,
            incr_num_planets(&mut num_planets),
            sun_entity,
            1,
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
            incr_num_planets(&mut num_planets),
            sun_entity,
            1,
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
        let moon_entity = world.new_entity();
        let moon_component = Planet::new(
            false,
            incr_num_planets(&mut num_planets),
            planet_entity,
            2,
            0.2128,
            55.75,
            0.0,
            -0.01608,
            -0.01608,
            "res/triton.png",
            nalgebra_glm::vec3(0., 0.0, 0.0),
            nalgebra_glm::vec3(0., 0., 0.),
        );
        world.add_component_to_entity(moon_entity, moon_component);

        let camera = Camera::new(
            nalgebra_glm::vec3(0.0, 0.0, 0.0),
            nalgebra_glm::vec3(0.0, 0.0, 0.0),
            nalgebra_glm::vec3(0.0, 0.0, 1.0),
            0.94, // 50mm focal length (iPhone 13 camera)
        );

        Self {
            world,
            camera,
            selection: 3,
            selected_pos: nalgebra_glm::vec3(0.0, 0.0, 0.0),
            prev_selected_pos: nalgebra_glm::vec3(0.0, 0.0, 0.0),
            transition: 1.0,
            selected_body_radius: 0.0,
            phi: 2.5,
            theta: 0.0,
            distance: 20.0,
            prev_enter_state: false,
            number_planets: num_planets,
        }
    }

    fn control(&mut self, app: &App) {
        let curr_enter_state = app.keys[Scancode::Return as usize];
        if curr_enter_state && !self.prev_enter_state {
            self.selection += 1;
            self.selected_body_radius = 0.0;
            self.prev_selected_pos = self.selected_pos;
            self.transition = app.seconds;
            if self.selection >= self.number_planets {
                self.selection = 0;
            }
        }
        self.prev_enter_state = curr_enter_state;

        let control_speed = 0.005;
        let zoom_control_speed = 0.15 * (self.distance - self.selected_body_radius);
        if app.mouse_left_down {
            self.phi -= control_speed * (app.mouse_rel_x as f32);
            self.theta = (self.theta - control_speed * (app.mouse_rel_y as f32))
                .max(control_speed - PI / 2.0)
                .min(PI / 2.0 - control_speed);
        }
        self.distance = (self.distance - zoom_control_speed * (app.mouse_wheel as f32))
            .max(self.selected_body_radius * 2.0)
            .min(self.selected_body_radius * 3.0 + 234.0);
    }

    /// Goes through each planet and updates it's position
    fn planet_system(&mut self, app: &App, tier: u32) {
        let mut planets = self.world.borrow_component_vec::<Planet>().unwrap();
        let planet_pos: Vec<Option<nalgebra_glm::Vec3>> = planets
            .iter()
            .map(|p| {
                if let Some(planet) = p {
                    Some(planet.position)
                } else {
                    None
                }
            })
            .collect();
        let iter = planets.iter_mut().filter_map(|p| {
            if p.is_some() && p.as_ref().unwrap().tier == tier {
                Some(p.as_mut()?)
            } else {
                None
            }
        });

        for planet in iter {
            planet.update(app.seconds, planet_pos[planet.parent_entity_id].unwrap());
            if planet.id == self.selection {
                self.selected_pos = planet.position;
                self.selected_body_radius = planet.body_radius;
            }
        }
    }

    fn camera_update(&mut self, _app: &App) {
        let rot_matrix = nalgebra_glm::rotate_y(
            &nalgebra_glm::rotate_z(&nalgebra_glm::one(), self.phi),
            self.theta,
        );
        self.camera.position = (rot_matrix * nalgebra_glm::vec4(self.distance, 0., 0., 0.)).xyz();
    }

    fn planet_render_system(&mut self, app: &App) {
        let mut planets = self.world.borrow_component_vec::<Planet>().unwrap();
        let iter = planets.iter_mut().filter_map(|p| Some(p.as_mut()?));
        let transition = cubic_ease_out((app.seconds - self.transition).min(1.0));
        let offset = (1.0 - transition) * self.prev_selected_pos + transition * self.selected_pos;
        for planet in iter {
            planet.draw(
                (app.screen_height as f32).min(app.screen_width as f32),
                app.program_id,
                &self.camera,
                offset,
            );
        }
    }
}

fn cubic_ease_out(t: f32) -> f32 {
    // Cubic easing out function
    1.0 - (1.0 - t).powf(30.0)
}
