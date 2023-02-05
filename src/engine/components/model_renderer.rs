use std::{sync::Arc, cell::RefMut};

use wgpu::util::DeviceExt;

use crate::{
    ecs::{Component, VecStorage},
    graphics::{Mesh, Model, GraphicsBundle}, assets::AssetManager
};

use super::Transform;


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ModelRenderer {
    pub model_id: i64,
    pub instance_buffer: Option<wgpu::Buffer>,


}

impl ModelRenderer {
    pub fn new(asset_manager: Arc<AssetManager>, model_id: i64) -> Self{
        ModelRenderer { model_id: model_id, instance_buffer: None }
    }
    
    pub fn init_instance_buffer(&mut self, bundle: &GraphicsBundle, instances: Vec<&Transform>) {
        let instance_data = instances.iter().map(|x| { Model::to_instance_raw(x) }).collect::<Vec<_>>();
        let instance_buffer = bundle.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        //return instance_buffer;
        self.instance_buffer = Some(instance_buffer);
    }
}
