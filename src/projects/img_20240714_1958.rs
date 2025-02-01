use crate::coord::Coord;
use crate::folding::get_path_in;
use crate::layer::{create_filtered_strict_layer, load_image_layer_box, AbsLayer};
use color_manipulator_rust::{add_list, color, filter_scalar_and_stretch, get_r, RGB};

pub fn img_20240714_1958() {
    let filename = "20240714_1958.jpg";

    let filtered_layer = create_filtered_strict_layer(
        load_image_layer_box(&get_path_in(filename)),
        |c: RGB, p: Coord| {
            return add_list(&[
                Some(color(0.0, 0.03, 0.4)),
                Some(color(
                    0.0,
                    0.0,
                    filter_scalar_and_stretch(get_r(c), 0.5, 0.8) * 0.6,
                )),
            ]);
        },
    );
    let abs_layer: &dyn AbsLayer = &filtered_layer;
    abs_layer.save(filename);
}
