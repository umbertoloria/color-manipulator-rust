use color_manipulator_rust::RGB;
use image::{ImageBuffer, Rgb};

// Coord
#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}
#[derive(Clone, Copy, Debug)]
pub struct CoordSquare {
    // Both incl.
    pub top_left: Coord,
    pub bottom_right: Coord,
}
impl CoordSquare {
    pub fn get_width(&self) -> usize {
        self.bottom_right.x - self.top_left.x + 1
    }
    pub fn get_height(&self) -> usize {
        self.bottom_right.y - self.top_left.y + 1
    }
}

// Img Buffer
pub struct ImgBuffer {
    width: u32,
    height: u32,
    filepath: String,
    image_buffer: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
}
impl ImgBuffer {
    pub fn new(width: u32, height: u32, filepath: String) -> Self {
        Self {
            width,
            height,
            filepath,
            image_buffer: None,
        }
    }
    pub fn paint(&mut self, fn_get_color: impl Fn(usize, usize) -> RGB) {
        let mut image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let out_color: RGB = fn_get_color(x as usize, y as usize);
                image_buffer.put_pixel(
                    x,
                    y,
                    Rgb([
                        (out_color.r * 255.0) as u8,
                        (out_color.g * 255.0) as u8,
                        (out_color.b * 255.0) as u8,
                    ]),
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

// Separation of CoordSquare
pub fn get_coord_chunks(chunks_count: usize, width: usize, height: usize) -> Vec<CoordSquare> {
    let mut result: Vec<CoordSquare> = Vec::new();

    let chunk_width_approx = width / chunks_count; // Floor.

    let mut next_x_ready = 0;
    for _ in 0..(chunks_count - 1) {
        let prev_x = next_x_ready;
        next_x_ready += chunk_width_approx;
        result.push(CoordSquare {
            top_left: Coord { x: prev_x, y: 0 },
            bottom_right: Coord {
                x: next_x_ready - 1,
                y: height - 1,
            },
        });
    }
    result.push(CoordSquare {
        top_left: Coord {
            x: next_x_ready,
            y: 0,
        },
        bottom_right: Coord {
            x: width - 1,
            y: height - 1,
        },
    });

    /*
    // For debug only.
    for result_item in &result {
        println!(
            "{:?}, width={}, height={}",
            result_item,
            result_item.get_width(),
            result_item.get_height()
        );
    }
    */

    result
}
