use glam::{ Vec3, Quat, EulerRot };
use winit::event::VirtualKeyCode;
use std::{f32::consts::PI, ops::Add};


use crate::{
    ecs::{
        Component, VecStorage, System, ReadStorage, Read, Write, Join, WriteStorage
    }, input::Input
};



pub trait Rotation {
}
impl Rotation for Quat {
    
}

pub trait Position {

}
impl Position for Vec3 {
}

#[derive(Default, Debug, Clone)]
pub struct EulerRotation{pub x: f32, pub y: f32, pub z: f32}

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

pub trait Conversions<T> {
    fn to_rad(self) -> T;
}

impl Conversions<f32> for f32 {
    fn to_rad(self) -> f32 {
        return self * (PI / 180.0);
    }
}

pub enum Axis {
    X, Y, Z
}


impl Transform {
    pub fn new(x: f32, y: f32, z: f32) -> Transform {
        
        Transform { position: Vec3{x: x, y:y, z:z}, rotation: Quat::IDENTITY, scale: Vec3 { x:0.0, y: 0.0, z: 0.0 } }
    }

    pub fn yaw_pitch_roll(&self) -> (f32, f32, f32) {
        return self.rotation.to_euler(glam::EulerRot::XYZ);
    }
    pub fn forward(&self) -> Vec3 {
        return Quat::mul_vec3(self.rotation, Vec3::new(0.0, 0.0, 1.0)).normalize();
    }
    pub fn right(&self) -> Vec3 {
        //return Quat::mul_vec3(self.rotation, Vec3::new(-1.0, 0.0, 0.0));
        return self.forward().cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
    }
    pub fn up(&self) -> Vec3 {
        return Quat::mul_vec3(self.rotation, Vec3::new(0.0, 1.0, 0.0));
    }

    pub fn rotate(&mut self, rotation: Vec3) {
        let eul_rot = Quat::from_euler(glam::EulerRot::XYZ, rotation.x, rotation.y, rotation.z).normalize();
        self.rotation = Quat::mul_quat(self.rotation, eul_rot);
    }
    pub fn rotate_axis(&mut self, axis: Vec3, rotation: f32 ) {
        let axis_rot = Quat::from_axis_angle(axis, rotation);
        self.rotation = Quat::mul_quat(self.rotation, axis_rot);
    }
    pub fn set_axis(&mut self, axis: Axis, angle: f32 ) {
        let euler = self.rotation.to_euler(EulerRot::XYZ);
        let axis_rot = match axis {
            Axis::X => { Quat::from_rotation_x(angle.to_radians()) },
            Axis::Y => { Quat::from_rotation_y(angle.to_radians()) },
            Axis::Z => { Quat::from_rotation_z(angle.to_radians()) }
        };
        
        self.rotation = Quat::add(self.rotation, axis_rot);
    }
    
    pub fn get_rotation(&mut self) -> Vec3 {
        let euler = self.rotation.mul_vec3(Vec3::new(0.0, 0.0, 0.0));
        return Vec3::new(euler.x, euler.y, euler.z);
    }

    pub fn rotation_to_rad(&self, rotation: Vec3) -> Vec3 {
        return Vec3::new(rotation.x * (PI /180.0), rotation.y * (PI / 180.0), rotation.z * (PI / 180.0));
    }
    
    pub fn set_rotation(&mut self, rotation: Vec3) {
        let eul_rad = self.rotation_to_rad(rotation);
        let eul_rot = Quat::from_euler(glam::EulerRot::XYZ, eul_rad.x, eul_rad.y, eul_rad.z);
        self.rotation = eul_rot;
    }
    
}
