/*use tobj::LoadError;

use std::{io::{BufReader, Cursor}, error::Error};

use cfg_if::cfg_if;
use wgpu::util::DeviceExt;

use crate::{graphics::{GraphicsBundle, Texture, Model, Material, Mesh, Vertex}, components::{Transform}};

#[cfg(target_arch = "wasm32")]
fn format_url(file_name: &str) -> reqwest::Url {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let base = reqwest::Url::parse(&format!(
        "{}/{}/",
        location.origin().unwrap(),
        option_env!("RES_PATH").unwrap_or("res"),
    )).unwrap();
    base.join(file_name).unwrap()
}

pub async fn load_string(file_name: &str) -> Result<String, bool> {
    
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("res")
        .join(file_name);

    match std::fs::read_to_string(path) {
        Ok(res) => Ok(res),
        Err(err) => Err(false),
    }
}

pub async fn load_binary(file_name: &str) -> Result<Vec<u8>, bool> {
    
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("res")
        .join(file_name);
    match std::fs::read(path) {
        Ok(res) => Ok(res),
        Err(err) => Err(false),
    }
}

pub async fn load_texture(bundle: &GraphicsBundle<'_>, file_name: &str) -> Result<Texture, bool> {
    let data = load_binary(file_name).await;
    match &data {
        Ok(res) => match Texture::from_bytes(bundle, res, file_name) {
            Ok(res) => Ok(res),
            Err(err) => Err(err),
        },
        Err(err) => Err(false),
    }
    
}

pub async fn load_model(
    bundle: &GraphicsBundle<'_>,
    file_name: &str,
    instances: Vec<Transform>,
) -> Result<Model, ()> {
    let obj_text = load_string(file_name).await.unwrap();
    let obj_cursor = Cursor::new(obj_text);
    let mut obj_reader = BufReader::new(obj_cursor);

    let (models, obj_materials) = tobj::load_obj_buf_async(
        &mut obj_reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| async move {
            match load_string(&p).await {
                Ok(res) => {tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(res)))},
                Err(err) => { Err(LoadError::OpenFileFailed) },
            }
            
        },
    )
    .await.unwrap();

    let mut materials = Vec::new();
    match obj_materials {
        Ok(mat) => {
            let obj_mat = mat;
            if obj_mat.len() > 0 {
                for m in obj_mat {
                    let diffuse_texture = match load_texture(&bundle, &m.diffuse_texture).await {
                        Ok(res) => res,
                        Err(err) => {
                            load_texture(&bundle, "cube-diffuse.jpg").await.unwrap()
                        }
                    };
            
                    materials.push(Material::new(&bundle, "Universal", diffuse_texture, &m.name));
                }
            } else {                
                let diffuse_texture: Texture = load_texture(&bundle, "cube-diffuse.jpg").await.unwrap();
        
                materials.push(Material::new(&bundle, "Universal", diffuse_texture, "Generic Material"));
        
            }
        },
        Err(..) => {
            let diffuse_texture: Texture = load_texture(&bundle, "Body_color.png").await.unwrap();
        
            materials.push(Material::new(&bundle, "Universal", diffuse_texture, "Generic Material"));
        }
    }

    let meshes = models
        .into_iter()
        .map(|m| {
            let vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| Vertex {
                    position: [
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ],
                    uvs: match m.mesh.texcoords.len() {
                        0 =>[m.mesh.positions[i * 3], m.mesh.positions[i * 3 + 1]],
                        _ => [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]]
                    },
                    normal: match m.mesh.normals.len() {
                        0 =>[0.0, 0.0, 0.0],
                        _ => [
                            m.mesh.normals[i * 3],
                            m.mesh.normals[i * 3 + 1],
                            m.mesh.normals[i * 3 + 2],
                        ]
                    },
                })
                .collect::<Vec<_>>();

            Mesh::new(bundle, file_name, &vertices, &m.mesh.indices, m.mesh.material_id.unwrap_or(0))
        })
        .collect::<Vec<_>>();

    Ok(Model::new(meshes, materials, instances))
}
*/