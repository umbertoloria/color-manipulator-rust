use image::{GenericImageView, ImageBuffer, Rgb};

fn main() -> Result<(), image::ImageError> {
    let img = image::open("input/20230301_224920.jpg")?;

    let mut output_img = ImageBuffer::new(img.width(), img.height());

    for (x, y, pixel) in img.pixels() {
        // println!("Pixel at ({}, {}): {:?}", x, y, pixel);
        let r = *(pixel.0.get(0).unwrap());
        let g = *(pixel.0.get(1).unwrap());
        let b = *(pixel.0.get(2).unwrap());
        // println!("   : {},{},{}\n", r, g, b);

        let new_r = (r as f32 * 0.5) as u8;
        let new_g = (g as f32 * 0.5) as u8;
        let new_b = (b as f32 * 0.5) as u8;

        output_img.put_pixel(x, y, Rgb([new_r, new_g, new_b]));
    }

    output_img.save("out/output.png")?;

    Ok(())
}
