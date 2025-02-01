use crate::coord::{get_coord_chunks, Coord, ImgBuffer};
use crate::folding::{get_path_in, get_path_out};
use crate::layer::{create_filtered_strict_layer, load_image_layer_box, AbsLayer, MergeLayer};
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
    // abs_layer.save(output_file_path); // Avoid calculating the entire image.

    // Separated chunks
    let width = abs_layer.width();
    let height = abs_layer.height();
    let chunks_count = 4;
    let coord_chunks = get_coord_chunks(chunks_count, width, height);
    let mut i = 0;
    for coord_chunk in coord_chunks {
        let mut img_buffer_chunk = ImgBuffer::new(
            coord_chunk.get_width() as u32,
            coord_chunk.get_height() as u32,
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
    let merge_layer = MergeLayer::new(filepaths);
    let output_file_path = format!("{}_new.jpg", filename);
    // let output_file_path = "20241009_161110.jpg";
    let abs_layer: &dyn AbsLayer = &merge_layer;
    abs_layer.save(&output_file_path);
}
