
// mod material;

// mod mesh;
mod shared;
mod renderer;
pub mod util;

pub use renderer::Renderer as Renderer;
pub use renderer::Renderable as Renderable;

pub use shared::material::Material as Material;
pub use shared::mesh::Mesh as Mesh;
pub use shared::model::Model as Model;
pub use shared::model::DrawModel as DrawModel;
pub use shared::vertex::Vertex as Vertex;
pub use shared::texture::Texture as Texture;
//pub use shared::camera::Camera as Camera;
//pub use shared::camera::CameraUniform as CameraUniform;
pub use shared::instance::InstanceRaw as InstanceRaw;

pub use util::graphics_bundle::GraphicsBundle as GraphicsBundle;
pub use util::graphics_settings::GraphicsSettings as GraphicsSettings;
pub use util::color::Color as Color;
pub use util::render_pipeline;



// mod texture;