use crate::folding::get_path_out;
use crate::int::int_color::rgb_to_color;
use crate::int::int_layer::{create_int_filtered_layer, load_int_file_image_layer_box};
use color_manipulator_rust::{
    add, color, filter_scalar_and_stretch, get_b, get_g, get_r, mult, scalar,
};

pub fn img_20230301_225057() {
    create_int_filtered_layer(
        load_int_file_image_layer_box("./input/20230301_225057.jpg"),
        |c, _, _, _| {
            let c = c.to_rgb();
            rgb_to_color(add(
                color(0.0, 0.2, 0.15),
                add(
                    mult(
                        scalar(filter_scalar_and_stretch(get_b(c), 0.28, 0.39)),
                        color(0.0, 0.6, 0.0),
                    ),
                    add(
                        mult(
                            scalar(filter_scalar_and_stretch(get_r(c), 0.1, 0.2)),
                            color(0.001, 0.025, 0.008),
                        ),
                        mult(
                            scalar(filter_scalar_and_stretch(get_g(c), 0.68, 0.7)),
                            color(0.6, 0.8, 0.01),
                        ),
                    ),
                ),
            ))
        },
    )
    .save(get_path_out("20230303_162133.jpg"));
}
