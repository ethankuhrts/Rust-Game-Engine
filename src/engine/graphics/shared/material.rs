use crate::{graphics::{GraphicsBundle, render_pipeline, Texture}, assets::asset_manager::AssetRef};

#[derive(Debug)]
pub struct Material {
    pub name: String,
    pub render_pipeline: AssetRef,
    pub diffuse: AssetRef,
}

impl Material {
    pub fn new (render_pipeline: AssetRef, diffuse: AssetRef, name: &str) -> Self {
        Material { name: String::from(name), render_pipeline: render_pipeline, diffuse }
    }

    
}