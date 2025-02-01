use crate::int_layer::Color;
use image::{ImageBuffer, Rgb};

// Img Buffer
pub struct IntImgBuffer {
    width: usize,
    height: usize,
    filepath: String,
    image_buffer: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
}
impl IntImgBuffer {
    pub fn new(width: usize, height: usize, filepath: String) -> Self {
        Self {
            width,
            height,
            filepath,
            image_buffer: None,
        }
    }
    pub fn paint(&mut self, fn_get_color: impl Fn(usize, usize) -> Color) {
        let mut image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::new(self.width as u32, self.height as u32);
        for y in 0..self.height {
            for x in 0..self.width {
                let out_color = fn_get_color(x, y);
                image_buffer.put_pixel(
                    x as u32,
                    y as u32,
                    Rgb([out_color.r, out_color.g, out_color.b]),
                );
            }
        }
        self.image_buffer = Some(image_buffer);
    }
    pub fn save(&self) {
        if self.image_buffer.is_some() {
            // TODO: Avoid clone
            self.image_buffer
                .clone()
                .unwrap()
                .save(self.filepath.clone())
                .unwrap();
        }
    }
}
