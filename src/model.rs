use cgmath::{Vector3, Point3};
use tobj;

pub struct Model {
    pub verts: Vec<Point3<f32>>,
    pub faces: Vec<Vector3<usize>>, 
    pub texcoords: Vec<f32>,
    pub texcoords_indices: Vec<Vector3<usize>>,
}

impl Model {
    // 
    pub fn new(path: &str) -> Self {
        let file = tobj::load_obj(
            path,
            &tobj::LoadOptions {
                ..Default::default()
            }
        );

        let (models, _materials) = file.expect("Failed to open the .obj file");

        // only get the first model
        let model = &models[0];

        let mut verts = Vec::new();
        let mut faces = Vec::new();
        let mut texcoords_indices = Vec::new();

        let n_vert = model.mesh.positions.len() / 3; // the number of vertices
        println!("number of vertices: {}", n_vert);
        for chunk in model.mesh.positions.chunks(3) {
            verts.push(Point3::new(chunk[0], chunk[1], chunk[2]));
        }

        let n_face = model.mesh.indices.len() / 3;
        println!("number of faces: {}", n_face);
        println!("indices len: {}", model.mesh.indices.len());
        println!("tex coord indices len: {}", model.mesh.texcoord_indices.len());
        for i in 0..n_face {
            faces.push(Vector3::new(model.mesh.indices[3*i] as usize, model.mesh.indices[3*i+1] as usize, model.mesh.indices[3*i+2] as usize));
            texcoords_indices.push(Vector3::new(model.mesh.texcoord_indices[3*i] as usize, model.mesh.texcoord_indices[3*i+1] as usize, model.mesh.texcoord_indices[3*i+2] as usize));
        }

        Self {
            verts,
            faces,
            texcoords: model.mesh.texcoords.clone(),
            texcoords_indices,
        }
    }
}