use std::{collections::HashMap, io::{self, Cursor, BufReader, BufRead}, fs, path::{Path}, ops::DerefMut};

use indexmap::IndexMap;
use wgpu::RenderPipeline;

use crate::graphics::{Mesh, GraphicsBundle, Texture, Material};




pub enum Error {
    FileNotFound,
    FileTooLarge,
    WGPUUnknown,
    NullReference,
    AssetNotFound
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotFound => write!(f, "AssetManager : File not found, make sure the file is in the correct folder (default '../res/')"),
            Self::FileTooLarge => write!(f, "AssetManager : File too large."),
            Self::WGPUUnknown => write!(f, "AssetManager : Unknown error occured within WGPU"),
            Self::NullReference => write!(f, "AssetManager : AssetRef does not contain a name OR id"),
            Self::AssetNotFound => write!(f, "AssetManager : Asset not registered"),
        }
    }
}


#[derive(Debug)]
pub struct AssetManager {
    meshes: IndexMap<String, Mesh>,
    textures: IndexMap<String, Texture>,
    pub materials: IndexMap<String, Material>,
    render_pipelines: IndexMap<String, wgpu::RenderPipeline>,
    shaders: IndexMap<String, wgpu::ShaderModule>,
}

impl AssetManager {
    pub fn new() -> Self {
        let meshes = IndexMap::new();
        let textures = IndexMap::new();
        let materials = IndexMap::new();
        let render_pipelines = IndexMap::new();
        let shaders = IndexMap::new();
        AssetManager { meshes, textures, materials, render_pipelines, shaders }
    }

    /// loads a file as bytes
    pub async fn load_bytes(file_name: &str) -> Result<Vec<u8>, io::Error> {
        let path = Path::new(env!("OUT_DIR"))
            .join("res")
            .join(file_name);
        fs::read(path)
    }
    /// loads a file as a String
    pub async fn load_string(file_name: &str) -> Result<String, io::Error> {
        let path = Path::new(env!("OUT_DIR"))
            .join("res")
            .join(file_name);
        fs::read_to_string(path)
    }

    pub async fn load_texture(&mut self, bundle: &GraphicsBundle<'_>, file_name: &str) -> Result<AssetRef, Error> {
        let bytes: Vec<u8>;
        match AssetManager::load_bytes(file_name).await {
            Ok(res) => {bytes = res},
            Err(err) => {return Err(Error::FileNotFound)}
        };
        let texture: Texture;
        match Texture::from_bytes(bundle, &bytes, file_name) {
            Ok(res) => {texture = res},
            Err(err) => {return Err(Error::WGPUUnknown)},
        };
        
        let name = Path::new(&file_name).file_stem().and_then(|s| s.to_str()).unwrap();
        
        self.textures.insert(String::from(name), texture);
        
        let id = self.textures.len() - 1;
        Ok(AssetRef {
            id: Some(id),
            name: Some(String::from(name)),
            asset_type: AssetType::Texture,
        })


    }

    pub fn get_texture (&self, target: AssetRef) -> Result<&Texture, Error> {
        let storage = &self.textures;
        if target.id.is_some() {
            match storage.get_index(target.id.unwrap()) {
                Some(res) => {return Ok(res.1)},
                None => {}
            }
        }
        if target.name.is_some() {
            let name = &target.name.as_ref().unwrap().clone();
            match storage.get(name) {
                Some(res) => {return Ok(res)},
                None => {return Err(Error::AssetNotFound)}
            }
        }
        return Err(Error::NullReference);
    }
    pub fn get_mesh (&self, target: AssetRef) -> Result<&Mesh, Error> {
        let storage = &self.meshes;
        if target.id.is_some() {
            match storage.get_index(target.id.unwrap()) {
                Some(res) => {return Ok(res.1)},
                None => {}
            }
        }
        if target.name.is_some() {
            let name = &target.name.as_ref().unwrap().clone();
            match storage.get(name) {
                Some(res) => {return Ok(res)},
                None => {return Err(Error::AssetNotFound)}
            }
        }
        return Err(Error::NullReference);
    }
    pub fn get_material (&self, target: AssetRef) -> Result<&Material, Error> {
        let storage = &self.materials;
        if target.id.is_some() {
            match storage.get_index(target.id.unwrap() - 1) {
                Some(res) => {return Ok(res.1)},
                None => {}
            }
        }
        if target.name.is_some() {
            let name = &target.name.as_ref().unwrap().clone();
            match storage.get(name) {
                Some(res) => {return Ok(res)},
                None => {return Err(Error::AssetNotFound)}
            }
        }
        return Err(Error::NullReference);
    }

    pub fn get_pipeline (&self, target: AssetRef) -> Result<&RenderPipeline, Error> {
        let storage = &self.render_pipelines;
        if target.id.is_some() {
            match storage.get_index(target.id.unwrap()) {
                Some(res) => {return Ok(res.1)},
                None => {}
            }
        }
        if target.name.is_some() {
            let name = &target.name.as_ref().unwrap().clone();
            match storage.get(name) {
                Some(res) => {return Ok(res)},
                None => {return Err(Error::AssetNotFound)}
            }
        }
        return Err(Error::NullReference);
    }

    pub fn insert_asset<T: Asset>(&mut self, asset: T, asset_type: AssetType, name: &str) -> AssetRef {
        let id = asset.insert(self, name);
        
        AssetRef {
            id: Some(id),
            name: Some(String::from(name)),
            asset_type: asset_type,
        }
    }

    /*pub fn validate<T, E>(input: Result<T, io::Error>) -> T {
        match input {
            Ok(res) => {return res}
            Err(e) => {
                match e. {
                    io::ErrorKind::FileTooLarge => { println!("")}
                }
            }
        }
    }
    */
    /*
    /// Loads First mesh found in file and registers it in the asset manager. Returns a Reference struct for the mesh.
    pub async fn load_mesh(&mut self, bundle: &GraphicsBundle<'_>, file_name: &str) -> Result<Vec<AssetRef>, io::Error> {
        let bytes: Vec<u8>;
        match AssetManager::load_bytes(file_name).await {
            Ok(res) => {bytes = res},
            Err(e) => {return Err(e)}
        };

        let mut reader = BufReader::new(Cursor::new(String::));

        for line in reader.lines() {
            let (line, mut words) = match line {
                Ok(ref line) => (&line[..], line[..].split_whitespace()),
                Err(_e) => {
                    
                    println!("load_obj - failed to read line due to {}", _e);
                    return Err(_e);
                }
            };

            println!("{:?}", line);
        }
        
        
        Ok(vec!(
            AssetRef {
                id: 0,
                name: file_name.to_string(),
                asset_type: AssetType::Mesh,
            }
        ))
    }
    */
}

#[derive(Debug, Copy, Clone)]
pub enum AssetType {
    Mesh, Texture, Material, Shader, RenderPipeline
}

pub trait Asset: Sized {
    fn insert(self, asset_manager: &mut AssetManager, name: &str) -> usize {
        0
    } 
}
impl Asset for Texture {
    fn insert(self, asset_manager: &mut AssetManager, name: &str) -> usize {
        asset_manager.textures.insert(name.to_owned(), self);
        asset_manager.textures.len()
    } 
}
impl Asset for Mesh {
    fn insert(self, asset_manager: &mut AssetManager, name: &str) -> usize {
        asset_manager.meshes.insert(name.to_owned(), self);
        asset_manager.meshes.len()
    } 
}
impl Asset for wgpu::ShaderModule {
    fn insert(self, asset_manager: &mut AssetManager, name: &str) -> usize {
        asset_manager.shaders.insert(name.to_owned(), self);
        asset_manager.shaders.len()
    } 
}
impl Asset for Material {
    fn insert(self, asset_manager: &mut AssetManager, name: &str) -> usize {
        asset_manager.materials.insert(name.to_owned(), self);
        asset_manager.materials.len()
    } 
}
impl Asset for RenderPipeline {
    fn insert(self, asset_manager: &mut AssetManager, name: &str) -> usize {
        asset_manager.render_pipelines.insert(name.to_owned(), self);
        asset_manager.render_pipelines.len()
    } 
}


#[derive(Debug)]
pub struct AssetRef {
    pub id: Option<usize>,
    pub name: Option<String>,
    pub asset_type: AssetType,
}

impl Clone for AssetRef {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), name: self.name.clone(), asset_type: self.asset_type.clone() }
    }
}


impl AssetRef {
    
    pub fn new(id: Option<usize>, name: Option<String>, asset_type: AssetType) -> Self {
        AssetRef { id: id, name: name, asset_type: asset_type }
    }


    pub fn validate(&self) -> Result<(bool, bool), Error> {
        let has_name = match &self.name {
            Some(res) => true,
            None => false,
        };
        let has_id = match &self.id {
            Some(res) => true,
            None => false,
        };

        if !has_id && !has_id {
            return Err(Error::NullReference);
        } else {
            return Ok((has_id, has_name));
        }
        
    }
}