use color_manipulator_rust::RGB;
use image::{GenericImageView, ImageBuffer, Rgb};
use std::path::PathBuf;
use std::{fs, io};

fn calculate_color(c: RGB) -> RGB {
    return RGB {
        r: c.r * 0.5,
        g: c.g * 0.5,
        b: c.b * 0.5,
    };
}

fn get_file_paths(dir: &str) -> Result<Vec<PathBuf>, io::Error> {
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_file() {
            paths.push(path);
        }
    }
    Ok(paths)
}

fn main() -> Result<(), image::ImageError> {
    let dir_path = "./input";
    let file_paths = get_file_paths(dir_path)?;

    for path in file_paths {
        let img = image::open(path)?;

        let mut output_img = ImageBuffer::new(img.width(), img.height());

        for (x, y, pixel) in img.pixels() {
            let src_r = *(pixel.0.get(0).unwrap());
            let src_g = *(pixel.0.get(1).unwrap());
            let src_b = *(pixel.0.get(2).unwrap());

            // Calculate color
            let in_color = RGB {
                r: src_r as f32 / 255.0,
                g: src_g as f32 / 255.0,
                b: src_b as f32 / 255.0,
            };
            let out_color: RGB = calculate_color(in_color);

            // Write destination color
            output_img.put_pixel(x, y, Rgb([
                (out_color.r * 255.0) as u8,
                (out_color.g * 255.0) as u8,
                (out_color.b * 255.0) as u8,
            ]));
        }

        output_img.save("out/output.png")?;
    }

    Ok(())
}
