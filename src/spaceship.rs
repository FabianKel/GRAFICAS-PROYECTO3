use nalgebra_glm::Vec3;
use crate::fragment::Fragment;
use crate::Uniforms;
use crate::color::Color;
use crate::vertex::Vertex;

pub struct Spaceship {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: f32,
    pub vertices: Vec<Vertex>,
    pub shader: fn(&Fragment, &Uniforms) -> Color,
}

impl Spaceship {
    pub fn new(
        position: Vec3,
        rotation: Vec3,
        scale: f32,
        vertices: Vec<Vertex>,
        shader: fn(&Fragment, &Uniforms) -> Color,
    ) -> Self {
        Spaceship {
            position,
            rotation,
            scale,
            vertices,
            shader,
        }
    }

    pub fn move_forward(&mut self, speed: f32) {
        let direction = Vec3::new(self.rotation.y.sin(), 0.0, self.rotation.y.cos()).normalize();
        self.position += direction * speed;
    }

    pub fn move_backward(&mut self, speed: f32) {
        let direction = Vec3::new(self.rotation.y.sin(), 0.0, self.rotation.y.cos()).normalize();
        self.position -= direction * speed;
    }

    pub fn rotate_left(&mut self, angle: f32) {
        self.rotation.y -= angle;
    }

    pub fn rotate_right(&mut self, angle: f32) {
        self.rotation.y += angle;
    }
}
