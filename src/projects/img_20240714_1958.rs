use crate::coord::get_coord_chunks;
use crate::folding::{get_path_in, get_path_out};
use crate::int_color::rgb_to_color;
use crate::int_img_buffer::IntImgBuffer;
use crate::int_layer::{
    create_int_filtered_layer, load_int_file_image_layer_box, IntAbsLayer, IntMergeLayer,
};
use color_manipulator_rust::{add_list, color, filter_scalar_and_stretch, get_r};

pub fn img_20240714_1958() {
    let filename = "20240714_1958.jpg";

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
    // abs_layer.save(output_file_path); // Avoid calculating the entire image.

    // Separated chunks
    let width = abs_layer.width();
    let height = abs_layer.height();
    let chunks_count = 4;
    let coord_chunks = get_coord_chunks(chunks_count, width, height);
    let mut i = 0;
    for coord_chunk in coord_chunks {
        let mut img_buffer_chunk = IntImgBuffer::new(
            coord_chunk.get_width(),
            coord_chunk.get_height(),
            format!("{}-{}.jpg", get_path_out(filename), i),
        );
        img_buffer_chunk.paint(|x, y| {
            abs_layer.get_color(coord_chunk.top_left.x + x, coord_chunk.top_left.y + y)
        });
        img_buffer_chunk.save();
        i += 1;
    }

    // Merge chunks
    let mut filepaths = Vec::new();
    for i in 0..chunks_count {
        filepaths.push(format!("{}-{}.jpg", get_path_out(filename), i));
    }
    let merge_layer = IntMergeLayer::new(filepaths);
    let output_file_path = format!("{}_new.jpg", filename);
    // let output_file_path = "20241009_161110.jpg";
    let abs_layer: &dyn IntAbsLayer = &merge_layer;
    abs_layer.save(&output_file_path);
}
