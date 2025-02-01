use crate::coord::get_coord_chunks;
use crate::folding::get_path_out;
use crate::int::int_img_buffer::IntImgBuffer;
use crate::int::int_layer::{IntAbsLayer, IntMergeLayer};
use std::fs::remove_file;

pub fn split_layer_in_chunks_and_save_parts_and_combined(
    abs_layer: &dyn IntAbsLayer,
    filename: &str,
    chunks_count: usize,
) {
    // TODO: Multithreading here please

    // Separated chunks
    let width = abs_layer.width();
    let height = abs_layer.height();
    let coord_chunks = get_coord_chunks(chunks_count, width, height);
    let mut i = 0;

    let mut filepaths = Vec::new();

    for coord_chunk in coord_chunks {
        let filepath = format!("{}-{}.png", get_path_out(filename), i);

        // TODO: Extract method **1
        let mut img_buffer_chunk = IntImgBuffer::new(
            coord_chunk.get_width(),
            coord_chunk.get_height(),
            filepath.clone(),
        );
        img_buffer_chunk.paint(|x, y| {
            abs_layer.get_color(coord_chunk.top_left.x + x, coord_chunk.top_left.y + y)
        });
        img_buffer_chunk.save();

        filepaths.push(filepath);
        i += 1;
    }

    // Merge chunks
    let merge_layer = IntMergeLayer::new(&filepaths);
    let abs_layer: &dyn IntAbsLayer = &merge_layer;

    abs_layer.save(&get_path_out(filename));

    // Remove files of chunks
    for filepath in &filepaths {
        remove_file(filepath).unwrap();
    }
}
