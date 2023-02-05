use std::ops::Range;

use glam::{Mat4, Vec3};
use wgpu::{util::DeviceExt, Device, SurfaceConfiguration};

use crate::{graphics::{ Vertex, render_pipeline, GraphicsBundle, GraphicsSettings, Mesh, InstanceRaw, Material}, components::{Transform, transform::Rotation}};


pub trait DrawModel<'a> {
    fn draw_mesh(&mut self, device: &wgpu::Device, pipeline: &'a wgpu::RenderPipeline, model: &'a Model, mesh: &'a Mesh);
    fn draw_mesh_instanced(
        &mut self,
        device: &wgpu::Device,
        model: &'a Model,
        mesh: &'a Mesh,
        pipeline: &'a wgpu::RenderPipeline,
        instances: Range<u32>,
    );
}
/*
impl <'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a> where 'b: 'a, {
    fn draw_mesh(&mut self, device: &wgpu::Device, pipeline: &'b wgpu::RenderPipeline, model: &'b Model, mesh: &'b Mesh) {
        self.draw_mesh_instanced(device, model, mesh, pipeline, 0..1);
    }
    fn draw_mesh_instanced(&mut self, device: &wgpu::Device, model: &'b Model, mesh: &'b Mesh, pipeline: &'b wgpu::RenderPipeline, instances: Range<u32>) {
        let tex_bind_group = match &model.get_mesh_material(&mesh).diffuse.bind_group {
            Some(v) => v,
            None=> {panic!("material using texture without a bind group")}
        };
        self.set_pipeline(pipeline);
        self.set_bind_group(1, tex_bind_group, &[]);
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.draw_indexed(0..mesh.element_count, 0, 0..model.instances.len() as _);
    }
}
*/

#[derive(Debug)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    pub instances: Vec<Transform>,

}

impl Model {
    pub fn new(meshes: Vec<Mesh>, materials: Vec<Material>, instances: Vec<Transform>) -> Self {
        
        Model {
            meshes,              
            materials,           
            instances: instances,
        }
    }



    pub fn to_instance_raw(transform: &Transform) -> InstanceRaw {
        
        let matrix = Mat4::from_scale_rotation_translation(
            Vec3::new(1.0, 1.0, 1.0), // scale
            transform.rotation, // rotation
            transform.position // position
        );
        InstanceRaw {
            model: matrix.to_cols_array_2d(),
        }
    }

    
}