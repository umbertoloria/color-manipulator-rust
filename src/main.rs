use crate::gpu::render_loop::{
    gpu_main, GPU_DIFF_FILENAME, GPU_FINAL_FILENAME, GPU_INPUT_FILENAME,
};
use crate::projects::img_test_img_diffs::create_and_save_int_diff_image_from_paths;

mod coord;
mod folding;
mod gpu;
mod int;
mod projects;

fn main() {
    /*
    img_20230226_201501();
    img_20230301_224920_two_images();
    img_20230301_225057();
    img_20231002_103537_three_images();
    img_20241009_161110();
    img_20240714_1958();
    // test_img_diffs();
    */

    pollster::block_on(gpu_main());
    create_and_save_int_diff_image_from_paths(
        //
        GPU_INPUT_FILENAME,
        GPU_FINAL_FILENAME,
        GPU_DIFF_FILENAME,
    );
}
