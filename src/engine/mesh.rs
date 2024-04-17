use super::objects::*;

use std::ffi::CString;
use std::path::Path;

use obj::{load_obj, Obj, TexturedVertex};

pub struct Mesh {
    v_ibo: Ibo,
    v_vbo: Vbo,
    v_vao: Vao,

    n_ibo: Ibo,
    n_vbo: Vbo,
    n_vao: Vao,

    t_ibo: Ibo,
    t_vbo: Vbo,
    t_vao: Vao,

    texture: Texture,

    indices: Vec<u16>,
    vertices: Vec<f32>,
    normals: Vec<f32>,
    uv: Vec<f32>,
    // program: u32,
}

impl Mesh {
    pub fn new(texture_filename: &str, program: u32) -> Self {
        let input = include_bytes!("../../res/uv-sphere.obj"); // TODO: Load this at runtime
        let obj: Obj<TexturedVertex> = load_obj(&input[..]).unwrap();
        let vb: Vec<TexturedVertex> = obj.vertices;

        let indices = obj.indices;
        let vertices = flatten_positions(&vb);
        let normals = flatten_normals(&vb);
        let uv = flatten_uv(&vb);

        // Vertex inputs
        let v_ibo = Ibo::gen();
        let v_vao = Vao::gen();
        let v_vbo = Vbo::gen();

        // Normal inputs
        let n_ibo = Ibo::gen();
        let n_vao = Vao::gen();
        let n_vbo = Vbo::gen();

        // Texture UV inputs
        let t_ibo = Ibo::gen();
        let t_vao = Vao::gen();
        let t_vbo = Vbo::gen();

        v_vao.set(0);
        n_vao.set(1);
        t_vao.set(2);

        let texture = Texture::new();
        texture.load(&Path::new(texture_filename)).unwrap();
        let uniform = CString::new("texture0").unwrap();
        unsafe { gl::Uniform1i(gl::GetUniformLocation(program, uniform.as_ptr()), 0) }; // TODO: Is this needed?

        Mesh {
            v_ibo,
            v_vao,
            v_vbo,
            n_ibo,
            n_vao,
            n_vbo,
            t_ibo,
            t_vao,
            t_vbo,
            texture,
            indices,
            vertices,
            normals,
            uv,
            // program,
        }
    }

    pub fn set(&self, program: u32) {
        self.texture.activate(gl::TEXTURE0);
        let uniform = CString::new("texture0").unwrap();
        unsafe { gl::Uniform1i(gl::GetUniformLocation(program, uniform.as_ptr()), 0) };

        self.v_vbo.set(&self.vertices);
        self.v_vao.enable(0);
        self.v_ibo.set(&vec_u32_from_vec_u16(&self.indices));

        self.n_vbo.set(&self.normals);
        self.n_vao.enable(1);
        self.n_ibo.set(&vec_u32_from_vec_u16(&self.indices));

        self.t_vbo.set(&self.uv);
        self.t_vao.enable(2);
        self.t_ibo.set(&vec_u32_from_vec_u16(&self.indices));
    }

    pub fn indices_len(&self) -> i32 {
        self.indices.len() as i32
    }
}

fn flatten_positions(vertices: &Vec<TexturedVertex>) -> Vec<f32> {
    let mut retval = vec![];
    for vertex in vertices {
        retval.push(vertex.position[0]);
        retval.push(vertex.position[1]);
        retval.push(vertex.position[2]);
    }
    retval
}

fn flatten_normals(vertices: &Vec<TexturedVertex>) -> Vec<f32> {
    let mut retval = vec![];
    for vertex in vertices {
        retval.push(vertex.normal[0]);
        retval.push(vertex.normal[1]);
        retval.push(vertex.normal[2]);
    }
    retval
}

fn flatten_uv(vertices: &Vec<TexturedVertex>) -> Vec<f32> {
    let mut retval = vec![];
    for vertex in vertices {
        retval.push(vertex.texture[0]);
        retval.push(vertex.texture[1]);
        retval.push(vertex.texture[2]);
    }
    retval
}

fn vec_u32_from_vec_u16(input: &Vec<u16>) -> Vec<u32> {
    let mut retval = vec![];
    for x in input {
        retval.push(*x as u32);
    }
    retval
}
