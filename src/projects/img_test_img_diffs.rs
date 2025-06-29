use crate::folding::{get_path_in, get_path_out};
use crate::int::int_layer::{create_int_diff_layer, load_int_file_image_layer_box};
use crate::projects::img_20250201_0220::img_20250201_0220;

pub fn test_img_diffs() {
    println!("Creating images: diff from ints");
    img_20250201_0220();
    create_and_save_int_diff_image_from_paths(
        //
        &get_path_in("20250201_0220.png"),
        &get_path_out("20250201_0220.png"),
        &get_path_out("20250201_0220.png_diff.png"),
    );
}

pub fn create_and_save_int_diff_image_from_paths(
    filename1: &str,
    filename2: &str,
    filename_diff: &str,
) {
    create_int_diff_layer(
        load_int_file_image_layer_box(filename1),
        load_int_file_image_layer_box(filename2),
    )
    .save(filename_diff.into());
}
