use crate::coord::get_coord_chunks;
use crate::folding::{get_path_in, get_path_out};
use crate::int::int_img_buffer::IntImgBuffer;
use crate::int::int_layer::{
    create_int_filtered_layer, load_int_file_image_layer_box, IntAbsLayer, IntMergeLayer,
};

pub fn img_20250201_0220() {
    let filename = "20250201_0220.png";

    let filtered_layer = create_int_filtered_layer(
        load_int_file_image_layer_box(&get_path_in(filename)),
        |c, p| {
            if c.r == 237 && c.g == 28 && c.b == 36 {
                // println!("Color 1");
            } else if c.r == 255 && c.g == 242 && c.b == 0 {
                // println!("Color 2");
            } else if c.r == 34 && c.g == 177 && c.b == 76 {
                // println!("Color 3");
            } else if c.r == 0 && c.g == 162 && c.b == 232 {
                // println!("Color 4");
            } else {
                println!("Unknown: {:?}", c);
            }
            c
            /*let rgb = c.to_rgb();
            let result = rgb_to_color(add_list(&[
                //
                Some(rgb),
                Some(color(0.0, 0.0, 0.005)),
            ]));
            println!("{}", result.b);
            return result;*/
        },
    );
    let abs_layer: &dyn IntAbsLayer = &filtered_layer;
    let output_file_path = format!("{}_new.png", filename);
    // abs_layer.save(&output_file_path); // Avoid calculating the entire image.

    // Separated chunks
    let width = abs_layer.width();
    let height = abs_layer.height();
    let chunks_count = 4;
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
    let merge_layer = IntMergeLayer::new(filepaths);
    let abs_layer: &dyn IntAbsLayer = &merge_layer;
    abs_layer.save(&output_file_path);
}
