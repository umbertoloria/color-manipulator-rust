use crate::folding::get_path_out;
use crate::int::computing::create_mean_canvas_layer_from_int_layer;
use crate::int::int_color::rgb_to_color;
use crate::int::int_layer::{
    create_int_filtered_layer, load_int_file_image_layer, load_int_file_image_layer_box,
    IntAbsLayer,
};
use color_manipulator_rust::{add_list, color, get_distance, get_r, POS, RGB};

fn cell_shading(scalar: f32, num_shades: usize) -> f32 {
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
    create_int_filtered_layer(
        load_int_file_image_layer_box("./input/20240714_1958.png"),
        |c, p, width, height| {
            let c = c.to_rgb();
            let p = p.to_pos(width, height);

            let center = POS { x: 0.48, y: 0.555 };
            let closer_from_center = set_boundaries(1.0 - get_distance(p, center) * 1.8);
            let shaded_value_gradient = cell_shading(closer_from_center, 5);

            return rgb_to_color(add_list(&[
                Some(color(0.0, 0.03, 0.4)),
                Some(color(
                    get_r(c) * cell_shading(shaded_value_gradient, 5),
                    0.0,
                    0.0,
                )),
            ]));
        },
    )
    .save(get_path_out("20240714_1958.png"));

    // Mean image
    let image = load_int_file_image_layer("./input/20240714_1958.png");
    let mut canvas = create_mean_canvas_layer_from_int_layer(&image as &dyn IntAbsLayer);
    let mean_times = 300;
    for i in 0..mean_times {
        let new_canvas = create_mean_canvas_layer_from_int_layer(&canvas as &dyn IntAbsLayer);
        canvas = new_canvas;
        println!("Painted mean {i}");
    }
    create_int_filtered_layer(Box::new(canvas), |c, _, _, _| c)
        .save(get_path_out("20240714_1958_mean_300.png"));
}
