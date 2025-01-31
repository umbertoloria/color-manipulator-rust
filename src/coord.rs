use color_manipulator_rust::RGB;
use image::{ImageBuffer, Rgb};

pub struct ImgBuffer {
    width: u32,
    height: u32,
    filepath: String,
    image_buffer: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
}
impl ImgBuffer {
    pub fn new(width: u32, height: u32, filepath: String) -> Self {
        Self { width, height, filepath, image_buffer: None }
    }
    pub fn paint(&mut self, fn_get_color: impl Fn(usize, usize) -> RGB) {
        let mut image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let out_color: RGB = fn_get_color(x as usize, y as usize);
                image_buffer.put_pixel(x, y, Rgb([
                    (out_color.r * 255.0) as u8,
                    (out_color.g * 255.0) as u8,
                    (out_color.b * 255.0) as u8,
                ]));
            }
        }
        self.image_buffer = Some(image_buffer);
    }
    pub fn save(&self) {
        if self.image_buffer.is_some() {
            // TODO: Is "clone" heavy?
            self.image_buffer.clone().unwrap().save(self.filepath.clone()).unwrap();
        }
    }
}
