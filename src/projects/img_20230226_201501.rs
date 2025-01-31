use crate::layer::{create_filtered_layer, load_image_layer};
use color_manipulator_rust::{
    add, filter_color, get_g, mult, only_b, only_g, only_r, safe_color, scalar, sub,
};

pub fn img_20230226_201501() {
    create_filtered_layer(
        Box::new(load_image_layer("./input/20230226_201501.jpg")),
        |c, _p| {
            let custom_channel_1 = safe_color(((1.0 - c.b) - 0.5) * 10.0 + 0.5);

            return add(
                mult(
                    add(only_g(custom_channel_1 * 0.2), only_b(custom_channel_1)),
                    scalar(0.85),
                ),
                only_r(get_g(sub(filter_color(c, 0.5, 0.7), scalar(0.5)))),
            );
        },
    )
    .save("20230226_201501.jpg");
}
