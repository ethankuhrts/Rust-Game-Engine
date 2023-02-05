use specs::{Component, VecStorage};
use wgpu::util::DeviceExt;

use crate::graphics::Color;

use super::{transform::Position, Transform};



#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    position: [f32; 3],
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    _padding: u32,
    color: [f32; 3],
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    _padding2: u32,
}

impl LightUniform {
    pub fn new(color: Color) -> Self {
        LightUniform { position: [0.0; 3], _padding: 0, color: color.into(), _padding2: 0 }
    }
    pub fn update(&mut self, color: Color, transform: &Transform) {
        self.position = transform.position.into();
        self.color = color.into();
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Light {
    pub intensity: f32,
    pub color: Color,

    pub uniform: LightUniform,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl Light {
    pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        })
    }
    pub fn new(device: &wgpu::Device, color: Color, intensity: f32) -> Self {
        let uniform = LightUniform::new(Color::GREEN);
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Light VB"),
                contents: bytemuck::cast_slice(&[uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let bind_group_layout = Light::bind_group_layout(device);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
        });
        Light {intensity, color, uniform, buffer, bind_group}
    }

    pub fn update_uniform(&mut self, transform: &Transform) {
        self.uniform.update(self.color, transform);
    }
}