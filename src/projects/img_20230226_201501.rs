use crate::folding::get_path_out;
use crate::int::int_color::rgb_to_color;
use crate::int::int_layer::{create_int_filtered_layer, load_int_file_image_layer_box};
use color_manipulator_rust::{
    add, filter_color, get_g, mult, only_b, only_g, only_r, safe_color, scalar, sub,
};

pub fn img_20230226_201501() {
    create_int_filtered_layer(
        load_int_file_image_layer_box("./input/20230226_201501.jpg"),
        |c, _, _, _| {
            let c = c.to_rgb();
            let custom_channel_1 = safe_color(((1.0 - c.b) - 0.5) * 10.0 + 0.5);

            return rgb_to_color(add(
                mult(
                    add(only_g(custom_channel_1 * 0.2), only_b(custom_channel_1)),
                    scalar(0.85),
                ),
                only_r(get_g(sub(filter_color(c, 0.5, 0.7), scalar(0.5)))),
            ));
        },
    )
    .save(get_path_out("20230226_201501.jpg"));
}
