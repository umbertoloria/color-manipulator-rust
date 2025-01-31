use crate::layer::{create_filtered_layer, load_image_layer};
use color_manipulator_rust::{add, color, filter_scalar, get_r, mult, only_b, scalar, xor_color};

pub fn img_20230301_224920_two_images() {
    create_filtered_layer(
        Box::new(load_image_layer("./input/20230301_224920.jpg")),
        |c, _p| {
            add(
                mult(
                    scalar(filter_scalar(get_r(c), 0.6, 0.85) - 0.6),
                    color(10.0, 5.0, 0.0),
                ),
                only_b(0.35),
            )
        },
    )
    .save("20230301_224920_1.jpg");
    create_filtered_layer(
        Box::new(load_image_layer("./input/20230301_224920.jpg")),
        |c, _p| {
            xor_color(
                mult(
                    scalar(0.4 - filter_scalar(get_r(c), 0.35, 0.4)),
                    color(7.0, 10.0, 5.0),
                ),
                only_b(0.35),
            )
        },
    )
    .save("20230301_224920_2.jpg");
}
