#![allow(
    unused_variables,
    dead_code,
    unused_imports,
)]

use std::{default, borrow::BorrowMut};

use ecs::{Dispatcher, WorldExt, World};
use game::Time;
use wgpu::{
    include_wgsl,
    util::{
        DeviceExt,
    }
};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{
        WindowBuilder,
        Window,
        
    }
};

pub mod game;
pub mod input;
pub mod graphics;
pub mod components;
pub mod systems;
pub mod assets;

pub use assets::resources as resources;
pub use input::Input as Input;

pub use specs as ecs;

pub use crate::{    
    game::GameState as GameState,
    graphics::{
        //Renderer,
    }
};

pub fn init_window() -> (Window, EventLoop<()>) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    return (window, event_loop);
}

pub async fn run<'a>(mut game_state: GameState, window: Window, event_loop: EventLoop<()>, mut dispatcher: Dispatcher<'static, 'static>) {
    let start_time = chrono::Local::now().timestamp_millis();
    let mut loaded = false;

    event_loop.run(
        move |event, _, control_flow|{
            pollster::block_on(game_state.update(&window, &mut dispatcher));
            if !loaded {
                println!("Game Started in {:?} milliseconds!", chrono::Local::now().timestamp_millis() - start_time);
            }
            loaded = true;

            match event {
                Event::WindowEvent { ref event, window_id } if window_id == window.id()
                => if !game_state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => { println!("Application Closed"); if game_state.on_exit() { *control_flow = ControlFlow::Exit } },
                        WindowEvent::Resized(physical_size) => {
                            game_state.resize(*physical_size);
                        },
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so we dereference it twice
                            game_state.resize(**new_inner_size);
                        },
                        WindowEvent::KeyboardInput {  // KEY DOWN EVENT
                            input: KeyboardInput { 
                                state: ElementState::Pressed,
                                virtual_keycode,
                                ..
                            },
                            ..
                        } => {
                            match virtual_keycode {
                                Some(key) => {game_state.world.write_resource::<Input>().on_key_down(virtual_keycode.unwrap(), &game_state.world)},
                                None => { print!("unkown key"); }
                            }                            
                        },
                        WindowEvent::KeyboardInput {  // KEY UP EVENT
                            input: KeyboardInput { 
                                state: ElementState::Released,
                                virtual_keycode,
                                ..
                            },
                            ..
                        } => { 
                            match virtual_keycode {
                                Some(key) => {game_state.world.write_resource::<Input>().on_key_up(virtual_keycode.unwrap(), &game_state.world)     },
                                None => { print!("unkown key"); }
                            }  
                        },
                        WindowEvent::CursorMoved { position,  .. } => {
                            game_state.world.write_resource::<Input>().set_cursor_pos((position.x, position.y));
                        }
    
                        _ => {},
                    }
                },
                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion{ delta, },
                    
                    ..
                } => {                    
                    game_state.world.write_resource::<Input>().update_mouse_motion(delta, &window);
                }
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    
                    
                    match game_state.renderer.render(&game_state.asset_manager, &game_state.world) {
                        Ok(_) => {},
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => game_state.resize(game_state.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                        
                    }
                },
                Event::MainEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually request it.
                    window.request_redraw();
                },
    
                _ => {}
            }
            
        }
    )

}