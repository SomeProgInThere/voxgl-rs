
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Color(pub [u8; 4]);

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [
            self.0[0] as f32 / 255.0,
            self.0[1] as f32 / 255.0,
            self.0[2] as f32 / 255.0,
            self.0[3] as f32 / 255.0,
        ]
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        self.0
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color([
            (r.min(1.0).max(0.0) * 255.0) as u8,
            (g.min(1.0).max(0.0) * 255.0) as u8,
            (b.min(1.0).max(0.0) * 255.0) as u8,
            (a.min(1.0).max(0.0) * 255.0) as u8
        ])
    }
}
