use std::f32::consts::PI;

use gl::types::GLuint;

use crate::engine::{camera::Camera, mesh::Mesh, objects::Uniform};

pub const YEAR_SPEED: f32 = 2.0 * PI / 219000.0; // One full earth year in 300 seconds (5 minutes)

pub const ICO_DATA: &[u8] = include_bytes!("../../res/ico-sphere.obj");
pub const UV_DATA: &[u8] = include_bytes!("../../res/uv-sphere.obj");

pub struct Planet {
    pub body_radius: f32,
    orbital_radius: f32,
    tilt: f32,
    orbital_time_years: f32,
    day_time_years: f32,
    mesh: Mesh,
    atmosphere_color: nalgebra_glm::Vec3,
    emissive_color: nalgebra_glm::Vec3,

    pub position: nalgebra_glm::Vec3,
    rotation: f32,
}

impl Planet {
    pub fn new(
        gaseous: bool,
        body_radius: f32,
        orbital_radius: f32,
        tilt: f32,
        orbital_time_years: f32,
        day_time_years: f32,
        texture_filename: &str,
        atmosphere_color: nalgebra_glm::Vec3,
        emissive_color: nalgebra_glm::Vec3,
    ) -> Self {
        let mesh_data = if gaseous { UV_DATA } else { UV_DATA };
        let mesh = Mesh::new(mesh_data, texture_filename);
        Planet {
            body_radius,
            orbital_radius,
            tilt,
            orbital_time_years,
            day_time_years,
            mesh,
            atmosphere_color,
            emissive_color,
            position: nalgebra_glm::vec3(0., 0., 0.),
            rotation: 0.0,
        }
    }

    pub fn update(&mut self, t: f32) {
        if self.orbital_time_years > 0.0 {
            self.position = nalgebra_glm::vec3(
                (YEAR_SPEED * t / self.orbital_time_years).cos() * self.orbital_radius,
                (YEAR_SPEED * t / self.orbital_time_years).sin() * self.orbital_radius,
                0.0,
            );
        }
        if self.day_time_years > 0.0 {
            self.rotation = YEAR_SPEED * t / self.day_time_years;
        }
    }

    // Given a planet, the shader id, a mesh, and the camera, renders out a 3d planet!
    pub fn draw(&self, program_id: GLuint, camera: &Camera, selected_offset: nalgebra_glm::Vec3) {
        let subtracted = self.position - selected_offset;
        let mut model_matrix = nalgebra_glm::one();
        model_matrix = nalgebra_glm::translate(&model_matrix, &subtracted);
        model_matrix = nalgebra_glm::rotate_y(&model_matrix, self.tilt);
        model_matrix = nalgebra_glm::rotate_z(&model_matrix, self.rotation);
        model_matrix = nalgebra_glm::scale(
            &model_matrix,
            &nalgebra_glm::vec3(self.body_radius, self.body_radius, self.body_radius),
        );
        let (view_matrix, proj_matrix) = camera.gen_view_proj_matrices();

        unsafe {
            // These Uniforms allow us to pass data (ex: window size, elapsed time) to the GPU shaders
            let u_model_matrix = Uniform::new(program_id, "u_model_matrix").unwrap();
            let u_view_matrix = Uniform::new(program_id, "u_view_matrix").unwrap();
            let u_proj_matrix = Uniform::new(program_id, "u_proj_matrix").unwrap();
            let u_sun_pos = Uniform::new(program_id, "u_sun_dir_vec3").unwrap();
            let u_atmos_color = Uniform::new(program_id, "u_atmos_color").unwrap();
            let u_emissive_color = Uniform::new(program_id, "u_emissive_color").unwrap();
            gl::UniformMatrix4fv(
                u_model_matrix.id,
                1,
                gl::FALSE,
                &model_matrix.columns(0, 4)[0],
            );
            gl::UniformMatrix4fv(
                u_view_matrix.id,
                1,
                gl::FALSE,
                &view_matrix.columns(0, 4)[0],
            );
            gl::UniformMatrix4fv(
                u_proj_matrix.id,
                1,
                gl::FALSE,
                &proj_matrix.columns(0, 4)[0],
            );
            gl::Uniform3f(
                u_sun_pos.id,
                -selected_offset.x,
                -selected_offset.y,
                -selected_offset.z,
            );
            gl::Uniform3f(
                u_atmos_color.id,
                self.atmosphere_color.x,
                self.atmosphere_color.y,
                self.atmosphere_color.z,
            );
            gl::Uniform3f(
                u_emissive_color.id,
                self.emissive_color.x,
                self.emissive_color.y,
                self.emissive_color.z,
            );

            self.mesh.set(program_id);

            gl::DrawElements(
                gl::TRIANGLES,
                self.mesh.indices_len(),
                gl::UNSIGNED_INT,
                0 as *const _,
            );
        }
    }
}
