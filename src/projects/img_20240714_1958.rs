use crate::folding::get_path_in;
use crate::int::chunking_layer::split_layer_in_chunks_and_save_parts_and_combined;
use crate::int::int_color::rgb_to_color;
use crate::int::int_layer::{
    create_int_filtered_layer, load_int_file_image_layer_box, IntAbsLayer,
};
use color_manipulator_rust::{add_list, color, filter_scalar_and_stretch, get_r};

pub fn img_20240714_1958() {
    let filename = "20240714_1958.png";

    let filtered_layer = create_int_filtered_layer(
        load_int_file_image_layer_box(&get_path_in(filename)),
        |c, p| {
            let rgb = c.to_rgb();
            return rgb_to_color(add_list(&[
                Some(color(0.0, 0.03, 0.4)),
                Some(color(
                    0.0,
                    0.0,
                    filter_scalar_and_stretch(get_r(rgb), 0.5, 0.8) * 0.6,
                )),
            ]));
        },
    );
    let abs_layer: &dyn IntAbsLayer = &filtered_layer;
    // abs_layer.save(&get_path_out(filename));
    split_layer_in_chunks_and_save_parts_and_combined(abs_layer, &filename, 8);
}
