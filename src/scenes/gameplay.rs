use core::f32;
use std::ops::Deref;

use apricot::{
    app::{App, Scene},
    bvh::{BVHNodeId, BVH},
    camera::{Camera, ProjectionKind},
    chunked_map::ChunkedPerlinMap,
    perlin::HeightMap,
    ray::Ray,
    rectangle::Rectangle,
    render2d::NineSlice,
    render_core::ModelComponent,
    shadow_map::DirectionalLightSource,
};
use hecs::{Entity, World};
use rand::SeedableRng;
use sdl2::keyboard::Scancode;

const MAP_WIDTH: usize = 16384; // 16k is desireable!
const CHUNK_SIZE: usize = 16;
const UNIT_PER_METER: f32 = 0.05;
const MINUTES_PER_DAY: f32 = 10.0;
const TICKS_OFFSET: f32 = 0.0;

pub const QUAD_DATA: &[u8] = include_bytes!("../../res/quad.obj");
pub const QUAD_XY_DATA: &[u8] = include_bytes!("../../res/quad-xy.obj");
pub const CUBE_DATA: &[u8] = include_bytes!("../../res/cube.obj");
pub const CONE_DATA: &[u8] = include_bytes!("../../res/cone.obj");
pub const BUSH_DATA: &[u8] = include_bytes!("../../res/bush.obj");

struct Player {
    bvh_node_id: BVHNodeId,
}

pub struct Rock {}

pub struct Gameplay {
    world: World,
    camera_3d: Camera,
    directional_light: DirectionalLightSource,
    map: ChunkedPerlinMap,
    bvh: BVH<Entity>,

    // Player stuff
    position: nalgebra_glm::Vec3,
    velocity: nalgebra_glm::Vec3,

    prev_space_state: bool,
    debug: bool,

    update_swap: u32,
}

impl Scene for Gameplay {
    fn update(&mut self, app: &App) {
        self.directional_light.light_dir.z =
            (app.ticks as f32 / (60.0 * 60.0 * 0.5 * MINUTES_PER_DAY) + TICKS_OFFSET).cos();
        self.directional_light.light_dir.y =
            (app.ticks as f32 / (60.0 * 60.0 * 0.5 * MINUTES_PER_DAY) + TICKS_OFFSET).sin();
        self.map.check_chunks(
            &app.renderer,
            self.position.xy(),
            &mut self.world,
            &mut self.bvh,
        );
        self.update_view(app);
        self.update_clickers(app);
        self.update_swap += 1;
    }

    fn render(&mut self, app: &App) {
        // sky system
        let model_t = app.ticks as f32 / (60.0 * 60.0 * 0.5 * MINUTES_PER_DAY) + TICKS_OFFSET;
        unsafe {
            let day_color = nalgebra_glm::vec3(172.0, 205.0, 248.0);
            let night_color = nalgebra_glm::vec3(5.0, 6.0, 7.0);
            let red_color = nalgebra_glm::vec3(124.0, 102.0, 86.0);
            let do_color = if model_t.cos() > 0.0 {
                day_color
            } else {
                night_color
            };
            let dnf = model_t.sin().powf(100.0);
            let result = dnf * red_color + (1.0 - dnf) * do_color;
            gl::ClearColor(result.x / 255., result.y / 255., result.z / 255., 1.0);
        }

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
            self.debug,
        );
        app.renderer
            .set_color(nalgebra_glm::vec4(1.0, 1.0, 1.0, 0.8));
        app.renderer
            .render_3d_outlines_system(&mut self.world, &self.bvh);

        let font = app.renderer.get_font_id_from_name("font").unwrap();
        app.renderer.set_font(font);
        app.renderer
            .draw_text(nalgebra_glm::vec2(10.0, 10.0), "Hunger");

        let nine_slice = NineSlice {
            texture: app
                .renderer
                .get_texture_id_from_name("nine-slice-test")
                .unwrap(),
            border: 8.0,
        };
        let nine_slice2 = NineSlice {
            texture: app
                .renderer
                .get_texture_id_from_name("nine-slice-test2")
                .unwrap(),
            border: 8.0,
        };
        app.renderer
            .render_nine_slice(nine_slice, Rectangle::new(70.0, 10.0, 164.0, 16.0));
        app.renderer.render_nine_slice(
            nine_slice2,
            Rectangle::new(
                70.0,
                10.0,
                (app.ticks as f32 * 0.01).cos().abs() * 100.0 + 16.0,
                16.0,
            ),
        );

        self.render_outline_text(app);

        // Draw the two hand inventory slots
        const SLOT_SIZE: f32 = 96.0;
        app.renderer
            .set_color(nalgebra_glm::vec4(0.0, 0.0, 0.0, 0.5));
        app.renderer.fill_rect(Rectangle::new(
            app.window_size.x as f32 * 0.5 - SLOT_SIZE * 1.5,
            app.window_size.y as f32 - SLOT_SIZE,
            SLOT_SIZE,
            SLOT_SIZE,
        ));
        app.renderer.fill_rect(Rectangle::new(
            app.window_size.x as f32 * 0.5 + SLOT_SIZE * 0.5,
            app.window_size.y as f32 - SLOT_SIZE,
            SLOT_SIZE,
            SLOT_SIZE,
        ));
    }
}

impl Gameplay {
    pub fn new(app: &App) -> Self {
        let mut world = World::new();

        let mut rng = rand::rngs::StdRng::from_entropy();
        let mut map =
            ChunkedPerlinMap::new(MAP_WIDTH, CHUNK_SIZE, 0.01, rand::Rng::gen(&mut rng), 1.0);

        // Setup the mesh manager
        let quad_mesh = app.renderer.add_mesh_from_obj(QUAD_DATA, Some("quad"));
        app.renderer
            .add_mesh_from_obj(QUAD_XY_DATA, Some("quad-xy"));
        let cube_mesh = app.renderer.add_mesh_from_obj(CUBE_DATA, Some("cube"));
        app.renderer.add_mesh_from_obj(CONE_DATA, Some("tree"));
        app.renderer.add_mesh_from_obj(BUSH_DATA, Some("bush"));

        // Setup the texture manager
        let grass_texture = app
            .renderer
            .add_texture_from_png("grass.png", Some("grass"));
        let water_texture = app
            .renderer
            .add_texture_from_png("water.png", Some("water"));
        app.renderer.add_texture_from_png("tree.png", Some("tree"));
        app.renderer.add_texture_from_png("rock.png", Some("rock"));
        app.renderer
            .add_texture_from_png("nine-slice-test.png", Some("nine-slice-test"));
        app.renderer
            .add_texture_from_png("nine-slice-test2.png", Some("nine-slice-test2"));

        // Setup the font manager
        app.renderer
            .add_font("res/Consolas.ttf", "font", 16, sdl2::ttf::FontStyle::NORMAL);

        let mut bvh = BVH::<Entity>::new();

        let spawn_point =
            nalgebra_glm::vec3(MAP_WIDTH as f32 / 2.0 + 1.0, MAP_WIDTH as f32 / 2.0, 2.5);
        loop {
            if map.chunkless_height(spawn_point.xy()) > 0.74 {
                break;
            }
            map = ChunkedPerlinMap::new(MAP_WIDTH, CHUNK_SIZE, 0.01, rand::Rng::gen(&mut rng), 1.0);
        }

        // Add player
        let scale_vec = nalgebra_glm::vec3(0.2, 0.2, 1.0);
        let player_entity = world.spawn((ModelComponent::new(
            cube_mesh,
            grass_texture,
            spawn_point,
            scale_vec,
        ),));
        let player_node_id = bvh.insert(
            player_entity,
            app.renderer
                .get_mesh_aabb(cube_mesh)
                .scale(scale_vec)
                .translate(spawn_point),
        );
        world
            .insert(
                player_entity,
                (Player {
                    bvh_node_id: player_node_id,
                },),
            )
            .unwrap();

        // Add water plane
        let scale_vec = nalgebra_glm::vec3(MAP_WIDTH as f32, MAP_WIDTH as f32, MAP_WIDTH as f32);
        let water_entity = world.spawn((ModelComponent::new(
            quad_mesh,
            water_texture,
            nalgebra_glm::vec3(0.0, 0.0, 0.5),
            scale_vec,
        ),));
        bvh.insert(
            water_entity,
            app.renderer
                .get_mesh_aabb(quad_mesh)
                .scale(scale_vec)
                .translate(nalgebra_glm::vec3(0.0, 0.0, 0.5)),
        );

        Self {
            world,
            camera_3d: Camera::new(
                spawn_point,
                nalgebra_glm::vec3(MAP_WIDTH as f32 / 2.0, MAP_WIDTH as f32 / 2.0, 0.5),
                nalgebra_glm::vec3(0.0, 0.0, 1.0),
                ProjectionKind::Perspective { fov: 0.65 },
            ),
            bvh,
            map,
            directional_light: DirectionalLightSource::new(
                Camera::new(
                    nalgebra_glm::vec3(MAP_WIDTH as f32 / -2.0, 0.0, 2.0),
                    nalgebra_glm::vec3(MAP_WIDTH as f32 / 2.0, MAP_WIDTH as f32 / 2.0, 0.5),
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
                nalgebra_glm::vec3(-0.1, 0.0, 0.86),
                MAP_WIDTH as i32,
            ),

            position: spawn_point,
            velocity: nalgebra_glm::vec3(0.0, 0.0, 0.0),

            prev_space_state: false,
            debug: false,
            update_swap: 0,
        }
    }

    fn update_view(&mut self, app: &App) {
        let mut player_entt: Option<Entity> = None;
        for (entt, _) in &mut self.world.query::<&Player>() {
            player_entt = Some(entt);
            break;
        }
        let zoom = 1.0;

        let curr_w_state = app.keys[Scancode::W as usize];
        let curr_s_state = app.keys[Scancode::S as usize];
        let curr_a_state = app.keys[Scancode::A as usize];
        let curr_d_state = app.keys[Scancode::D as usize];
        let curr_space_state = app.keys[Scancode::Space as usize];
        let walking = curr_w_state || curr_s_state || curr_a_state || curr_d_state;
        let walk_speed: f32 = 1.6 * 2.5 * zoom;
        let facing_vec = nalgebra_glm::vec3(1.0, 0.0, 0.0);
        let sideways_vec = nalgebra_glm::vec3(0.0, 1.0, 0.0);
        let mut player_vel_vec: nalgebra_glm::Vec3 = nalgebra_glm::zero();
        if curr_w_state {
            player_vel_vec += -facing_vec;
        }
        if curr_s_state {
            player_vel_vec += facing_vec;
        }
        if curr_a_state {
            player_vel_vec += -sideways_vec;
        }
        if curr_d_state {
            player_vel_vec += sideways_vec;
        }
        self.debug = false;
        if curr_space_state && !self.prev_space_state {
        } else if walking {
            // Move the player, this way moving diagonal isn't faster
            self.velocity +=
                player_vel_vec.normalize() * walk_speed * 4.317 * UNIT_PER_METER / 62.5;
        }
        self.prev_space_state = curr_space_state;
        self.position += self.velocity;
        self.position.z = self.map.height_interpolated(self.position.xy());

        let mut model = self
            .world
            .get::<&mut ModelComponent>(player_entt.unwrap())
            .unwrap();
        model.set_position(self.position);
        let player_bvh_node_id = self
            .world
            .get::<&Player>(player_entt.unwrap())
            .unwrap()
            .bvh_node_id;
        self.bvh.move_obj(
            player_bvh_node_id,
            &app.renderer.get_model_aabb(&model),
            &self.velocity,
        );
        self.velocity *= 0.8; // friction

        self.camera_3d
            .set_position(self.position + nalgebra_glm::vec3(13.85, 0.0, 8.00) * zoom);
        self.camera_3d.set_lookat(self.position);

        app.renderer.set_camera(self.camera_3d);
    }

    fn update_clickers(&mut self, app: &App) {
        if app.mouse_left_clicked {
            println!("{:?} {:?}", app.mouse_pos.x, app.mouse_pos.y);
        }
        let ndc_x = (2.0 * app.mouse_pos.x as f32) / app.window_size.x as f32 - 1.0;
        let ndc_y = 1.0 - (2.0 * (app.mouse_pos.y as f32)) / app.window_size.y as f32;

        let clip_coordinates = nalgebra_glm::vec4(ndc_x, ndc_y, -0.0, 1.0);

        let (inv_proj, inv_view) = self.camera_3d.inv_proj_and_view();
        let mut eye_coords = inv_proj * clip_coordinates;
        eye_coords /= eye_coords.w;

        let world_coords = inv_view * eye_coords;
        let dir = (world_coords.xyz() - self.camera_3d.position()).normalize();

        let ray = Ray {
            dir,
            origin: self.camera_3d.position(),
        };

        // Set all outlines to false
        for (_, model) in &mut self.world.query::<&mut ModelComponent>() {
            model.outlined = false;
        }

        // Set hovered outlines to true
        let hovereds: Vec<Entity> = self
            .bvh
            .iter_ray(&ray)
            .filter(|entity| self.world.get::<&Rock>(*entity).is_ok())
            .collect();
        for entity in hovereds {
            self.world
                .get::<&mut ModelComponent>(entity)
                .unwrap()
                .outlined = true;
            if app.mouse_left_clicked {
                println!("{:?}", entity);
            }
        }
    }

    fn render_outline_text(&mut self, app: &App) {
        let hovereds: Vec<Entity> = self
            .bvh
            .iter_frustrum(&self.camera_3d.frustum(), false)
            .filter(|entity| {
                if let Ok(model) = self.world.get::<&ModelComponent>(*entity) {
                    model.outlined
                } else {
                    false
                }
            })
            .collect();

        for entity in hovereds {
            // TODO: Convert from 2d/3d points in camera, both ways
            app.renderer.draw_text(app.mouse_pos, "This is a rock!");
        }
    }
}
