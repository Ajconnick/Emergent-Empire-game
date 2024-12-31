//! This module is responsible for defining the planet component
//! TODO: Should probably rename to `body.rs`, since this can represent moons and suns

use apricot::{
    bvh::{BVHNodeId, BVH},
    render_core::{LinePathComponent, ModelComponent, RenderContext, TextureId},
};
use hecs::{Entity, World};

pub struct Planet {
    pub id: usize,
    pub parent_planet_id: usize,
    pub tier: u32,
    pub body_radius: f32,
    pub orbital_radius: f32,
    pub orbital_time_years: f32,
    pub day_time_years: f32,
    pub rotation: f32,
    pub bvh_node_id: BVHNodeId,
    pub name: &'static str,
}

impl Planet {
    pub fn new(
        world: &mut World,
        renderer: &RenderContext,
        bvh: &mut BVH<Entity>,

        gaseous: bool,
        id: usize,
        parent_planet_id: usize,
        tier: u32,
        body_radius: f32,
        orbital_radius: f32,
        orbital_time_years: f32,
        day_time_years: f32,
        texture_id: TextureId,
        name: &'static str,
    ) -> usize {
        let planet_mesh = if gaseous {
            renderer.get_mesh_id_from_name("uv").unwrap()
        } else {
            renderer.get_mesh_id_from_name("ico").unwrap()
        };

        let position = nalgebra_glm::vec3(0., 0., 0.);
        let scale_vec = nalgebra_glm::vec3(body_radius, body_radius, body_radius);

        let planet_entity = world.spawn((
            ModelComponent::new(planet_mesh, texture_id, position, scale_vec),
            LinePathComponent::from_orbit(orbital_radius, 0.0, 1024),
        ));
        let bvh_node_id = bvh.insert(
            planet_entity,
            renderer
                .get_mesh_aabb(planet_mesh)
                .scale(scale_vec)
                .translate(position),
        );

        world
            .insert(
                planet_entity,
                (Planet {
                    id,
                    parent_planet_id,
                    tier,
                    body_radius,
                    orbital_radius,
                    orbital_time_years,
                    day_time_years,
                    rotation: 0.0,
                    bvh_node_id,
                    name,
                },),
            )
            .unwrap();

        id
    }
}
