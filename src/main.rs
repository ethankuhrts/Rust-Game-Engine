use std::{sync::Arc, cell::RefCell};

use winit::{
    event_loop::{EventLoop},
    window::{
        Window,
    }
};


use wvreng::{
    GameState,
    ecs::{
        Builder, DispatcherBuilder, WorldExt,
    },
    components::{
        Transform, ModelRenderer, Camera, transform::EulerRotation, Sprite, Light,
    }, systems::CameraController, assets::{AssetManager, asset_manager::{AssetType, AssetRef}}, graphics::{util::MeshPrimitives, Material, Color},
};



fn main() {
    let _start_time = chrono::Local::now().timestamp_millis();
    let (window, event_loop): (Window, EventLoop<()>) = wvreng::init_window();
    let mut game: GameState = pollster::block_on(GameState::new(&window));
    let dispatcher_builder = DispatcherBuilder::new();

    game.world.register::<Transform>();
    game.world.register::<Sprite>();
    game.world.register::<Camera>();
    game.world.register::<ModelRenderer>();
    game.world.register::<Light>();

    let wall_texture = 
        pollster::block_on(game.asset_manager.load_texture(&game.renderer.get_graphics_bundle(), "download.png")).unwrap();
    
    let light_icon = 
        pollster::block_on(game.asset_manager.load_texture(&game.renderer.get_graphics_bundle(), "light-icon.png")).unwrap();
    
    
    let wall_material = game.asset_manager.insert_asset(
        Material::new(AssetRef::new(None, Some("Universal".to_string()), AssetType::RenderPipeline), wall_texture, "wall_material"),
        AssetType::Material, 
        "wall_material"
    );
    let light_icon_mat = game.asset_manager.insert_asset(
        Material::new(AssetRef::new(None, Some("Debug Icon".to_string()), AssetType::RenderPipeline), light_icon, "light_icon"),
        AssetType::Material, 
        "light_icon_mat"
    );

    let plane = game.asset_manager.insert_asset(
        MeshPrimitives::plane(&game.renderer.get_graphics_bundle(), "plane"),
        AssetType::Mesh,
        "plane",
    );
    
    for x in 0..10 as u32 {
        for y in 0..10 as u32 {
            game.world.create_entity()
            .with(Transform::new(x as f32 * 2.0 - 5.0, y as f32 / 2.0, y as f32 * 2.0 - 5.0))
            .with(Sprite::new(wall_material.clone(), plane.clone()))
            .build();
        }
    }
    game.world.create_entity()
        .with(Transform::new(0.0, 2.0, 0.0))
        .with(Sprite::new(light_icon_mat.clone(), plane.clone()))
        .with(Light::new(&game.renderer.device, Color::BLUE, 5.0))
        .build();
    
    // camera
    let bundle = &game.renderer.get_graphics_bundle();
    let mut camera = Camera::new(
        &bundle,
        bundle.config.width, bundle.config.height, 45.0, 0.1, 100.0, 
    );
    camera.update_view_proj(&Transform::new(0.0, 0.0, 0.0));
    game.world.create_entity()
        .with(camera)
        .with(Transform::new(0.0, 0.0, 0.0))
        .build();

    game.init_internal_resources(&window);
    
    game.register_quit_event(move || {
        println!("Shutting down");
    });


    let dispatcher = dispatcher_builder
        //.with(TransformSystem, "transform_system", &[])
        .with(CameraController, "camera_controller", &[])
        .build();
    
    println!("Game loaded in {:?} milliseconds!", chrono::Local::now().timestamp_millis() - _start_time);
    pollster::block_on(wvreng::run(game, window, event_loop, dispatcher));
}
