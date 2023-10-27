use cgmath::Vector3;

use super::color::Color;

#[derive(Debug)]
pub enum Face {
    Right, Left, Top, Bottom, Front, Back
}

impl Face {
    #[allow(dead_code)]
    pub fn get_normal(&self) -> Vector3<f32> {
        match self {
            Self::Right  =>  Vector3::<f32>::unit_x(),
            Self::Left   => -Vector3::<f32>::unit_x(),
            Self::Top    =>  Vector3::<f32>::unit_y(),
            Self::Bottom => -Vector3::<f32>::unit_y(),
            Self::Front  =>  Vector3::<f32>::unit_z(),
            Self::Back   => -Vector3::<f32>::unit_z(),
        }
    }
}

#[derive(Debug)]
pub struct Quad {
    pub vertices: [Vector3<f32>; 4],
    pub face: Face,
    pub color: Color,
}

const HALF_SIZE: f32 = 0.5;

impl Quad {
    pub fn from_face(face: Face, pos: Vector3<f32>) -> Self {
        let vertices = match face {
            Face::Right => [
                Vector3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
            ],
            Face::Left => [
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
            ],
            Face::Top => [
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
                Vector3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
                Vector3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
            ],
            Face::Bottom => [
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
            ],
            Face::Front => [
                Vector3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
            ],
            Face::Back => [
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
            ],
        };

        let color = Color::new(0.12, 0.4, 0.2, 1.0);

        Self {
            vertices, face, color,
        }
    }
}