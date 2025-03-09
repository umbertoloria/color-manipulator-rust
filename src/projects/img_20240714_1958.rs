use crate::folding::get_path_out;
use crate::int::int_color::rgb_to_color;
use crate::int::int_layer::{
    load_int_file_image_layer_box,
    IntAbsLayer, MultipleLayer,
};
use color_manipulator_rust::{add_list, color, get_distance, get_r, POS, RGB};
use std::collections::HashMap;

fn cell_shading(scalar: f32, num_shades: usize) -> f32 {
    // Gives from 0.0 to 1.0 in "num_shades" phases.
    let num_shades_f32 = (num_shades - 1) as f32;
    scale_on_shade_excl(scalar, num_shades) as f32 / num_shades_f32
}
fn scale_on_shade_excl(scalar: f32, num_shades: usize) -> usize {
    // Output: [0, num_shades)
    let result = f32::round(scalar / (1.0 / num_shades as f32)) as usize;
    if result >= num_shades {
        num_shades - 1
    } else {
        result
    }
}

fn grayscale(rgb: RGB, strength_scalar: f32) -> RGB {
    let mean = (rgb.r + rgb.g + rgb.b) / 3.0;
    let dist_r = rgb.r - mean;
    let dist_g = rgb.g - mean;
    let dist_b = rgb.b - mean;
    color(
        // R
        mean + dist_r * (1.0 - strength_scalar),
        // G
        mean + dist_g * (1.0 - strength_scalar),
        // B
        mean + dist_b * (1.0 - strength_scalar),
    )
}

fn set_boundaries(scalar: f32) -> f32 {
    if scalar > 1.0 {
        1.0
    } else if scalar < 0.0 {
        0.0
    } else {
        scalar
    }
}

pub fn img_20240714_1958() {
    /*
    // Mean image
    let image = load_int_file_image_layer("./input/20240714_1958.jpg");
    let mut canvas = create_mean_canvas_layer_from_int_layer(&image as &dyn IntAbsLayer);
    let mean_times = 300;
    for i in 0..mean_times {
        let new_canvas = create_mean_canvas_layer_from_int_layer(&canvas as &dyn IntAbsLayer);
        println!("Painted mean {i}");
        if i == 51 {
            create_int_filtered_layer(Box::new(canvas), |c, _, _, _| c)
                .save(get_path_out("20240714_1958_mean_50.png"));
        } else if i == 11 {
            create_int_filtered_layer(Box::new(canvas), |c, _, _, _| c)
                .save(get_path_out("20240714_1958_mean_10.png"));
        }
        canvas = new_canvas;
    }
    create_int_filtered_layer(Box::new(canvas), |c, _, _, _| c)
        .save(get_path_out("20240714_1958_mean_300.png"));
    */

    // Real image
    MultipleLayer::new(
        load_int_file_image_layer_box("./input/20240714_1958.jpg"),
        HashMap::from([
            (
                10,
                load_int_file_image_layer_box("./out/20240714_1958_mean_10.png"),
            ),
            (
                50,
                load_int_file_image_layer_box("./out/20240714_1958_mean_50.png"),
            ),
            (
                300,
                load_int_file_image_layer_box("./out/20240714_1958_mean_300.png"),
            ),
        ]),
        |c, _p, width, height, others| {
            let c_0 = c.to_rgb();
            let p = _p.to_pos(width, height);

            let c_10 = others.get(&10).unwrap();
            let c_50 = others.get(&50).unwrap();
            let c_300 = others.get(&300).unwrap();

            let center = POS { x: 0.48, y: 0.555 };
            let closer_from_center = set_boundaries(1.0 - get_distance(p, center) * 1.8);
            let cell_shading_phases = cell_shading(closer_from_center, 5);
            let o1_x = 25;
            let o1_y = 25;
            if cell_shading_phases == 0.0 {
                rgb_to_color(add_list(&[Some(color(0.0, 0.03, 0.4))]))
            } else if cell_shading_phases == 0.25 {
                let mut new_x = _p.x - o1_x;
                if new_x >= c_300.width() {
                    new_x = c_300.width() - 1;
                }

                let mut new_y = _p.y + o1_y;
                if new_y >= c_300.height() {
                    new_y = c_300.height() - 1;
                }
                let c = c_300.get_color(new_x, new_y).to_rgb();

                rgb_to_color(add_list(&[
                    Some(color(0.0, 0.03, 0.4)),
                    Some(color(get_r(c) * cell_shading_phases, 0.0, 0.0)),
                ]))
            } else if cell_shading_phases == 0.5 {
                let c = c_50.get_color(_p.x - o1_x, _p.y - o1_y).to_rgb();
                rgb_to_color(add_list(&[
                    Some(color(0.0, 0.03, 0.4)),
                    Some(color(get_r(c) * cell_shading_phases, 0.0, 0.0)),
                ]))
            } else if cell_shading_phases == 0.75 {
                let c = c_10.get_color(_p.x + o1_x, _p.y + o1_y).to_rgb();
                rgb_to_color(add_list(&[
                    Some(color(0.0, 0.03, 0.4)),
                    Some(color(get_r(c) * cell_shading_phases, 0.0, 0.0)),
                ]))
            // } else if cell_shading_phases == 1.0 {
            } else {
                let c = c_0;
                rgb_to_color(add_list(&[
                    Some(color(0.0, 0.03, 0.4)),
                    Some(color(get_r(c) * cell_shading_phases, 0.0, 0.0)),
                ]))
            }
        },
    )
    .save(get_path_out("20240714_1958.png"));
}
