use glam::{Mat4, Vec3, vec3};
use glium::winit::keyboard::{KeyCode, PhysicalKey};
use std::collections::HashSet;

pub struct Camera {
    fov: f32,
    dist: f32,
    position: Vec3,
    target: Vec3,
    up: Vec3,
    speed: f32,

    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub fn new(fov: f32, dist: f32) -> Self {
        Self {
            fov,
            dist,
            position: vec3(0.0, 0.0, 1.0),
            target: vec3(0.0, 0.0, 0.0),
            up: Vec3::Y,
            speed: 10.0,

            yaw: 0.0,
            pitch: 0.0,
        }
    }
    pub fn get_view_matrix(&self) -> Mat4 {
        // Create a view matrix using the camera's position, target, and up vector
        Mat4::look_at_rh(self.position, self.target, self.up)
    }
    fn translate(&mut self, delta: Vec3) {
        self.position += delta;
        self.target += delta;
    }
    fn forward(&mut self) -> Vec3 {
        (self.target - self.position).normalize()
    }
    fn right(&mut self) -> Vec3 {
        self.forward().cross(self.up).normalize()
    }
    fn forward_from(&self) -> Vec3 {
        let cy = self.yaw.cos();
        let sy = self.yaw.sin();
        let cp = self.pitch.cos();
        let sp = self.pitch.sin();

        vec3(sy * cp, sp, cy * cp).normalize()
    }

    pub fn add_to_yaw_pitch(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch += delta_pitch;
    }

    pub fn apply_yaw_pitch(&mut self) {
        let limit = std::f32::consts::FRAC_PI_2 - 0.001;
        self.pitch = self.pitch.clamp(-limit, limit);
        let f = self.forward_from();
        self.target = self.position + f;
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    pub fn get_dist(&self) -> f32 {
        self.dist
    }

    pub fn update_camera(&mut self, keys_pressed: &HashSet<PhysicalKey>, delta_time: f32) {
        let forward = self.forward();
        let right = self.right();
        let up = self.up.normalize_or_zero();

        let mut direction = Vec3::ZERO;
        if keys_pressed.contains(&PhysicalKey::Code(KeyCode::KeyW)) {
            direction += forward;
        }
        if keys_pressed.contains(&PhysicalKey::Code(KeyCode::KeyS)) {
            direction -= forward;
        }
        if keys_pressed.contains(&PhysicalKey::Code(KeyCode::KeyA)) {
            direction -= right;
        }
        if keys_pressed.contains(&PhysicalKey::Code(KeyCode::KeyD)) {
            direction += right;
        }
        if keys_pressed.contains(&PhysicalKey::Code(KeyCode::Space)) {
            direction += up;
        }

        if keys_pressed.contains(&PhysicalKey::Code(KeyCode::ControlLeft)) {
            direction -= up;
        }

        if direction.length_squared() > 0.0 {
            let delta = direction.normalize() * self.speed * delta_time;
            self.translate(delta);
        }
    }
}
