use std::ops::Range;

use glam::{Mat4, Vec3};
use specs::{Component, VecStorage};
use wgpu::{util::DeviceExt, Device};

use crate::{assets::{asset_manager::{AssetRef, AssetType, self, Asset}, AssetManager}, graphics::{GraphicsBundle, InstanceRaw, Mesh, Material, Texture, Renderable}};

use super::Transform;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub mesh: AssetRef,
    pub material: AssetRef,
    pub instance_buffer: Option<wgpu::Buffer>,
}

impl Default for Sprite {
    fn default() -> Self {
        Sprite { 
            mesh: AssetRef { id: None, name: Some(String::from("default_sprite_plane")), asset_type: AssetType::Mesh },
            material: AssetRef { id: None, name: Some(String::from("default_material")), asset_type: AssetType::Material },
            instance_buffer: None,
        }
    }
}

impl Sprite {
    pub fn new(material: AssetRef, mesh: AssetRef) -> Self{
        Sprite { material: material, instance_buffer: None, mesh: mesh }
    }

    pub fn generate_instance_buffer(&mut self, bundle: &GraphicsBundle, instances: Vec<&Transform>) -> wgpu::Buffer {
        let instance_data = instances.iter().map(|x| { Sprite::to_instance_raw(x) }).collect::<Vec<_>>();
        let instance_buffer = bundle.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        instance_buffer
    }

    pub fn update_instance_buffer(&mut self, device: &Device, instances: Vec<&Transform>) {
        let instance_data = instances.iter().map(|x| { Sprite::to_instance_raw(x) }).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        //return instance_buffer;
        self.instance_buffer = Some(instance_buffer);
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


impl Renderable for Sprite {
    fn get_mesh<'a>(&self, asset_manager: &'a AssetManager) -> &'a Mesh { 
        return asset_manager.get_mesh(self.mesh.clone()).unwrap();
    }
    fn get_material<'a>(&self, asset_manager: &'a AssetManager) -> &'a Material {
        return asset_manager.get_material(self.material.clone()).unwrap();
    }

    fn get_instance_buffer(&mut self, device: &wgpu::Device, instances: Vec<&Transform>) { 
        let instance_data = instances.iter().map(|x| { Sprite::to_instance_raw(x) }).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        //return instance_buffer;
        self.instance_buffer = Some(instance_buffer);
    }

    fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>, device: &wgpu::Device, asset_manager: &'a AssetManager, instances: &Transform) {
        let mesh: &Mesh = self.get_mesh(asset_manager);
        let material: &Material = self.get_material(asset_manager);
        let pipeline: &wgpu::RenderPipeline = asset_manager.get_pipeline(material.render_pipeline.clone()).unwrap();
        let diffuse: &Texture = asset_manager.get_texture(material.diffuse.clone()).unwrap();

        
        let tex_bind_group = match &diffuse.bind_group {
            Some(v) => v,
            None=> {panic!("material using texture without a bind group")}
        };
        self.get_instance_buffer(device, vec![instances]);
        let instance_buffer= self.instance_buffer.as_ref().unwrap();

        
        render_pass.set_bind_group(1, tex_bind_group, &[]);
        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        
        render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
        
        render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..mesh.element_count, 0, 0..1 as _);
    }
}
/*
pub trait DrawSprite<'a> {
    fn draw_sprite(&mut self, device: &wgpu::Device, asset_manager: &'a AssetManager, sprite: &'a mut Sprite, instances: Vec<&Transform>);
    fn draw_sprite_instanced(
        &mut self,
        device: &wgpu::Device,
        sprite: &'a mut Sprite,
        asset_manager: &'a AssetManager,
        instances: Vec<&Transform>,
    );
}

impl <'a, 'b> DrawSprite<'a> for wgpu::RenderPass<'a> where 'b: 'a, {


    fn draw_sprite(&mut self, device: &wgpu::Device, asset_manager: &'a AssetManager, sprite: &'a mut Sprite, instances: Vec<&Transform>) {
        self.draw_sprite_instanced(device, sprite, asset_manager, instances);
    }
    fn draw_sprite_instanced(&mut self,
        device: &wgpu::Device,
        sprite: &'a mut Sprite,
        asset_manager: &'a AssetManager,
        instances: Vec<&Transform>) 
    {
        
        let mesh: &Mesh = sprite.get_mesh(asset_manager);
        let material: &Material = sprite.get_material(asset_manager);
        let pipeline: &wgpu::RenderPipeline = asset_manager.get_pipeline(material.render_pipeline.clone()).unwrap();
        let diffuse: &Texture = asset_manager.get_texture(material.diffuse.clone()).unwrap();

        
        let tex_bind_group = match &diffuse.bind_group {
            Some(v) => v,
            None=> {panic!("material using texture without a bind group")}
        };

        let instance_buffer: &wgpu::Buffer;
        let instance_count = instances.len();
        sprite.update_instance_buffer(device, instances);

        match &sprite.instance_buffer {
            Some(res) => {instance_buffer = res},
            None => { panic!("Instance buffer for sprite {:?} is None", sprite) }
        }

        self.set_pipeline(pipeline);
        self.set_bind_group(1, tex_bind_group, &[]);
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        
        self.set_vertex_buffer(1, instance_buffer.slice(..));
        
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.draw_indexed(0..mesh.element_count, 0, 0..instance_count as _);
    }
}

*/