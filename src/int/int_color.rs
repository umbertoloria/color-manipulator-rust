use color_manipulator_rust::RGB;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Color {
    pub fn to_rgb(&self) -> RGB {
        RGB {
            r: self.r as f32 / 255.0,
            g: self.g as f32 / 255.0,
            b: self.b as f32 / 255.0,
        }
    }
}

pub fn rgb_to_color(rgb: RGB) -> Color {
    Color {
        r: (rgb.r * 255.0) as u8,
        g: (rgb.g * 255.0) as u8,
        b: (rgb.b * 255.0) as u8,
    }
}
