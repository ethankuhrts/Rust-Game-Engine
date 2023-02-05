
pub struct GraphicsSettings {
    pub power_preference: wgpu::PowerPreference,
    pub alpha_mode:       wgpu::CompositeAlphaMode,
    pub present_mode:     wgpu::PresentMode,
    pub cull_back_face:   bool,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            power_preference: wgpu::PowerPreference::LowPower,
            alpha_mode:       wgpu::CompositeAlphaMode::Auto,
            present_mode:     wgpu::PresentMode::Fifo,
            cull_back_face:   true,
        }
    }
}

