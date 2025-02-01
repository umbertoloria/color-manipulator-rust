use crate::folding::{get_path_in, get_path_oracle, get_path_oracle_diff, get_path_out};
use crate::int::int_layer::{create_int_diff_layer, load_int_file_image_layer_box};
use crate::layer::{create_diff_layer, load_image_layer};

pub fn img_diffs_create() {
    println!("Creating images: diff from oracles");
    /*
    create_and_save_diff_image_from_oracle("20230226_201501.jpg");
    create_and_save_diff_image_from_oracle("20230301_224920_1.jpg");
    create_and_save_diff_image_from_oracle("20230301_224920_2.jpg");
    create_and_save_diff_image_from_oracle("20230303_162133.jpg");
    create_and_save_diff_image_from_oracle("20231002_103537_0.jpg");
    create_and_save_diff_image_from_oracle("20231002_103537_1.jpg");
    create_and_save_diff_image_from_oracle("20231002_103537_2.jpg");
    */
    create_and_save_int_diff_image_from_paths("20250201_0220.png");
}
fn create_and_save_diff_image_from_oracle(name: &str) {
    println!("File \"{}\"", name);
    create_diff_layer(
        Box::new(load_image_layer(get_path_out(name).as_str())),
        Box::new(load_image_layer(get_path_oracle(name).as_str())),
    )
    .save(get_path_oracle_diff(name).as_str());
}
fn create_and_save_int_diff_image_from_paths(filename_in: &str) {
    create_int_diff_layer(
        load_int_file_image_layer_box(&get_path_in(filename_in)),
        load_int_file_image_layer_box(&get_path_out(&format!("{}_new.png", filename_in))),
    )
    .save(get_path_out(&format!("{}_diff.png", filename_in)));
}
