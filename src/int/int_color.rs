use crate::int::int_layer::Color;
use color_manipulator_rust::RGB;

pub fn rgb_to_color(rgb: RGB) -> Color {
    Color {
        r: (rgb.r * 255.0) as u8,
        g: (rgb.g * 255.0) as u8,
        b: (rgb.b * 255.0) as u8,
    }
}
