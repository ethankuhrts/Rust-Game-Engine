use std::{
    mem,
    borrow::BorrowMut, sync::Arc
};


use chrono::{DateTime, Utc};
use winit::{window::Window, event::{WindowEvent, VirtualKeyCode, KeyboardInput, ElementState}};
use specs::{ World, WorldExt, Builder, shrev::Event, System, DispatcherBuilder, Dispatcher, shred::{FetchMut, Fetch} };

use crate::{
    graphics::{Renderer, GraphicsSettings
        //Renderer,
    },
    input::{
        Input,
    }, assets::AssetManager,
};
pub struct GameEvent {
    pub f: Box<dyn FnMut() + 'static>,
}
impl GameEvent {
    pub fn new<F>(f: F) -> GameEvent where F: FnMut() + 'static {
        GameEvent {
            f: Box::new(f),
        }
    }
    pub fn run(&mut self) {
        (self.f)();
    }
}

pub struct GameState {
    pub world: World,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub renderer: Renderer,
    pub asset_manager: AssetManager,
    
    exit_events: Vec<GameEvent>,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Time {
    pub start_time: i64,
    pub time_since_start: i64,
    old_time_since_start: i64,
    pub frame: i64,
    pub delta: f32,
}

impl Time {
    pub fn new() -> Self {
        Self { start_time: chrono::Utc::now().timestamp_millis(), old_time_since_start: 0, time_since_start: 0, frame: 0, delta: 0.0 }
    }
    pub fn frame_step(&mut self) {
        self.frame += 1;
    }
    pub fn time_step(&mut self) {
        let millis = chrono::Utc::now().timestamp_millis();
        self.time_since_start = millis - self.start_time;
        self.delta = (self.time_since_start as f32 - self.old_time_since_start as f32) * 0.001;
        self.old_time_since_start = self.time_since_start;
    }

}



impl GameState {
    pub async fn new(window: &Window) -> GameState {
        let world = World::new();
        let size = window.inner_size();
        let exit_events = Vec::new();
        let dispatcher = DispatcherBuilder::new();
        let mut asset_manager = AssetManager::new();
        let renderer = Renderer::new(window, &mut asset_manager, GraphicsSettings::default()).await;
        
        GameState { 
            world,
            renderer,
            size,
            exit_events,
            asset_manager,
        }
    }

    pub fn init_internal_resources(&mut self, window: &Window) {
        self.world.insert(Input::new());
        self.world.insert(Time::new());
    } 


    pub fn register_quit_event<F>(&mut self, f: F) where F: FnMut() + 'static {
        self.exit_events.push(GameEvent::new(f));
    }

    pub async fn update(&mut self, window: &Window, dispatcher: &mut Dispatcher<'_, '_>) -> bool {
        let mut time = self.world.write_resource::<Time>();
        let mut input = self.world.write_resource::<Input>();
        time.frame_step();
        time.time_step();
        input.window_update(window);
        
        drop(time);
        drop(input);
        
        dispatcher.dispatch(&self.world);

        self.renderer.update(&self.world);
        
        return true;    
    }

    pub fn on_exit(&mut self) -> bool {
        println!("Game Quit");
        for num in 0..self.exit_events.len() {
            self.exit_events[num].run();
        }
        return true;
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        return match event {
            _ => { false }
        }
    }

    pub fn register_system() {

    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        let mut input = self.world.write_resource::<Input>();
        println!("Window resized: {:?} : {:?}", new_size.width, new_size.height);
        self.renderer.resize(&self.world, new_size);
        input.resize(new_size);
    }
}
