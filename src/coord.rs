use color_manipulator_rust::RGB;
use image::{ImageBuffer, Rgb};

// Coord
pub struct Coord {
    pub x: usize,
    pub y: usize,
}
pub struct CoordSquare {
    pub top_left: Coord,
    pub bottom_right: Coord,
}
impl CoordSquare {
    pub fn get_width(&self) -> usize {
        self.bottom_right.x - self.top_left.x
    }
    pub fn get_height(&self) -> usize {
        self.bottom_right.y - self.top_left.y
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
            // TODO: Is "clone" heavy?
            self.image_buffer
                .clone()
                .unwrap()
                .save(self.filepath.clone())
                .unwrap();
        }
    }
}

// Separation of CoordSquare
pub fn get_separated_coord_squares(
    threads_num: usize,
    width: usize,
    height: usize,
) -> Vec<CoordSquare> {
    let slice_for_thread = width / threads_num;
    let mut threads_coords: Vec<CoordSquare> = Vec::new();
    let mut next_x_ready = 0;
    for _ in 0..(threads_num - 1) {
        let prev_x = next_x_ready;
        next_x_ready = (prev_x + slice_for_thread) + 1;
        threads_coords.push(CoordSquare {
            top_left: Coord { x: prev_x, y: 0 },
            bottom_right: Coord {
                x: next_x_ready - 1,
                y: height,
            },
        });
    }
    threads_coords.push(CoordSquare {
        top_left: Coord {
            x: next_x_ready,
            y: 0,
        },
        bottom_right: Coord {
            x: width,
            y: height,
        },
    });
    // println!("threads_coords: {:?}", threads_coords); // For debug only.
    threads_coords
}
