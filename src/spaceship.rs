use nalgebra_glm::Vec3;

pub struct Spaceship {
    pub position: Vec3,
    pub direction: Vec3,
    pub speed: f32,
    pub rotation_speed: f32,
}

impl Spaceship {
    pub fn new(position: Vec3, direction: Vec3) -> Self {
        Self {
            position,
            direction: direction.normalize(),
            speed: 0.5,
            rotation_speed: 0.1,
        }
    }

    pub fn move_forward(&mut self) {
        self.position += self.direction * self.speed;
    }

    pub fn move_backward(&mut self) {
        self.position -= self.direction * self.speed;
    }

    pub fn rotate_left(&mut self) {
        self.direction = nalgebra_glm::rotate_vec3(
            &self.direction,
            self.rotation_speed,
            &Vec3::new(0.0, 1.0, 0.0),
        );
    }

    pub fn rotate_right(&mut self) {
        self.direction = nalgebra_glm::rotate_vec3(
            &self.direction,
            -self.rotation_speed,
            &Vec3::new(0.0, 1.0, 0.0),
        );
    }
}
