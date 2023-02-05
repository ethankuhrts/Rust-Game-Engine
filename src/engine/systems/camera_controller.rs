use glam::{Vec3, Quat, Vec2, EulerRot};
use specs::{Write, System, WriteStorage, Join, Read};
use winit::event::VirtualKeyCode;
use std::{f32::consts::PI};

use crate::{
    components::{ Transform, Camera, ModelRenderer, transform::{EulerRotation, Axis} }, Input, graphics::Model, game::Time
};

pub struct CameraController;

const MIN_PITCH: f32 = (-PI / 2.0) + 0.05;
const MAX_PITCH: f32 = (PI / 2.0) - 0.05;

impl<'a> System<'a> for CameraController {
    type SystemData = (
        Write<'a, Input>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Camera>,
        Read<'a, Time>,
    );
    

    fn run (&mut self, data: Self::SystemData) {
        let (mut input, mut transform, mut camera, time) = data;
        let speed: f32 = 5.0;
        
        
        
        for (transform, camera) in (&mut transform, &mut camera).join() {

            if input.get_key(VirtualKeyCode::W) {
                transform.position += transform.forward() * speed* time.delta;
            }
            if input.get_key(VirtualKeyCode::A) {
                transform.position -= transform.right() * speed* time.delta;
            }
            if input.get_key(VirtualKeyCode::S) {
                transform.position -= transform.forward() * speed* time.delta;
            }
            if input.get_key(VirtualKeyCode::D) {
                transform.position += transform.right() * speed* time.delta;
            }
            if input.get_key(VirtualKeyCode::Space) {
                transform.position += Vec3::new(0.0, 1.0, 0.0) * speed* time.delta;
            } 
            if input.get_key(VirtualKeyCode::LShift) {
                transform.position -= Vec3::new(0.0, 1.0, 0.0) * speed* time.delta;
            }
            if input.get_key_down(VirtualKeyCode::Escape, time.frame) {
                input.cursor_locked = !input.cursor_locked;
                input.cursor_visible = !input.cursor_locked;
            } 
            
            let motion = input.mouse.get_motion() * Vec2::new(time.delta, time.delta);
            
            let (mut yaw, mut pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            yaw -= motion.x;
            pitch = (pitch + motion.y).clamp(MIN_PITCH, MAX_PITCH);
            let roll = 0.0;

            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

        }
        
    }
}