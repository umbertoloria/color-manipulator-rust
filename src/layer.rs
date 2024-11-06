use crate::folding::get_path_out;
use crate::RGB;
use color_manipulator_rust::POS;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};

pub trait AbsLayer {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get_color(&self, x: usize, y: usize) -> RGB;
}
impl dyn AbsLayer {
    pub fn save(&self, output_file_path: &str) {
        println!("File \"{}\": saving in", output_file_path);

        let width = self.width();
        let height = self.height();

        let mut output_img = ImageBuffer::new(width as u32, height as u32);

        for y in 0..height {
            for x in 0..width {
                let out_color: RGB = self.get_color(x, y);

                // Write destination color
                output_img.put_pixel(x as u32, y as u32, Rgb([
                    (out_color.r * 255.0) as u8,
                    (out_color.g * 255.0) as u8,
                    (out_color.b * 255.0) as u8,
                ]));
            }
        }

        output_img.save(get_path_out(output_file_path)).unwrap();
    }
}

// File Image Layer
pub struct FileImageLayer {
    image: DynamicImage,
}
impl FileImageLayer {
    fn new(path: &str) -> Self {
        let image = image::open(&path).unwrap();
        Self { image }
    }
    /* pub fn exec<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut DynamicImage) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    {
        let result = func(&mut self.image);
        if let Some(x) = result {
            self.image = DynamicImage::ImageRgba8(x);
        }
    } */
    /* pub fn get_image(&self) -> &DynamicImage {
        &self.image
    } */
}
impl AbsLayer for FileImageLayer {
    fn width(&self) -> usize {
        self.image.width() as usize
    }
    fn height(&self) -> usize {
        self.image.height() as usize
    }
    fn get_color(&self, x: usize, y: usize) -> RGB {
        let color_src = self.image.get_pixel(x as u32, y as u32).0;
        let r_src = *(color_src.get(0).unwrap());
        let g_src = *(color_src.get(1).unwrap());
        let b_src = *(color_src.get(2).unwrap());
        RGB {
            r: r_src as f32 / 255.0,
            g: g_src as f32 / 255.0,
            b: b_src as f32 / 255.0,
        }
    }
}
pub fn load_image_layer(path: &str) -> FileImageLayer {
    FileImageLayer::new(path)
}

// Diff Layer
pub struct DiffLayer {
    a: Box<dyn AbsLayer>,
    b: Box<dyn AbsLayer>,
}
impl DiffLayer {
    fn new(a: Box<dyn AbsLayer>, b: Box<dyn AbsLayer>) -> Self {
        Self { a, b }
    }
    pub fn save(&self, path: &str) {
        let width = self.width();
        let height = self.height();
        let mut output_img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width as u32, height as u32);
        for y in 0..height {
            for x in 0..width {
                let out_color = self.get_color(x, y);

                let int_r = (out_color.r * 255.0) as u32;
                let int_g = (out_color.g * 255.0) as u32;
                let int_b = (out_color.b * 255.0) as u32;

                let int_r8 = int_r as u8;
                let int_g8 = int_g as u8;
                let int_b8 = int_b as u8;

                output_img.put_pixel(
                    x as u32,
                    y as u32,
                    Rgb([
                        int_r8,
                        int_g8,
                        int_b8,
                    ]),
                )
            }
        }
        output_img.save(path).unwrap();
    }
}
impl AbsLayer for DiffLayer {
    fn width(&self) -> usize {
        self.a.width()
    }
    fn height(&self) -> usize {
        self.a.height()
    }
    fn get_color(&self, x: usize, y: usize) -> RGB {
        let a_color = self.a.get_color(x, y);
        let b_color = self.b.get_color(x, y);

        let r_diff = a_color.r - b_color.r;
        let g_diff = a_color.g - b_color.g;
        let b_diff = a_color.b - b_color.b;

        RGB {
            r: r_diff.abs(),
            g: g_diff.abs(),
            b: b_diff.abs(),
        }
    }
}
pub fn create_diff_layer(p0: Box<dyn AbsLayer>, p1: Box<dyn AbsLayer>) -> DiffLayer {
    DiffLayer::new(p0, p1)
}

// Filtered Layer
pub struct FilteredLayer {
    layer: Box<dyn AbsLayer>,
    calculate_color_func: fn(c: RGB, p: POS) -> RGB,
}
impl FilteredLayer {
    fn new(layer: Box<dyn AbsLayer>, calculate_color_func: fn(c: RGB, p: POS) -> RGB) -> Self {
        Self { layer, calculate_color_func }
    }
}
impl AbsLayer for FilteredLayer {
    fn width(&self) -> usize {
        self.layer.width()
    }
    fn height(&self) -> usize {
        self.layer.height()
    }
    fn get_color(&self, x: usize, y: usize) -> RGB {
        let width = self.layer.width();
        let height = self.layer.height();

        let in_color = self.layer.get_color(x, y);
        let in_position = POS {
            x: x as f32 / width as f32,
            y: y as f32 / height as f32,
        };

        (self.calculate_color_func)(in_color, in_position)
    }
}
pub fn create_filtered_layer(layer: Box<dyn AbsLayer>, calculate_color_func: fn(c: RGB, p: POS) -> RGB) -> Box<dyn AbsLayer> {
    Box::new(FilteredLayer::new(layer, calculate_color_func))
}
