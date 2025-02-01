use crate::folding::{get_path_oracle, get_path_oracle_diff, get_path_out};
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
    create_diff_layer(
        Box::new(load_image_layer("out/20240714_1958.jpg")),
        Box::new(load_image_layer("out/20240714_1958.jpg_new.jpg")),
    )
    .save("out/20240714_1958_diff.jpg");
}
fn create_and_save_diff_image_from_oracle(name: &str) {
    println!("File \"{}\"", name);
    create_diff_layer(
        Box::new(load_image_layer(get_path_out(name).as_str())),
        Box::new(load_image_layer(get_path_oracle(name).as_str())),
    )
    .save(get_path_oracle_diff(name).as_str());
}
