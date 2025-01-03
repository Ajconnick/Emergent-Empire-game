//! This module is responsible for defining the gameplay scene.

use std::f32::consts::PI;

use apricot::{
    app::{App, Scene},
    bvh::BVH,
    camera::{Camera, ProjectionKind},
    opengl::create_program,
    render_core::ModelComponent,
    shadow_map::DirectionalLightSource,
};
use hecs::{Entity, World};
use sdl2::keyboard::Scancode;

use crate::components::planet::Planet;

/// Object file data, used for meshes
pub const QUAD_XY_DATA: &[u8] = include_bytes!("../../res/quad-xy.obj");
pub const ICO_DATA: &[u8] = include_bytes!("../../res/ico-sphere.obj");
pub const UV_DATA: &[u8] = include_bytes!("../../res/uv-sphere.obj");

/// Struct that contains info about the game state
pub struct Gameplay {
    /// The world where all the entities live
    world: World,
    /// The camera used for rendering 3d models
    camera_3d: Camera,
    /// The sun's light source
    directional_light: DirectionalLightSource,
    /// A bounding-volume hierarchy, a container that stores models and allows for efficient lookup for fast rendering
    bvh: BVH<Entity>,

    /// Which planetary body is currently selected
    selection: usize,
    /// The radius of the currently selected planetary body, for limiting zoom
    selected_body_radius: f32,
    /// The position of the selected planetary body, used for swoosh animation
    selected_pos: nalgebra_glm::Vec3,
    /// The prev selected position, used for swoosh animation
    prev_selected_pos: nalgebra_glm::Vec3,
    /// Animation key frame counter
    transition: f32,

    /// Up-down view angle
    phi: f32,
    /// Side-side view angle
    theta: f32,
    /// How far the camera swivels around the currently selected body
    distance: f32,

    /// Used for enter key latch
    prev_enter_state: bool,
    /// How many planets there are
    number_planets: usize,
}

impl Scene for Gameplay {
    /// Update the scene every tick
    fn update(&mut self, app: &App) {
        self.control(app);
        self.planet_system(app, 0);
        self.planet_system(app, 1);
        self.planet_system(app, 2);
        self.camera_update(app);
    }

    /// Render the scene to the screen when time allows
    fn render(&mut self, app: &App) {
        app.renderer.set_camera(self.camera_3d);
        app.renderer.directional_light_system(
            &mut self.directional_light,
            &mut self.world,
            &self.bvh,
        );
        app.renderer.render_3d_models_system(
            &mut self.world,
            &self.directional_light,
            &self.bvh,
            false,
        );

        for (_entity, planet) in self.world.query::<&Planet>().iter() {
            if planet.id == self.selection {
                let font = app.renderer.get_font_id_from_name("font").unwrap();
                app.renderer.set_font(font);
                app.renderer
                    .draw_text(nalgebra_glm::vec2(10.0, 10.0), &planet.name);
            }
        }
    }
}

/// Increments a counter and returns the previous value
fn incr_num_planets(num: &mut usize) -> usize {
    let tmp = *num;
    *num += 1;
    tmp
}

impl Gameplay {
    /// Constructs a new Gameplay struct with everything setup
    /// TODO: Most of this stuff will need to be moved to the init scene. Remind me to make an issue for this!
    pub fn new(app: &App) -> Self {
        let mut world = World::new();

        // Add programs to the renderer
        app.renderer.add_program(
            create_program(
                include_str!("../shaders/3d.vert"),
                include_str!("../shaders/3d.frag"),
            )
            .unwrap(),
            Some("3d"),
        );
        app.renderer.add_program(
            create_program(
                include_str!("../shaders/2d.vert"),
                include_str!("../shaders/2d.frag"),
            )
            .unwrap(),
            Some("2d"),
        );
        app.renderer.add_program(
            create_program(
                include_str!("../shaders/shadow.vert"),
                include_str!("../shaders/shadow.frag"),
            )
            .unwrap(),
            Some("shadow"),
        );
        app.renderer.add_program(
            create_program(
                include_str!("../shaders/2d.vert"),
                include_str!("../shaders/solid-color.frag"),
            )
            .unwrap(),
            Some("2d-solid"),
        );
        app.renderer.add_program(
            create_program(
                include_str!("../shaders/3d.vert"),
                include_str!("../shaders/solid-color.frag"),
            )
            .unwrap(),
            Some("3d-solid"),
        );

        // Setup the mesh manager
        app.renderer
            .add_mesh_from_obj(QUAD_XY_DATA, Some("quad-xy"));
        app.renderer.add_mesh_from_obj(UV_DATA, Some("uv"));
        app.renderer.add_mesh_from_obj(ICO_DATA, Some("ico"));

        // Setup the texture manager
        app.renderer
            .add_texture_from_png("res/sun.png", Some("sun"));
        app.renderer
            .add_texture_from_png("res/earth.png", Some("earth"));
        app.renderer
            .add_texture_from_png("res/moon.png", Some("moon"));

        // Setup the font manager
        app.renderer
            .add_font("res/Consolas.ttf", "font", 16, sdl2::ttf::FontStyle::NORMAL);

        let mut bvh = BVH::<Entity>::new();

        let mut num_planets: usize = 0;
        let sun_planet_id = Planet::new(
            &mut world,
            &app.renderer,
            &mut bvh,
            true,
            incr_num_planets(&mut num_planets),
            0,
            0,
            109.17,
            0.01,
            0.0,
            0.0,
            app.renderer.get_texture_id_from_name("sun").unwrap(),
            "Sun",
        );

        let planet_planet_id = Planet::new(
            &mut world,
            &app.renderer,
            &mut bvh,
            false,
            incr_num_planets(&mut num_planets),
            sun_planet_id,
            1,
            1.,
            23486.0,
            1.0,
            0.0027,
            app.renderer.get_texture_id_from_name("earth").unwrap(),
            "Earth",
        );

        let _moon_planet_id = Planet::new(
            &mut world,
            &app.renderer,
            &mut bvh,
            false,
            incr_num_planets(&mut num_planets),
            planet_planet_id,
            2,
            0.272,
            60.34,
            0.0749,
            0.0749,
            app.renderer.get_texture_id_from_name("moon").unwrap(),
            "Moon",
        );

        Self {
            world,
            camera_3d: Camera::new(
                nalgebra_glm::vec3(1.0, 0.0, 1.0),
                nalgebra_glm::vec3(0.0, 0.0, 0.0),
                nalgebra_glm::vec3(0.0, 0.0, 1.0),
                ProjectionKind::Perspective {
                    fov: 0.65,
                    far: 10000000.0,
                },
            ),
            bvh,
            directional_light: DirectionalLightSource::new(
                Camera::new(
                    nalgebra_glm::vec3(0.0, 0.0, 0.0),
                    nalgebra_glm::vec3(0.0, 10.0, 0.0),
                    nalgebra_glm::vec3(0.0, 0.0, 1.0),
                    ProjectionKind::Orthographic {
                        // These do not matter for now, they're reset later
                        left: 0.0,
                        right: 0.0,
                        bottom: 0.0,
                        top: 0.0,
                        near: 0.0,
                        far: 0.0,
                    },
                ),
                nalgebra_glm::vec3(-1.0, 0.0, 0.0),
                1024,
            ),

            selection: 1,
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

    /// Changes various game state based on user mouse and keyboard input
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
            self.phi -= control_speed * (app.mouse_vel.x as f32);
            self.theta = (self.theta - control_speed * (app.mouse_vel.y as f32))
                .max(control_speed - PI / 2.0)
                .min(PI / 2.0 - control_speed);
        }
        self.distance = (self.distance - zoom_control_speed * (app.mouse_wheel as f32))
            .max(self.selected_body_radius * 2.0)
            .min(self.selected_body_radius * 3.0 + 234.0);
    }

    /// Updates planets based on their on-rails orbits around their parent bodies
    fn planet_system(&mut self, app: &App, tier: u32) {
        let planet_pos: Vec<nalgebra_glm::Vec3> = self
            .world
            .query::<(&Planet, &ModelComponent)>()
            .iter()
            .map(|(_enitity, (_planet, model))| model.get_position())
            .collect();

        for (_entity, (model, planet)) in
            self.world.query_mut::<(&mut ModelComponent, &mut Planet)>()
        {
            if planet.tier != tier {
                continue;
            }

            const REAL_SECS_PER_GAME_YEAR: f32 = 600.0; // How many real seconds it takes for earth to go around the sun once
            const T_SEED: f32 = 98400.0; // An offset from t, so that the planets are not all in a line.
            let t = app.seconds;
            let parent_pos = planet_pos[planet.parent_planet_id];

            if planet.tier != 0 {
                let new_pos = nalgebra_glm::vec3(
                    (2.0 * PI * (t + T_SEED)
                        / (REAL_SECS_PER_GAME_YEAR * planet.orbital_time_years))
                        .cos()
                        * planet.orbital_radius
                        + parent_pos.x,
                    (2.0 * PI * (t + T_SEED)
                        / (REAL_SECS_PER_GAME_YEAR * planet.orbital_time_years))
                        .sin()
                        * planet.orbital_radius
                        + parent_pos.y,
                    0.0,
                );
                let vel = new_pos - model.get_position();
                model.set_position(new_pos);
                self.bvh.move_obj(
                    planet.bvh_node_id,
                    &app.renderer.get_model_aabb(&model),
                    &vel,
                );
            }
            if planet.day_time_years != 0.0 {
                planet.rotation = 2.0 * PI * (t + T_SEED)
                    / (REAL_SECS_PER_GAME_YEAR * planet.day_time_years)
                    + 3.14;
            }

            if planet.id == self.selection {
                self.selected_pos = model.get_position();
                self.selected_body_radius = planet.body_radius;
            }
        }
    }

    /// Updates the camera position and lookat based on mouse panning and body selection
    fn camera_update(&mut self, app: &App) {
        let rot_matrix = nalgebra_glm::rotate_y(
            &nalgebra_glm::rotate_z(&nalgebra_glm::one(), self.phi),
            self.theta,
        );
        let transition = cubic_ease_out((app.seconds - self.transition).min(1.0));
        let offset = (1.0 - transition) * self.prev_selected_pos + transition * self.selected_pos;
        // TODO: It's convenient to have planet model space be opengl-model-space, but there's a lot of floating point jitter.
        //       It also affects lighting and makes it weird, which is why it's turned off for this PR.
        //       We'll need opengl-model-space be centered at the selected body's position.
        self.camera_3d.set_position(
            (rot_matrix * nalgebra_glm::vec4(self.distance, 0., 0., 0.)).xyz() + offset,
        );
        self.camera_3d.set_lookat(self.selected_pos);
    }
}

/// Cubic easing out function - for animation
fn cubic_ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t).powf(30.0)
}
