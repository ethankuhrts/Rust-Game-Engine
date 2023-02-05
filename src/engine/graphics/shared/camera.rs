use cgmath::{Matrix4, SquareMatrix, Point3, Vector3};
use wgpu::util::DeviceExt;

use crate::graphics::GraphicsBundle;

/*/
#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,

    pub uniform: CameraUniform,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,

}

impl Camera {
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

    pub fn new(bundle: &GraphicsBundle, eye: Point3<f32>, target: Point3<f32>, up: Vector3<f32>, aspect: f32, fovy: f32, znear: f32, zfar: f32) -> Self {
        let view = cgmath::Matrix4::look_at_rh(eye, target, up);
        let proj = cgmath::perspective(cgmath::Deg(fovy), aspect, znear, zfar);        
        let uniform = CameraUniform::new((OPENGL_TO_WGPU_MATRIX * proj * view).into());
        
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
        Camera {
            eye, target, up, aspect, fovy, znear, zfar,
            uniform, bind_group, buffer
        }

    }

    pub fn update_uniform(&mut self) {
        self.uniform.view_proj = self.build_view_projection_matrix().into();
    }


    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {

        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);

        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new(view_proj: [[f32; 4]; 4]) -> Self {
        Self {
            view_proj,
        }
    }
}*/