use gl::types::GLuint;

use crate::engine::{camera::Camera, mesh::Mesh, objects::Uniform};

pub const YEAR_SPEED: f32 = 7.;

pub struct Planet {
    pub body_radius: f32,
    orbital_radius: f32,
    mesh: Mesh,
    atmosphere_color: nalgebra_glm::Vec3,

    pub position: nalgebra_glm::Vec3,
}

impl Planet {
    pub fn new(
        program_id: u32,
        body_radius: f32,
        orbital_radius: f32,
        texture_filename: &str,
        atmosphere_color: nalgebra_glm::Vec3,
    ) -> Self {
        let mesh = Mesh::new(texture_filename, program_id);
        Planet {
            body_radius,
            orbital_radius,
            mesh,
            atmosphere_color,
            position: nalgebra_glm::vec3(0., 0., 0.),
        }
    }

    pub fn update(&mut self, t: f32) {
        self.position = nalgebra_glm::vec3(
            (YEAR_SPEED * t / self.body_radius).cos() * self.orbital_radius,
            (YEAR_SPEED * t / self.body_radius).sin() * self.orbital_radius,
            0.0,
        );
    }

    // Given a planet, the shader id, a mesh, and the camera, renders out a 3d planet!
    pub fn draw(&self, program_id: GLuint, camera: &Camera) {
        let mut model_matrix = nalgebra_glm::one();
        model_matrix = nalgebra_glm::translate(&model_matrix, &self.position);
        model_matrix = nalgebra_glm::scale(
            &model_matrix,
            &nalgebra_glm::vec3(self.body_radius, self.body_radius, self.body_radius),
        );
        let (view_matrix, proj_matrix) = camera.gen_view_proj_matrices();

        unsafe {
            // These Uniforms allow us to pass data (ex: window size, elapsed time) to the GPU shaders
            // let u_model_matrix = Uniform::new(program_id, "u_model_matrix").unwrap();
            // let u_view_matrix = Uniform::new(program_id, "u_view_matrix").unwrap();
            // let u_proj_matrix = Uniform::new(program_id, "u_proj_matrix").unwrap();
            // let u_sun_pos = Uniform::new(program_id, "u_sun_dir_vec3").unwrap();
            // let u_atmos_color = Uniform::new(program_id, "u_atmos_color").unwrap();
            // gl::UniformMatrix4fv(
            //     u_model_matrix.id,
            //     1,
            //     gl::FALSE,
            //     &model_matrix.columns(0, 4)[0],
            // );
            // gl::UniformMatrix4fv(
            //     u_view_matrix.id,
            //     1,
            //     gl::FALSE,
            //     &view_matrix.columns(0, 4)[0],
            // );
            // gl::UniformMatrix4fv(
            //     u_proj_matrix.id,
            //     1,
            //     gl::FALSE,
            //     &proj_matrix.columns(0, 4)[0],
            // );
            // gl::Uniform3f(u_sun_pos.id, 0., 0., 0.);
            // gl::Uniform3f(
            //     u_atmos_color.id,
            //     self.atmosphere_color.x,
            //     self.atmosphere_color.y,
            //     self.atmosphere_color.z,
            // );

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
