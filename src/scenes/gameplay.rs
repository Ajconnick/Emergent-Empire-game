//! This module is responsible for defining the gameplay scene.

use std::{collections::HashMap, f32::consts::PI, sync::Arc};

use apricot::{
    app::{App, Scene},
    bvh::BVH,
    camera::{Camera, ProjectionKind},
    opengl::create_program,
    rectangle::Rectangle,
    render_core::{LinePathComponent, ModelComponent},
    shadow_map::DirectionalLightSource,
};
use hecs::{Entity, World};
use sdl2::keyboard::Scancode;

use crate::components::{
    button::{Button, Event, EventQueue},
    planet::Planet,
};

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
    bodies: Vec<Entity>,

    event_queue: Arc<EventQueue>,

    turn: usize,
}

impl Scene for Gameplay {
    /// Update the scene every tick
    fn update(&mut self, app: &App) {
        // Have an event queue, go through all GUI elements that trigger events, then loop through all events and
        // process them
        for (_entity, button) in self.world.query_mut::<&mut Button>() {
            button.update(app);
        }

        while let Some(event) = self.event_queue.pop() {
            match event {
                Event::ButtonClicked(id) => match id {
                    "next-turn" => {
                        self.turn += 1;
                        println!("doing the next turn!")
                    }
                    _ => panic!("unknown button id: {:?}", id),
                },
            }
        }

        self.control(app);
        self.planet_system(app, 0);
        self.planet_system(app, 1);
        self.planet_system(app, 2);
        self.orbit_system(app);
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

        let font = app.renderer.get_font_id_from_name("font").unwrap();
        app.renderer.set_font(font);
        for (entity, planet) in self.world.query::<&Planet>().iter() {
            if entity == self.bodies[self.selection] {
                app.renderer
                    .draw_text(nalgebra_glm::vec2(10.0, 10.0), &planet.name);
            }
        }

        for (_entity, button) in self.world.query_mut::<&mut Button>() {
            button.render(app);
        }

        app.renderer.draw_text(
            nalgebra_glm::vec2(
                app.window_size.x as f32 - 90.0,
                app.window_size.y as f32 - 20.0,
            ),
            format!("turn: {}", self.turn).to_string().as_str(),
        );

        app.renderer.render_3d_line_paths(&self.world);
    }
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
        app.renderer.add_program(
            create_program(
                include_str!("../shaders/line.vert"),
                include_str!("../shaders/solid-color.frag"),
            )
            .unwrap(),
            Some("line"),
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
        app.renderer
            .add_texture_from_png("res/next-turn.png", Some("next-turn"));
        app.renderer
            .add_texture_from_png("res/next-turn-hover.png", Some("next-turn-hover"));

        // Setup the font manager
        app.renderer
            .add_font("res/Consolas.ttf", "font", 16, sdl2::ttf::FontStyle::NORMAL);

        let mut bvh = BVH::<Entity>::new();

        let sun_entity = Planet::new(
            &mut world,
            &app.renderer,
            &mut bvh,
            true,
            Entity::DANGLING,
            0,
            110.0,
            0.0,
            0.0,
            0.0,
            app.renderer.get_texture_id_from_name("sun").unwrap(),
            "Sun",
        );

        let mercury_entity = Planet::new(
            &mut world,
            &app.renderer,
            &mut bvh,
            false,
            sun_entity,
            1,
            1.,
            10000.0,
            1.0,
            0.0027,
            app.renderer.get_texture_id_from_name("moon").unwrap(),
            "Mercury",
        );

        let planet_entity = Planet::new(
            &mut world,
            &app.renderer,
            &mut bvh,
            false,
            sun_entity,
            1,
            1.,
            20000.0,
            1.0,
            0.0027,
            app.renderer.get_texture_id_from_name("earth").unwrap(),
            "Earth",
        );

        let moon_entity = Planet::new(
            &mut world,
            &app.renderer,
            &mut bvh,
            false,
            planet_entity,
            2,
            0.2,
            60.0,
            0.0749,
            0.0749,
            app.renderer.get_texture_id_from_name("moon").unwrap(),
            "Moon",
        );

        let event_queue = Arc::new(EventQueue::new());

        world.spawn((Button::new(
            "next-turn",
            Rectangle::new(
                app.window_size.x as f32 - 100.0,
                app.window_size.y as f32 - 120.0,
                90.0,
                90.0,
            ),
            app.renderer.get_texture_id_from_name("next-turn").unwrap(),
            app.renderer
                .get_texture_id_from_name("next-turn-hover")
                .unwrap(),
            event_queue.clone(),
        ),));

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

            selection: 2,
            selected_pos: nalgebra_glm::vec3(0.0, 0.0, 0.0),
            prev_selected_pos: nalgebra_glm::vec3(0.0, 0.0, 0.0),
            transition: 1.0,
            selected_body_radius: 0.0,
            phi: 2.5,
            theta: 0.0,
            distance: 20.0,
            prev_enter_state: false,
            bodies: vec![sun_entity, mercury_entity, planet_entity, moon_entity],

            event_queue: event_queue.clone(),

            turn: 0,
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
            if self.selection >= self.bodies.len() {
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
            .min(self.selected_body_radius * 40000.0 + 234.0);
    }

    /// Updates planets based on their on-rails orbits around their parent bodies
    fn planet_system(&mut self, app: &App, tier: u32) {
        let mut parent_pos_map = HashMap::new();
        for (entity, (model, _planet)) in self.world.query::<(&ModelComponent, &Planet)>().iter() {
            parent_pos_map.insert(entity, model.get_position());
        }

        for (entity, (model, planet)) in
            self.world.query_mut::<(&mut ModelComponent, &mut Planet)>()
        {
            if planet.tier != tier {
                continue;
            }

            const REAL_SECS_PER_GAME_YEAR: f32 = 6.0; // How many real seconds it takes for earth to go around the sun once
            const T_SEED: f32 = 98400.0; // An offset from t, so that the planets are not all in a line.
            let t = self.turn as f32;

            if planet.tier != 0 {
                let parent_pos = parent_pos_map.get(&planet.parent_planet_id).unwrap();
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
            } else {
                model.set_position(nalgebra_glm::vec3(0.0, 0.0, 0.0));
            }

            if planet.day_time_years != 0.0 {
                planet.rotation = 2.0 * PI * (t + T_SEED)
                    / (REAL_SECS_PER_GAME_YEAR * planet.day_time_years)
                    + 3.14;
            }

            if entity == self.bodies[self.selection] {
                self.selected_pos = model.get_position();
                self.selected_body_radius = planet.body_radius;
            }
        }
    }

    fn orbit_system(&mut self, _app: &App) {
        let mut parent_pos_map = HashMap::new();
        for (entity, (model, _planet)) in self.world.query::<(&ModelComponent, &Planet)>().iter() {
            parent_pos_map.insert(entity, model.get_position());
        }

        for (_entity, (planet, orbit)) in
            self.world.query_mut::<(&Planet, &mut LinePathComponent)>()
        {
            let camera_distance = self.distance;
            let parent_pos = parent_pos_map.get(&planet.parent_planet_id).unwrap();
            orbit.color.w = if camera_distance < 5.0 * planet.body_radius {
                0.0
            } else {
                camera_distance / (planet.body_radius * 8.0).powf(5.0)
            };
            orbit.position = *parent_pos;
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
