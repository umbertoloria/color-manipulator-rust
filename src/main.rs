use crate::projects::img_20230226_201501::img_20230226_201501;
use crate::projects::img_20230301_224920_two_images::img_20230301_224920_two_images;
use crate::projects::img_20230301_225057::img_20230301_225057;
use crate::projects::img_20231002_103537_three_images::img_20231002_103537_three_images;
use crate::projects::img_20241009_161110::img_20241009_161110;

mod coord;
mod folding;
mod layer;
pub mod projects;

fn main() {
    // println!("Creating images: from sources and configs");

    img_20230226_201501();
    img_20230301_224920_two_images();
    img_20230301_225057();
    img_20231002_103537_three_images();
    img_20241009_161110();

    // create_diff_images();
}

/*
fn create_diff_images() {
    println!("Creating images: diff from oracles");
    create_and_save_diff_image_from_oracle("20230226_201501.jpg");
    create_and_save_diff_image_from_oracle("20230301_224920_1.jpg");
    create_and_save_diff_image_from_oracle("20230301_224920_2.jpg");
    create_and_save_diff_image_from_oracle("20230303_162133.jpg");
    create_and_save_diff_image_from_oracle("20231002_103537_0.jpg");
    create_and_save_diff_image_from_oracle("20231002_103537_1.jpg");
    create_and_save_diff_image_from_oracle("20231002_103537_2.jpg");
}
fn create_and_save_diff_image_from_oracle(name: &str) {
    println!("File \"{}\"", name);
    create_diff_layer(
        Box::new(load_image_layer(get_path_out(name).as_str())),
        Box::new(load_image_layer(get_path_oracle(name).as_str())),
    )
    .save(get_path_oracle_diff(name).as_str());
}
*/
