use crate::math::{Mat4, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3, up: Vec3) -> Self {
        Self {
            position,
            target,
            up,
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::look_at(self.position, self.target, self.up)
    }
}
