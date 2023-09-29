use cgmath::Vector3;
use rand::Rng;
use crate::voxgl::world::color::Color;

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

        let range = 0.7;
        let color = Color::new(
            (0.7 - range) + rand::thread_rng().gen_range(0.3f32..range),
            (0.2 - range) + rand::thread_rng().gen_range(0.1f32..range),
            (0.6 - range) + rand::thread_rng().gen_range(0.6f32..range),
            1.0,
        );

        Self {
            vertices, face, color,
        }
    }
}