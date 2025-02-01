use crate::int::int_color::rgb_to_color;
use crate::int::int_layer::{create_int_filtered_layer, load_int_file_image_layer_box};
use color_manipulator_rust::{add, color, filter_scalar, get_r, mult, only_b, scalar, xor_color};

pub fn img_20230301_224920_two_images() {
    create_int_filtered_layer(
        load_int_file_image_layer_box("./input/20230301_224920.jpg"),
        |c, _p| {
            let c = c.to_rgb();
            rgb_to_color(add(
                mult(
                    scalar(filter_scalar(get_r(c), 0.6, 0.85) - 0.6),
                    color(10.0, 5.0, 0.0),
                ),
                only_b(0.35),
            ))
        },
    )
    .save_via_chunks("20230301_224920_1.jpg");
    create_int_filtered_layer(
        load_int_file_image_layer_box("./input/20230301_224920.jpg"),
        |c, _p| {
            let c = c.to_rgb();
            rgb_to_color(xor_color(
                mult(
                    scalar(0.4 - filter_scalar(get_r(c), 0.35, 0.4)),
                    color(7.0, 10.0, 5.0),
                ),
                only_b(0.35),
            ))
        },
    )
    .save_via_chunks("20230301_224920_2.jpg");
}
