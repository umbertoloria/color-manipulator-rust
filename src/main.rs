use crate::folding::{get_path_oracle, get_path_oracle_diff, get_path_out};
use crate::layer::{create_and_save_filtered_layer, create_diff_layer, load_image_layer, AbsLayer};
use color_manipulator_rust::{add, add_list, color, filter_color, filter_scalar, filter_scalar_and_stretch, get_b, get_g, get_r, gradient_linear, mult, only_b, only_g, only_r, safe_color, scalar, sub, xor_color, POS, RGB};
use image::{GenericImageView, ImageBuffer, Rgba};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

mod layer;
mod folding;

fn get_file_paths(dir: &str) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            paths.push(path);
        }
    }
    paths
}

struct SourceFileConf {
    files: Vec<FinalFileConf>,
}
struct FinalFileConf {
    name: String,
    calculate_color: fn(c: RGB, p: POS) -> RGB,
}

fn main() -> Result<(), image::ImageError> {
    let mut list_source_file_conf: HashMap<&str, SourceFileConf> = HashMap::new();

    list_source_file_conf.insert("20230226_201501.jpg", SourceFileConf {
        files: vec!(
            FinalFileConf {
                name: String::from("20230226_201501.jpg"),
                calculate_color: |c, _p| {
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
                calculate_color: |c, _p| {
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
                calculate_color: |c, _p| {
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
                calculate_color: |c, _p| {
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

    /*let rgb_0_0 = color_hex("#ac7360");
    let rgb_1_1 = color_hex("#4c6d26");
    let rgb_1_2 = color_hex("#094837");
    let rgb_1_3 = color_hex("#480937");
    let rgb_1_4 = color_hex("#093748");
    let rgb_1_5 = color_hex("#15a267");
    let rgb_1_6 = color_hex("#d09f00");
    let rgb_1_7 = color_hex("#8c4615");
    let rgb_2_1 = color_hex("#0e0d1f");
    let rgb_2_2 = color_hex("#cc1a4d");
    let rgb_2_3 = color_hex("#00222f");
    let rgb_2_4 = color_hex("#001749");
    let rgb_2_5 = color_hex("#880c31");
    println!("{:?}", rgb_2_5);*/
    list_source_file_conf.insert("20231002_103537.jpg", SourceFileConf {
        files: vec!(
            FinalFileConf {
                name: String::from("20231002_103537_0.jpg"),
                calculate_color: |c, p| {
                    // Known data:
                    //   width=4624
                    //   height=3468
                    //   toAddOnAbove=578
                    //   toAddOnBelow=578
                    let converted_y = p.y * 4624.0;
                    if converted_y < 578.0
                        || converted_y >= (578.0 + 3468.0) {
                        // return rgb_0_0;
                        return RGB { r: 0.6745098, g: 0.4509804, b: 0.3764706 };
                    }
                    return c;
                },
            },
            FinalFileConf {
                name: String::from("20231002_103537_1.jpg"),
                calculate_color: |c, p| {
                    // Known data:
                    //   width=4624
                    //   height=3468
                    //   toAddOnAbove=578
                    //   toAddOnBelow=578
                    let converted_y = p.y * 4624.0;
                    if converted_y < 578.0
                        || converted_y >= (578.0 + 3468.0) {
                        // return rgb_1_1;
                        return RGB { r: 0.29803923, g: 0.42745098, b: 0.14901961 };
                    }
                    return add_list(&[

                        // Base
                        // Some(mult(rgb_1_2, scalar(0.10))),
                        Some(mult(RGB { r: 0.03529412, g: 0.28235295, b: 0.21568628 }, scalar(0.10))),

                        // Mattoni base
                        // Some(mult(rgb_1_3, scalar(
                        Some(mult(RGB { r: 0.28235295, g: 0.03529412, b: 0.21568628 }, scalar(
                            (filter_scalar_and_stretch(get_r(c), 0.11, 0.81)
                                - filter_scalar_and_stretch(get_b(c), 0.11, 0.81))
                                * 0.9 * gradient_linear(p.x, 0.5, 0.0)
                        ))),
                        // Some(mult(rgb_1_4, scalar(
                        Some(mult(RGB { r: 0.03529412, g: 0.21568628, b: 0.28235295 }, scalar(
                            (filter_scalar_and_stretch(get_r(c), 0.11, 0.81)
                                - filter_scalar_and_stretch(get_b(c), 0.11, 0.81))
                                * 0.9 * gradient_linear(p.x, 0.5, 1.0)
                        ))),

                        // Mattoni luce
                        // Some(mult(rgb_1_5, scalar(
                        Some(mult(RGB { r: 0.08235294, g: 0.63529414, b: 0.40392157 }, scalar(
                            (filter_scalar_and_stretch(get_r(c), 0.11, 0.81)
                                - filter_scalar_and_stretch(get_g(c), 0.11, 0.81))
                                * 0.9 * gradient_linear(p.x, 0.0, 0.5)))),
                        // Some(mult(rgb_1_6, scalar(
                        Some(mult(RGB { r: 0.8156863, g: 0.62352943, b: 0.0 }, scalar(
                            (filter_scalar_and_stretch(get_r(c), 0.11, 0.81)
                                - filter_scalar_and_stretch(get_g(c), 0.11, 0.81))
                                * 0.9 * gradient_linear(p.x, 1.0, 0.5)))),

                        // Effetti su mattoni
                        // Some(mult(rgb_1_6, scalar(
                        Some(mult(RGB { r: 0.8156863, g: 0.62352943, b: 0.0 }, scalar(
                            (filter_scalar_and_stretch(get_g(c), 0.5, 0.6)
                                - filter_scalar_and_stretch(get_b(c), 0.5, 0.6))
                                * 0.9))),

                        // Saracinesca
                        if p.y > 0.68 {
                            // Some(mult(rgb_1_7, scalar(filter_scalar_and_stretch(get_g(c), 0.85, 0.95) * 2.0)))
                            Some(mult(RGB { r: 0.54901963, g: 0.27450982, b: 0.08235294 }, scalar(filter_scalar_and_stretch(get_g(c), 0.85, 0.95) * 2.0)))
                        } else { None },

                        // Finestre
                        if p.y < 0.65 && p.x < 0.482 {
                            // Some(mult(rgb_1_6, scalar(filter_scalar_and_stretch(get_g(c), 0.85, 0.95) * 0.12)))
                            Some(mult(RGB { r: 0.8156863, g: 0.62352943, b: 0.0 }, scalar(filter_scalar_and_stretch(get_g(c), 0.85, 0.95) * 0.12)))
                        } else { None },
                        if p.y < 0.65 && p.x > 0.486 {
                            // Some(mult(rgb_1_5, scalar(filter_scalar_and_stretch(get_g(c), 0.85, 0.95) * 0.11)))
                            Some(mult(RGB { r: 0.08235294, g: 0.63529414, b: 0.40392157 }, scalar(filter_scalar_and_stretch(get_g(c), 0.85, 0.95) * 0.11)))
                        } else { None },
                    ]);
                },
            },
            FinalFileConf {
                name: String::from("20231002_103537_2.jpg"),
                calculate_color: |c, p| {
                    // Known data:
                    //   width=4624
                    //   height=3468
                    //   toAddOnAbove=578
                    //   toAddOnBelow=578
                    let converted_y = p.y * 4624.0;
                    if converted_y < 578.0
                        || converted_y >= (578.0 + 3468.0) {
                        // return rgb_2_1;
                        return RGB { r: 0.05490196, g: 0.050980393, b: 0.12156863 };
                    }
                    return add_list(&[
                        // Base
                        // Some(mult(rgb_2_2, scalar(0.02))),
                        Some(mult(RGB { r: 0.8, g: 0.101960786, b: 0.3019608 }, scalar(0.02))),

                        // Mattoni base
                        Some(mult(
                            scalar((
                                filter_scalar_and_stretch(get_r(c), 0.11, 0.81)
                                    - filter_scalar_and_stretch(get_b(c), 0.11, 0.81)
                            ) * 0.6),
                            // rgb_2_3,
                            RGB { r: 0.0, g: 0.13333334, b: 0.18431373 },
                        )),

                        // Mattoni luce
                        Some(mult(
                            scalar((
                                filter_scalar_and_stretch(get_r(c), 0.11, 0.81)
                                    - filter_scalar_and_stretch(get_g(c), 0.11, 0.81)
                            ) * 0.7),
                            // rgb_2_4,
                            RGB { r: 0.0, g: 0.09019608, b: 0.28627452 },
                        )),

                        // Effetti su mattoni
                        Some(mult(
                            scalar((
                                filter_scalar_and_stretch(get_g(c), 0.5, 0.6)
                                    - filter_scalar_and_stretch(get_b(c), 0.5, 0.6)
                            ) * 0.7),
                            // rgb_2_2,
                            RGB { r: 0.8, g: 0.101960786, b: 0.3019608 },
                        )),

                        // Mensole
                        Some(mult(
                            scalar(filter_scalar_and_stretch(get_g(c), 0.85, 0.95) * 0.23),
                            // rgb_2_5,
                            RGB { r: 0.53333336, g: 0.047058824, b: 0.19215687 },
                        )),
                    ]);
                },
            },
        ),
    });

    let files = get_file_paths("./input");

    println!("Creating images: from sources and configs");
    for file in files {
        let file_path_os_str = file.file_name().unwrap();
        let file_path_string = file_path_os_str.to_string_lossy().to_string();
        let file_path_str = &file_path_string[..];

        let source_file_conf_opt = list_source_file_conf.get(file_path_str);
        if source_file_conf_opt.is_some() {
            let source_file_conf = source_file_conf_opt.unwrap();

            let file_path = &file.as_path().to_str().unwrap();
            let mut layer = load_image_layer(file_path);

            // + Custom
            if file_path_str == "20231002_103537.jpg" {
                layer.exec(|img| {
                    let width = img.width() as usize;
                    let height = img.height() as usize;

                    // Backup image
                    let mut bkp_color_rows = vec!();
                    for y in 0..height {
                        let mut bkp_color_row = vec!();
                        for x in 0..width {
                            let src_color_hex = img.get_pixel(x as u32, y as u32);
                            bkp_color_row.push(src_color_hex);
                        }
                        bkp_color_rows.push(bkp_color_row);
                    }

                    // Resized image
                    let mut new_img = ImageBuffer::new(width as u32, width as u32);

                    let to_add_on_above = ((width - height) / 2) as usize; // "width" is the new "height".
                    // const toAddOnBelow = width - height - toAddOnAbove;

                    // Known data:
                    //   width=4624
                    //   height=3468
                    //   toAddOnAbove=578
                    //   toAddOnBelow=578

                    let default_color = Rgba::from([0, 0, 0, 1]);
                    // Above section
                    for y in 0..to_add_on_above { // "width" is the new "height".
                        for x in 0..width {
                            new_img.put_pixel(x as u32, y as u32, default_color);
                        }
                    }
                    // Image section
                    for y in to_add_on_above..(to_add_on_above + height) { // The old "height".
                        for x in 0..width {
                            let old_y = y - to_add_on_above;
                            let old_color = bkp_color_rows[old_y][x];
                            new_img.put_pixel(x as u32, y as u32, old_color);
                        }
                    }
                    // Below section
                    for y in to_add_on_above + height..width { // "width" is the new "height".
                        for x in 0..width {
                            new_img.put_pixel(x as u32, y as u32, default_color);
                        }
                    }
                    Some(new_img)
                });
            }
            // - Custom

            let mut layer_box: Box<dyn AbsLayer> = Box::new(layer);
            for final_file_conf in &source_file_conf.files {
                println!("File \"{}\" of dimensions", final_file_conf.name);
                create_and_save_filtered_layer(&mut layer_box, final_file_conf);
            }
        }
    }

    println!("Creating images: diff from oracles");
    create_and_save_diff_image_from_oracle("20230226_201501.jpg");
    create_and_save_diff_image_from_oracle("20230301_224920_1.jpg");
    create_and_save_diff_image_from_oracle("20230301_224920_2.jpg");
    create_and_save_diff_image_from_oracle("20230303_162133.jpg");
    create_and_save_diff_image_from_oracle("20231002_103537_0.jpg");
    create_and_save_diff_image_from_oracle("20231002_103537_1.jpg");
    create_and_save_diff_image_from_oracle("20231002_103537_2.jpg");

    Ok(())
}

fn create_and_save_diff_image_from_oracle(name: &str) {
    println!("File \"{}\"", name);
    create_diff_layer(Box::new(load_image_layer(get_path_out(name).as_str())),
                      Box::new(load_image_layer(get_path_oracle(name).as_str())))
        .save(get_path_oracle_diff(name).as_str());
}
