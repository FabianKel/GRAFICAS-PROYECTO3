use nalgebra_glm::{Vec3, rotate_vec3};

use crate::spaceship::Spaceship;

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub has_changed: bool,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera {
            eye,
            center,
            up,
            has_changed: true,
        }
    }

    pub fn follow_spaceship(&mut self, spaceship: &Spaceship) {
        // Ajuste más preciso de la posición de la cámara en función de la nave
        self.eye = spaceship.position - spaceship.rotation.normalize() * 20.0 + Vec3::new(0.0, 10.0, 0.0);
        self.center = spaceship.position;
        self.has_changed = true;
    }

    pub fn move_center(&mut self, direction: Vec3) {
        let radius_vector = self.center - self.eye;
        let radius = radius_vector.magnitude();

        if radius == 0.0 {
            return; // Evitar divisiones por cero
        }

        let angle_x = direction.x * 0.05;
        let angle_y = direction.y * 0.05;

        let rotated = rotate_vec3(&radius_vector, angle_x, &Vec3::new(0.0, 1.0, 0.0));
        let right = rotated.cross(&self.up).normalize();

        let final_rotated = rotate_vec3(&rotated, angle_y, &right);

        self.center = self.eye + final_rotated.normalize() * radius;
        self.has_changed = true;
    }
}
