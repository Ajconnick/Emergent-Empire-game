use crate::objects::*;

use obj::{load_obj, Obj, Vertex};

pub struct Mesh {
    v_ibo: Ibo,
    v_vbo: Vbo,
    v_vao: Vao,

    n_ibo: Ibo,
    n_vbo: Vbo,
    n_vao: Vao,

    indices: Vec<u16>,
    vertices: Vec<f32>,
    normals: Vec<f32>,
}

impl Mesh {
    pub fn new() -> Self {
        let input = include_bytes!("../res/ico-sphere.obj");
        let obj: Obj = load_obj(&input[..]).unwrap();
        let vb = obj.vertices;

        let indices = obj.indices;
        let vertices = flatten_positions(&vb);
        let normals = flatten_normals(&vb);

        let v_ibo = Ibo::gen();
        let v_vao = Vao::gen();
        let v_vbo = Vbo::gen();
        let n_ibo = Ibo::gen();
        let n_vao = Vao::gen();
        let n_vbo = Vbo::gen();

        v_vao.set(0);
        n_vao.set(0);

        Mesh {
            v_ibo,
            v_vao,
            v_vbo,
            n_ibo,
            n_vao,
            n_vbo,
            indices,
            vertices,
            normals,
        }
    }

    pub fn set(&self) {
        self.v_vbo.set(&self.vertices);
        self.v_vao.enable(0);
        self.v_ibo.set(&vec_u32_from_vec_u16(&self.indices));

        self.n_vbo.set(&self.normals);
        self.n_vao.enable(1);
        self.n_ibo.set(&vec_u32_from_vec_u16(&self.indices));
    }

    pub fn indices_len(&self) -> i32 {
        self.indices.len() as i32
    }
}

fn flatten_positions(vertices: &Vec<Vertex>) -> Vec<f32> {
    let mut retval = vec![];
    for vertex in vertices {
        retval.push(vertex.position[0]);
        retval.push(vertex.position[1]);
        retval.push(vertex.position[2]);
    };
    retval
}

fn flatten_normals(vertices: &Vec<Vertex>) -> Vec<f32> {
    let mut retval = vec![];
    for vertex in vertices {
        retval.push(vertex.normal[0]);
        retval.push(vertex.normal[1]);
        retval.push(vertex.normal[2]);
    };
    retval
}

fn vec_u32_from_vec_u16(input: &Vec<u16>) -> Vec<u32> {
    let mut retval = vec![];
    for x in input {
        retval.push(*x as u32);
    }
    retval
}