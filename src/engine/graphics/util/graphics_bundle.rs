use crate::graphics::GraphicsSettings;

pub struct GraphicsBundle<'a> {
    pub device: &'a wgpu::Device,
    pub surface: &'a wgpu::Surface,
    pub config: &'a wgpu::SurfaceConfiguration,
    pub settings: &'a GraphicsSettings,
    pub queue: &'a wgpu::Queue,
}