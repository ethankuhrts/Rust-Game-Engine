
#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

impl Color {
    pub const RED: Self = Color{ r: 1.0, g: 0.0, b: 0.0, a: 1.0};
    pub const GREEN: Self = Color{ r: 0.0, g: 1.0, b: 0.0, a: 1.0};
    pub const BLUE: Self = Color{ r: 0.0, g: 0.0, b: 1.0, a: 1.0};
    pub const BLACK: Self = Color{ r: 0.0, g: 0.0, b: 0.0, a: 1.0};
    pub const WHITE: Self = Color{ r: 1.0, g: 1.0, b: 1.0, a: 1.0};
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Color { r, g, b, a }
    }

    pub fn to_wgpu(&mut self) -> wgpu::Color {
        wgpu::Color {r: self.r, g: self.g, b: self.b, a: self.a}
    }

}
impl Into<[f32; 3]> for Color {
    fn into(self) -> [f32; 3] {
        [self.r as f32, self.g as f32, self.b as f32]
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}