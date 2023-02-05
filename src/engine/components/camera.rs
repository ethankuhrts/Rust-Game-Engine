use std::f32::consts::FRAC_PI_2;

use glam::{Mat4, Vec3, Vec4, Quat, EulerRot};
use wgpu::util::DeviceExt;

use crate::{
    ecs::{ Component, VecStorage }, components::{Transform}, graphics::GraphicsBundle
};

use super::transform::{Position, Rotation};


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Camera {
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub uniform: CameraUniform,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,

}

impl Camera {
    #[rustfmt::skip]
    pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array(&[
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
    ]);

    pub const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

    pub fn bind_group_layout(bundle: &GraphicsBundle) -> wgpu::BindGroupLayout {
        return bundle.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });
    }

    

    pub fn new(
        bundle: &GraphicsBundle,
        width: u32,
        height: u32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        let uniform = CameraUniform::new();

        let buffer = bundle.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let bind_group = bundle.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &Camera::bind_group_layout(bundle),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
            uniform,
            buffer,
            bind_group

        }
    }

    
    pub fn calc_matrix(&self, transform: &Transform) -> Mat4 {
        let direction = Quat::mul_vec3(transform.rotation, Vec3::new(0.0, 0.0, 1.0));
        let transform_matrix = Mat4::look_to_rh(
            transform.position,
            direction,
            Vec3::new(0.0, 1.0, 0.0),
        );
        let projection_matrix = Camera::OPENGL_TO_WGPU_MATRIX * Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar);
        return projection_matrix * transform_matrix;
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn update_view_proj(&mut self, transform: &Transform) {
        self.uniform.view_position = Vec4::from((transform.position, 0.0)).into();
        self.uniform.view_proj = self.calc_matrix(transform).to_cols_array_2d();
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_position: [f32; 4],
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_position: [0.0; 4],
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
}