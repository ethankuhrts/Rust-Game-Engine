use crate::graphics::{Material, GraphicsBundle, Vertex};
use wgpu::util::DeviceExt;

#[derive(Debug)]
pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub element_count: u32,
    
}


impl Mesh {

    pub fn new(bundle: &GraphicsBundle, name: &str, vertices: &[Vertex], indices: &[u32]) -> Self {
        let element_count = indices.len() as u32;
        Mesh {
            name: String::from(name),
            vertex_buffer: bundle.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{:?} Vertex Buffer", name)),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                }
            ),
            index_buffer: bundle.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{:?} Index Buffer", name)),
                    contents: bytemuck::cast_slice(&indices),
                    usage: wgpu::BufferUsages::INDEX,
                }
            ),
            element_count,
        }
    }
    
    
}