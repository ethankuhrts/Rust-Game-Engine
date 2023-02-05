use std::{
    collections::HashMap,
    fmt::Debug,
    hash::{Hash, Hasher},
    mem,
    rc::Rc, array, sync::Arc
};

use glam::{Vec2};
use specs::{WorldExt, World};
use winit::{event::{VirtualKeyCode, ElementState, self}, dpi::{PhysicalPosition, LogicalPosition}, window::{Window, CursorGrabMode}};

use crate::{GameState, game::Time};

#[derive(Debug, Copy, Clone)]
pub struct Mouse {
    pub position: Vec2,
    pub motion: Vec2,
    pub delta: Vec2,
    pub buttons: [bool; 3],
    last_move: i64,
}
impl Mouse {

    pub fn set_cursor_motion(&mut self, pos: (f64, f64)) {
        let vec_pos = Vec2::new(pos.0 as f32, pos.1 as f32);
        self.motion = vec_pos;
        self.last_move = chrono::offset::Local::now().timestamp_millis();
    }

    pub fn get_motion(&self) -> Vec2 {
        let now = chrono::offset::Local::now().timestamp_millis();
        if now - self.last_move > 50 {
            return Vec2::ZERO;
        } else {
            return self.motion;
        }
    }
}
impl Default for Mouse {
    fn default() -> Self {
        Mouse { position: Vec2::new(0.0, 0.0), motion: Vec2::new(0.0, 0.0), delta: Vec2::new(0.0, 0.0), buttons: [false, false, false], last_move: 0, }
    }
}



#[derive(Debug, Copy, Clone)]
pub struct KeyState {
    pub key: VirtualKeyCode,
    pub pressed: bool,
    pub repeat: bool,
    pub down_frame: i64,
    pub up_frame: i64,
}

impl KeyState {
    pub fn new(key: VirtualKeyCode, pressed: bool) -> Self {
        KeyState {key: key, repeat: false, pressed: pressed, down_frame: 0, up_frame: 0}
    }
    pub fn key_down(&mut self, frame: i64) {
        self.pressed = true;
        self.down_frame = frame;
        self.up_frame = 0;
    }
    pub fn key_up(&mut self, frame: i64) {
        self.pressed = false;
        self.down_frame = 0;
        self.up_frame = frame;
    }
}

#[derive(Default, Debug, Clone)]
pub struct Input {
    pub keys: HashMap<VirtualKeyCode, KeyState>,
    //key_timing: HashMap<VirtualKeyCode, i64>,
    pub mouse: Mouse,
    pub cursor_locked: bool,
    cursor_lock_pos: Vec2,
    pub cursor_visible: bool,
    pub window_size: Vec2,

}



impl Input {
    pub fn new() -> Input {
        let keys = HashMap::new();
        //let key_timing: HashMap::new();
        let mouse = Mouse { ..Default::default()};
        //let events = Vec::new();
        Input { keys, mouse, cursor_locked: false, cursor_lock_pos: Vec2::new(0.0, 0.0), window_size: Vec2::new(0.0, 0.0), cursor_visible: true}
    }

    pub fn copy(&mut self) -> Input {
        let keys = self.keys.clone();
        let mouse = self.mouse;
        let cursor_locked = self.cursor_locked;
        let cursor_lock_pos = self.cursor_lock_pos;
        let cursor_visible = self.cursor_visible;
        let window_size = self.window_size;
        
        Input { keys, mouse, cursor_locked, cursor_lock_pos, cursor_visible, window_size }
    }


    pub fn update_mouse_motion(&mut self, pos: (f64, f64), window: &Window) {
        self.mouse.set_cursor_motion(pos);
        if self.cursor_locked {
            window.set_cursor_position(PhysicalPosition::new(self.cursor_lock_pos.x, self.cursor_lock_pos.y)).unwrap();
        } 
    }
    /* 
    * Window Update function, called every frame.
    * requires a mutable winit::Window reference.
    */
    pub fn window_update(&mut self, window: &Window) {
        window.set_cursor_visible(self.cursor_visible);
    }

    /*
    * Resize called whenever winit detects a window resize.
    * resets cursor lock position to center of screen
    */
    pub fn resize(&mut self, size:  winit::dpi::PhysicalSize<u32>) {
        self.cursor_lock_pos = Vec2::new(size.width as f32 / 2.0, size.height as f32 / 2.0);
    }

    
    /*
    * Sets the position the cursor goes to when lock is on
    * normally center of screen.
    */
    pub fn set_cursor_pos(&mut self, pos: (f64, f64)) {
        let vec_pos = Vec2::new(pos.0 as f32, pos.1 as f32);
        self.mouse.position = vec_pos;
    }

    
    /*
    * Called on any key up event. 
    * Updates internal key set list
    * Not for use in detecting key events (see get_key & get_key_down)
    */
    pub fn on_key_down(&mut self, key: VirtualKeyCode, world: &World) {
        let frame = world.read_resource::<Time>().frame;
        self.update_key_set(key);
        let key_state = self.keys.get_mut(&key).unwrap();
        key_state.key_down(frame);
        key_state.down_frame = frame;

        let key_state_clone = key_state.clone();
        drop(key_state);
        drop(frame);
        self.keys.insert(key, key_state_clone);
    }
    /*
    * Called on any key up event. 
    * Updates internal key set list
    * Not for use in detecting key events (see get_key & get_key_down)
    */
    pub fn on_key_up(&mut self, key: VirtualKeyCode, world: &World) {
        let frame = world.read_resource::<Time>().frame;
        println!("{:?} : UP", key);
        self.update_key_set(key);
        let key_state = self.keys.get_mut(&key).unwrap();
        key_state.key_up(frame);
        key_state.up_frame = frame;

        let key_state_clone = key_state.clone();
        drop(key_state);
        drop(frame);
        self.keys.insert(key, key_state_clone);
    }

    /* get_key
    * Check whether specific key is currently pressed
    * returns boolean value, true means key is down
    */
    pub fn get_key(&mut self, key: VirtualKeyCode) -> bool {
        self.update_key_set(key);
        return self.keys[&key].pressed;
    }
    /* get_key_state
    * Check whether specific key is currently pressed
    * returns KeyState value.
    */
    pub fn get_key_state(&mut self, key: VirtualKeyCode) -> &KeyState {
        self.update_key_set(key);
        return self.keys.get(&key).unwrap();
    }
    /*
    * Check if key was pressed this loop cycle
    * returns boolean value, true means key was pressed in this cycle.
    * cannot be triggered again until key is released
    */
    pub fn get_key_down(&mut self, key: VirtualKeyCode, frame: i64) -> bool {
        self.update_key_set(key);
        let key_state = self.keys.get_mut(&key).unwrap();
        //println!("{:?}, {:?}", key_state.down_frame, frame);
        if frame == key_state.down_frame + 1 {
            return true;
        } else {
            return false;
        }
        
    }

    fn update_key_set(&mut self, key: VirtualKeyCode) -> bool { // true: already in there, false: wasnt in there, putting it in
        
        if self.keys.len() >= 149{ return true } // map is maxed, can't be in there
        else if self.keys.contains_key(&key) { return true } // already in there
        else {
            self.keys.insert(key, KeyState::new(key, false));
            return false;
        }
    }
    
}