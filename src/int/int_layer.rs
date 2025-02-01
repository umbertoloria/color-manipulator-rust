use crate::coord::Coord;
use crate::folding::get_path_out;
use crate::int::int_color::Color;
use crate::int::int_img_buffer::IntImgBuffer;
use image::{DynamicImage, GenericImageView};
use std::process::exit;

// ABS LAYER
pub trait IntAbsLayer {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get_color(&self, x: usize, y: usize) -> Color;
}
impl dyn IntAbsLayer {
    pub fn save(&self, output_file_path: &str) {
        println!("Painting file \"{}\"", output_file_path);

        let width = self.width();
        let height = self.height();

        let mut img_buffer_full = IntImgBuffer::new(width, height, get_path_out(output_file_path));
        img_buffer_full.paint(|x, y| self.get_color(x, y));
        img_buffer_full.save();
    }
}

// FILE IMAGE LAYER
pub struct IntFileImageLayer {
    image: DynamicImage,
}
impl IntFileImageLayer {
    fn new(path: &str) -> Self {
        Self {
            image: image::open(&path).unwrap(),
        }
    }
}
impl IntAbsLayer for IntFileImageLayer {
    fn width(&self) -> usize {
        self.image.width() as usize
    }
    fn height(&self) -> usize {
        self.image.height() as usize
    }
    fn get_color(&self, x: usize, y: usize) -> Color {
        let color_src = self.image.get_pixel(x as u32, y as u32).0;
        let r_src = *(color_src.get(0).unwrap());
        let g_src = *(color_src.get(1).unwrap());
        let b_src = *(color_src.get(2).unwrap());
        Color {
            r: r_src,
            g: g_src,
            b: b_src,
        }
    }
}
pub fn load_int_file_image_layer(path: &str) -> IntFileImageLayer {
    IntFileImageLayer::new(path)
}
pub fn load_int_file_image_layer_box(path: &str) -> Box<IntFileImageLayer> {
    Box::new(IntFileImageLayer::new(path))
}

// FILTERED LAYER
type IntFilteredLayerFn = fn(c: Color, p: Coord) -> Color;
pub struct IntFilteredLayer {
    layer: Box<dyn IntAbsLayer>,
    calculate_color_func: IntFilteredLayerFn,
}
impl IntAbsLayer for IntFilteredLayer {
    fn width(&self) -> usize {
        self.layer.width()
    }
    fn height(&self) -> usize {
        self.layer.height()
    }
    fn get_color(&self, x: usize, y: usize) -> Color {
        let in_color = self.layer.get_color(x, y);
        let in_position = Coord { x, y };
        (self.calculate_color_func)(in_color, in_position)
    }
}
pub fn create_int_filtered_layer(
    layer: Box<dyn IntAbsLayer>,
    calculate_color_func: IntFilteredLayerFn,
) -> IntFilteredLayer {
    IntFilteredLayer {
        layer,
        calculate_color_func,
    }
}

// MERGE LAYER
pub struct IntMergeLayer {
    pub filepath_list: Vec<String>,
    pub file_image_layers: Vec<IntFileImageLayer>,
    pub width: usize,
    pub height: usize,
}
impl IntMergeLayer {
    pub fn new(filepath_list: Vec<String>) -> Self {
        if filepath_list.is_empty() {
            println!("Filepath List empty");
            exit(0x0100);
        }
        let mut width = 0;
        let mut height = 0;
        let mut file_image_layers = Vec::new();
        for filepath in &filepath_list {
            let file_image_layer = load_int_file_image_layer(filepath);

            width += file_image_layer.width();
            if height == 0 {
                height = file_image_layer.height();
            } else {
                if height != file_image_layer.height() {
                    println!("Filepath List go to Images that have different heights");
                    exit(0x0100);
                }
            }

            file_image_layers.push(file_image_layer);
        }
        Self {
            filepath_list,
            file_image_layers,
            width,
            height,
        }
    }
}
impl IntAbsLayer for IntMergeLayer {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn get_color(&self, x: usize, y: usize) -> Color {
        let mut offset_x = 0;
        for file_image_layer in &self.file_image_layers {
            let this_width = file_image_layer.width();
            if offset_x <= x && x < offset_x + this_width {
                return file_image_layer.get_color(x - offset_x, y);
            }
            offset_x += file_image_layer.width();
        }

        // This should never happen.
        println!("MergeLayer::get_color: unknown x={x}, y={y}");
        exit(0x0100);
        // color(0.0, 0.0, 0.0)
    }
}
