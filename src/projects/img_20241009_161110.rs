use crate::coord::{get_coord_chunks, ImgBuffer};
use crate::folding::get_path_out;
use crate::layer::{load_image_layer, AbsLayer, ExtractedLayer, FilteredLayer, MergeLayer};
use color_manipulator_rust::{
    add_list, color, filter_scalar_and_stretch, get_distance, mult, scalar, zero_but_one_in_square,
    POS, RGB,
};

pub fn img_20241009_161110() {
    let file_image_layer = load_image_layer("./input/IMG20241009161110.jpg");

    let extract_layer = ExtractedLayer::new(Box::new(file_image_layer), 550, 130, 3600, 3050);

    let calculate_color_func = |c: RGB, p: POS| {
        // let pomello = zero_but_one_in_circle(p, POS { x: 0.54, y: 0.38 }, 0.08);
        let pomello_center = 0.8 - get_distance(p, POS { x: 0.54, y: 0.38 }) * 0.8;
        let xist = 1.0 - filter_scalar_and_stretch(c.r, 0.0, 0.45);
        // let istanbul = 1.0 - filter_scalar_and_stretch(c.r, 0.1, 0.5);
        let istanbul = 1.0 - filter_scalar_and_stretch(c.r, 0.0, 0.18);
        let quad_xist = zero_but_one_in_square(p, POS { x: 0.42, y: 0.05 }, POS { x: 0.6, y: 0.2 });
        let quad_istanbul =
            zero_but_one_in_square(p, POS { x: 0.28, y: 0.7 }, POS { x: 0.79, y: 0.85 });
        let riflessi_cerchi = 1.0 - filter_scalar_and_stretch(c.r, 0.1, 0.45);
        let color_istanbul = color(0.7, 0.35, 0.35);
        let color_xist = color(0.35, 0.65, 0.5);
        let bg_color = color(0.8, 0.56, 0.2);
        return add_list(&[
            Some(mult(
                mult(scalar(riflessi_cerchi * pomello_center), bg_color),
                scalar((1.0 - quad_istanbul) * (1.0 - quad_xist)),
            )),
            Some(mult(scalar(quad_xist), mult(scalar(xist), color_xist))),
            Some(mult(
                scalar(quad_istanbul),
                mult(scalar(istanbul), color_istanbul),
            )),
        ]);
    };

    let filtered_layer = FilteredLayer::new(Box::new(extract_layer), calculate_color_func);

    let output_file_path = "20241009_161110.jpg";
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
            format!("{}-{}.jpg", get_path_out(output_file_path), i),
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
        filepaths.push(format!("{}-{}.jpg", get_path_out(output_file_path), i));
    }
    let merge_layer = MergeLayer::new(filepaths);
    let output_file_path = "20241009_161110_new.jpg";
    // let output_file_path = "20241009_161110.jpg";
    let abs_layer: &dyn AbsLayer = &merge_layer;
    abs_layer.save(output_file_path);
}
