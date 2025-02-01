use crate::folding::get_path_in;
use crate::layer::{load_image_layer, AbsLayer, FilteredLayer};
use color_manipulator_rust::{add_list, color, filter_scalar_and_stretch, get_r, POS, RGB};

pub fn img_20240714_1958() {
    let filename = "20240714_1958.jpg";

    let file_image_layer = load_image_layer(&get_path_in(filename));
    let calculate_color_func = |c: RGB, p: POS| {
        return add_list(&[
            Some(color(0.0, 0.03, 0.4)),
            Some(color(
                0.0,
                0.0,
                filter_scalar_and_stretch(get_r(c), 0.5, 0.8) * 0.6,
            )),
        ]);
    };

    let filtered_layer = FilteredLayer::new(Box::new(file_image_layer), calculate_color_func);
    let abs_layer: &dyn AbsLayer = &filtered_layer;
    abs_layer.save(filename);
}
