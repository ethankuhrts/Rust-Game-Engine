use std::{collections::HashMap, sync::Arc, rc::Rc, borrow::Cow};

use glam::Vec3;
use specs::{WorldExt, Join};
use winit::window::Window;

use crate::{
    resources, ecs::{World}, 
    components::{ModelRenderer, model_renderer, Camera, CameraUniform, Transform, Sprite, Light},
    graphics::{
        GraphicsSettings, render_pipeline, Vertex, GraphicsBundle,
        Model, DrawModel, Material, Mesh, util::MeshPrimitives, Texture, shared::{material, instance}
    }, assets::{AssetManager, asset_manager::{AssetType, AssetRef}}};


use wgpu::{util::DeviceExt, RenderPipeline, PipelineLayoutDescriptor, PipelineLayout, SurfaceTexture, RenderPass, Device};



pub struct Renderer {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub settings: GraphicsSettings,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub universal_pipeline_layout: wgpu::PipelineLayout,


    //pub render_pipeline: wgpu::RenderPipeline,

    pub depth_texture: Texture,
}

pub trait Renderable {
    fn get_mesh<'a>(&self, asset_manager: &'a AssetManager) -> &'a Mesh { todo!() }
    fn get_material<'a>(&self, asset_manager: &'a AssetManager) -> &'a Material { todo!() }
    fn get_instance_buffer(&mut self, device: &wgpu::Device, instances: Vec<&Transform>) { todo!() }

    fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>, device: &wgpu::Device, asset_manager: &'a AssetManager, instances: &Transform) {
        
    }
}

impl Renderer {
    pub async fn new(window: &Window, asset_manager: &mut AssetManager, settings: GraphicsSettings) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: settings.power_preference,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: settings.present_mode,
            alpha_mode: settings.alpha_mode,
        };

        surface.configure(&device, &config);

        let bundle = GraphicsBundle {device: &device, surface: &surface, config: &config, settings: &settings, queue: &queue};
        

        let depth_texture = Texture::create_depth_texture(&bundle, "depth_texture");


        let universal_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/universal.wgsl").into()),
        });

        let universal_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &Camera::bind_group_layout(&bundle), 
                &Texture::bind_group_layout(&bundle),
                &Light::bind_group_layout(&bundle.device),
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline = render_pipeline::create_render_pipeline(&bundle, &universal_pipeline_layout, &universal_shader, false, true);
        
        let debug_icon_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Light Pipeline Layout"),
                bind_group_layouts: &[
                    &Camera::bind_group_layout(&bundle), 
                    &Texture::bind_group_layout(&bundle),
                ],
                push_constant_ranges: &[],
            });
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Debug Icon Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shaders/debug_icon.wgsl").into()),
            });
            render_pipeline::create_render_pipeline(&bundle, &layout, &shader, true, false)
        };
        

        asset_manager.insert_asset(render_pipeline, AssetType::RenderPipeline, "Universal");
        asset_manager.insert_asset(debug_icon_pipeline, AssetType::RenderPipeline, "Debug Icon");
        
        

        Self {
            surface,
            device,
            queue,
            config,
            settings,
            size,
            universal_pipeline_layout,
    
            depth_texture,
        }
    }
    /*
    pub fn register_shader(&mut self, asset_manager: &mut AssetManager, shader: &wgpu::ShaderModule, name: &str, pipeline_layout: Option<PipelineLayout>) {
        let layout = match &pipeline_layout {
            Some(res) => &res,
            None => &self.universal_pipeline_layout,
        };
        let pipeline = render_pipeline::create_render_pipeline(&self.get_graphics_bundle(), layout, shader, "vs_main", "fs_main");

        asset_manager.insert_asset(pipeline, AssetType::RenderPipeline, "Universal");
    }
    */


    pub fn build_model(&mut self, meshes: Vec<Mesh>, materials: Vec<Material>, transforms: Vec<Transform>) -> Model {
        Model::new(meshes, materials, transforms)
    }
    


    pub fn render(&mut self, asset_manager: &AssetManager, world: &World) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
       

        let mut models = world.write_storage::<ModelRenderer>();
        let mut sprites = world.write_storage::<Sprite>();
        let mut cameras = world.write_storage::<Camera>();
        let mut transforms = world.write_storage::<Transform>();
        let mut lights = world.write_storage::<Light>();

        let mut camera_position: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let mut current_pipeline: &RenderPipeline;
        
        
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(super::Color::new(0.2, 0.4, 0.8, 1.0).to_wgpu()),
                            store: true,
                        }
                    }
                )],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
            
            
            let mut camera_iter = 0;
            for (camera, transform) in (&mut cameras, &mut transforms).join() {
                if camera_iter != 0 {break;}

                camera.update_view_proj(transform);
                render_pass.set_bind_group(0, &camera.bind_group, &[]);
                
                self.queue.write_buffer(&camera.buffer, 0, bytemuck::cast_slice(&[camera.uniform]));

                camera_position = transform.position;
                camera_iter += 1;
            }  

            for (light, transform) in (&mut lights, &mut transforms).join() {
                light.update_uniform(transform);

                render_pass.set_bind_group(2, &light.bind_group, &[]);
                self.queue.write_buffer(&light.buffer, 0, bytemuck::cast_slice(&[light.uniform]));
            }
            
            
            /*
            for (model, transform) in (&mut models, &transforms).join() {
                let bundle = self.get_graphics_bundle();
                
                model.init_instance_buffer(&bundle, vec![&transform]);

                match &model.instance_buffer {
                    Some(res) => {render_pass.set_vertex_buffer(1, res.slice(..));},
                    None => {}
                };    
                /*let model: &Model = &model.get_model();
                
                for mesh in &model.model.meshes {
                    let pipeline = self.render_pipelines.get(&model.model.get_mesh_material(&mesh).pipeline).unwrap();
                    render_pass.draw_mesh(&self.device, pipeline, &model.model, mesh);
                    
                }
                */
            }
            */

            let mut draw_queue: Vec<(&AssetRef, Box<&mut dyn  Renderable>, &Transform)> = Vec::new();
            for (sprite, transform) in (&mut sprites, &transforms).join() {
                let bundle = self.get_graphics_bundle();
                
                //let pipeline = self.render_pipelines.get("Universal").unwrap();
                let material: &Material = sprite.get_material(asset_manager);
                let pipeline_ref = &material.render_pipeline;

                if transform.position.distance(camera_position) < 100.0 {
                    draw_queue.push((
                        pipeline_ref, 
                        Box::new(sprite),
                        transform,
                    ))
                    //render_pass.draw_sprite(&self.device, asset_manager, sprite, vec![transform]);
                }                          
            }       

            let mut current_pipeline_ref: &AssetRef = &AssetRef::new(None, None, AssetType::RenderPipeline);
            let mut current_pipeline: &wgpu::RenderPipeline;
            for renderable in draw_queue {
                let (pipeline_ref, object, transform) = renderable;

                if current_pipeline_ref.name.is_none() || &pipeline_ref.name.as_ref().unwrap() != &current_pipeline_ref.name.as_ref().unwrap() {
                    current_pipeline_ref = pipeline_ref;
                    current_pipeline = asset_manager.get_pipeline(current_pipeline_ref.clone()).unwrap();
                    //render_pass.set_pipeline(current_pipeline);
                }
                render_pass.set_pipeline(asset_manager.get_pipeline(current_pipeline_ref.clone()).unwrap());
                object.draw(&mut render_pass, &self.device, &asset_manager, transform);
                
            }


        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn get_graphics_bundle(&self) -> GraphicsBundle {
        GraphicsBundle { device: &self.device, surface: &self.surface, config: &self.config, settings: &self.settings, queue: &self.queue }
    }

    pub fn update(&mut self, world: &World) {
        
    }

    pub fn resize(&mut self, world: &World, new_size: winit::dpi::PhysicalSize<u32>) {
        
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }

        let mut cameras = world.write_component::<Camera>();
        let mut transforms = world.write_component::<Transform>();

        for (camera, transform) in (&mut cameras, &mut transforms).join() {
            camera.resize(new_size.width, new_size.height)
        }

        let bundle = self.get_graphics_bundle();

        self.depth_texture = Texture::create_depth_texture(&bundle, "depth_texture")
    }
}