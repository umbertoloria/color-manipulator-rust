use color_manipulator_rust::{add, color, filter_color, filter_scalar, filter_scalar_and_stretch, get_b, get_g, get_r, mult, only_b, only_g, only_r, safe_color, scalar, sub, xor_color, RGB};
use image::{GenericImageView, ImageBuffer, Rgb};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, io};

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

struct SourceFileConf {
    files: Vec<FinalFileConf>,
}
struct FinalFileConf {
    name: String,
    calculate_color: fn(rgba: RGB) -> RGB,
}

fn main() -> Result<(), image::ImageError> {
    let mut list_source_file_conf: HashMap<&str, SourceFileConf> = HashMap::new();

    list_source_file_conf.insert("20230226_201501.jpg", SourceFileConf {
        files: vec!(
            FinalFileConf {
                name: String::from("20230226_201501.jpg"),
                calculate_color: |c| {
                    let custom_channel_1 = safe_color(((1.0 - c.b) - 0.5) * 10.0 + 0.5);

                    return add(
                        mult(
                            add(
                                only_g(custom_channel_1 * 0.2),
                                only_b(custom_channel_1),
                            ),
                            scalar(0.85)
                        ),
                        only_r(
                            get_g(
                                sub(
                                    filter_color(c, 0.5, 0.7),
                                    scalar(0.5),
                                ),
                            ),
                        ),
                    );
                },
            },
        ),
    });

    list_source_file_conf.insert("20230301_224920.jpg", SourceFileConf {
        files: vec!(
            FinalFileConf {
                name: String::from("20230301_224920_1.jpg"),
                calculate_color: |c| {
                    add(
                        mult(
                            scalar(filter_scalar(get_r(c), 0.6, 0.85) - 0.6),
                            color(
                                10.0,
                                5.0,
                                0.0,
                            ),
                        ),
                        only_b(0.35),
                    )
                },
            },
            FinalFileConf {
                name: String::from("20230301_224920_2.jpg"),
                calculate_color: |c| {
                    xor_color(
                        mult(
                            scalar(0.4 - filter_scalar(get_r(c), 0.35, 0.4)),
                            color(
                                7.0,
                                10.0,
                                5.0,
                            ),
                        ),
                        only_b(0.35),
                    )
                },
            },
        ),
    });

    list_source_file_conf.insert("20230301_225057.jpg", SourceFileConf {
        files: vec!(
            FinalFileConf {
                name: String::from("20230303_162133.jpg"),
                calculate_color: |c| {
                    add(
                        color(
                            0.0,
                            0.2,
                            0.15,
                        ),
                        add(
                            mult(
                                scalar(filter_scalar_and_stretch(get_b(c), 0.28, 0.39)),
                                color(
                                    0.0,
                                    0.6,
                                    0.0,
                                ),
                            ),
                            add(
                                mult(
                                    scalar(filter_scalar_and_stretch(get_r(c), 0.1, 0.2)),
                                    color(
                                        0.001,
                                        0.025,
                                        0.008,
                                    ),
                                ),
                                mult(
                                    scalar(filter_scalar_and_stretch(get_g(c), 0.68, 0.7)),
                                    color(
                                        0.6,
                                        0.8,
                                        0.01,
                                    ),
                                ),
                            ),
                        ),
                    )
                },
            },
        ),
    });

    let files = get_file_paths("./input")?;

    for file in files {
        let file_path_os_str = file.file_name().unwrap();
        let file_path_string = file_path_os_str.to_string_lossy().to_string();
        let file_path_str = &file_path_string[..];

        let source_file_conf_opt = list_source_file_conf.get(file_path_str);
        if source_file_conf_opt.is_some() {
            let source_file_conf = source_file_conf_opt.unwrap();

            let img = image::open(&file)?;

            for final_file_conf in &source_file_conf.files {
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
                    let final_file_conf_fn = final_file_conf.calculate_color;
                    let out_color: RGB = final_file_conf_fn(in_color);

                    // Write destination color
                    output_img.put_pixel(x, y, Rgb([
                        (out_color.r * 255.0) as u8,
                        (out_color.g * 255.0) as u8,
                        (out_color.b * 255.0) as u8,
                    ]));
                }

                let output_file_path = format!("out/{}", final_file_conf.name);
                output_img.save(output_file_path)?;
            }
        }
    }

    Ok(())
}
