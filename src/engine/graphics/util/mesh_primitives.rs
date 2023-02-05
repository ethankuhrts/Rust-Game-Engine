use crate::graphics::{ Mesh, GraphicsBundle, Material, Vertex };

pub struct MeshPrimitives;

impl MeshPrimitives {

    pub fn cube (bundle: &GraphicsBundle, name: &str) -> Mesh {
        let vertices: &[Vertex] = &[
            Vertex { position: [-0.5, -0.5, 0.5], ..Default::default() },
            Vertex { position: [0.5, -0.5, 0.5], ..Default::default() },
            Vertex { position: [-0.5, 0.5, 0.5], ..Default::default() },
            Vertex { position: [0.5, 0.5, 0.5], ..Default::default() },

            Vertex { position: [-0.5, -0.5, -0.5], ..Default::default() },
            Vertex { position: [0.5, -0.5, -0.5], ..Default::default() },
            Vertex { position: [-0.5, 0.5, -0.5], ..Default::default() },
            Vertex { position: [0.5, 0.5, -0.5], ..Default::default() },
        ];

        let indices: &[u32] = &[
            //Top
            2, 6, 7,
            2, 3, 7,

            //Bottom
            0, 4, 5,
            0, 1, 5,

            //Left
            0, 2, 6,
            0, 4, 6,

            //Right
            1, 3, 7,
            1, 5, 7,

            //Front
            0, 2, 3,
            0, 1, 3,

            //Back
            4, 6, 7,
            4, 5, 7
        ];
        
        return Mesh::new(bundle, name, vertices, indices);
    }

    pub fn triangle (bundle: &GraphicsBundle, name: &str) -> Mesh {
        let vertices: &[Vertex] = &[
            Vertex { position: [0.0, 0.5, 0.0], ..Default::default() }, // A
            Vertex { position: [0.5, -0.5, 0.0], ..Default::default() }, // B
            Vertex { position: [-0.5, -0.5, 0.0], ..Default::default() }, // C
        ];

        let indices: &[u32] = &[
            0, 1, 2,
        ];
        
        return Mesh::new(bundle, name, vertices, indices);
    }
    pub fn plane (bundle: &GraphicsBundle, name: &str) -> Mesh {
        let vertices: &[Vertex] = &[
            Vertex { position: [-0.5, 0.5, 0.0], uvs: [1.0, 0.0], normal: [0.0, 0.0, 1.0] },
            Vertex { position: [0.5, 0.5, 0.0], uvs: [1.0, 1.0], normal: [0.0, 0.0, 1.0] },
            Vertex { position: [0.5, -0.5, 0.0], uvs: [0.0, 1.0], normal: [0.0, 0.0, 1.0] },
            Vertex { position: [-0.5, -0.5, 0.0], uvs: [0.0, 0.0], normal: [0.0, 0.0, 1.0] },
        ];

        let indices: &[u32] = &[
            0, 1, 2,
            2, 3, 0
        ];        
        return Mesh::new(bundle, name, vertices, indices);
    }
}