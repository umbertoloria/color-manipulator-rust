use crate::folding::get_path_in;
use crate::int::chunking_layer::split_layer_in_chunks_and_save_parts_and_combined;
use crate::int::int_layer::{
    create_int_filtered_layer, load_int_file_image_layer_box, IntAbsLayer,
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
    split_layer_in_chunks_and_save_parts_and_combined(abs_layer, &filename);
}
