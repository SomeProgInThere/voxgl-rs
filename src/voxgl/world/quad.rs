use cgmath::Vector3;

use super::color::Color;

#[derive(Debug)]
pub enum Face {
    Right, Left, Top, Bottom, Front, Back
}

impl Face {
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

impl Quad {
    pub fn from_face(face: Face, pos: Vector3<f32>) -> Self {
        let vertices = match face {
            Face::Right => [
                Vector3::new(pos.x, pos.y + 1.0, pos.z      ),
                Vector3::new(pos.x, pos.y + 1.0, pos.z + 1.0),
                Vector3::new(pos.x, pos.y,       pos.z + 1.0),
                Vector3::new(pos.x, pos.y,       pos.z      ),
            ],
            Face::Left => [
                Vector3::new(pos.x, pos.y,       pos.z      ),
                Vector3::new(pos.x, pos.y,       pos.z + 1.0),
                Vector3::new(pos.x, pos.y + 1.0, pos.z + 1.0),
                Vector3::new(pos.x, pos.y + 1.0, pos.z      ),
            ],
            Face::Top => [
                Vector3::new(pos.x,       pos.y, pos.z + 1.0),
                Vector3::new(pos.x + 1.0, pos.y, pos.z + 1.0),
                Vector3::new(pos.x + 1.0, pos.y, pos.z      ),
                Vector3::new(pos.x,       pos.y, pos.z      ),
            ],
            Face::Bottom => [
                Vector3::new(pos.x,       pos.y, pos.z      ),
                Vector3::new(pos.x + 1.0, pos.y, pos.z      ),
                Vector3::new(pos.x + 1.0, pos.y, pos.z + 1.0),
                Vector3::new(pos.x,       pos.y, pos.z + 1.0),
            ],
            Face::Front => [
                Vector3::new(pos.x + 1.0, pos.y,       pos.z),
                Vector3::new(pos.x + 1.0, pos.y + 1.0, pos.z),
                Vector3::new(pos.x,       pos.y + 1.0, pos.z),
                Vector3::new(pos.x,       pos.y,       pos.z),
            ],
            Face::Back => [
                Vector3::new(pos.x,       pos.y,       pos.z),
                Vector3::new(pos.x,       pos.y + 1.0, pos.z),
                Vector3::new(pos.x + 1.0, pos.y + 1.0, pos.z),
                Vector3::new(pos.x + 1.0, pos.y,       pos.z),
            ],
        };

        let color = Color::new(0.21, 0.80, 0.01, 1.0);

        Self {
            vertices, face, color,
        }
    }
}