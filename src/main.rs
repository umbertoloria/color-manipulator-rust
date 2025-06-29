use crate::folding::{get_path_in, get_path_out_gpu};
use crate::gpu::render_loop::gpu_main;

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

    pollster::block_on(
        //
        gpu_main(
            //
            &get_path_in("20230226_201501.jpg"),
            &get_path_out_gpu("20230226_201501.png"),
        ),
    );
    pollster::block_on(
        //
        gpu_main(
            //
            &get_path_in("20230301_224920.jpg"),
            &get_path_out_gpu("20230301_224920_1.png"),
        ),
    );
    pollster::block_on(
        //
        gpu_main(
            //
            &get_path_in("20230301_224920.jpg"),
            &get_path_out_gpu("20230301_224920_2.png"),
        ),
    );
    pollster::block_on(
        //
        gpu_main(
            //
            &get_path_in("20230301_225057.jpg"),
            &get_path_out_gpu("20230301_225057.png"),
        ),
    );
    pollster::block_on(
        //
        gpu_main(
            //
            &get_path_in("20231002_103537.jpg"),
            &get_path_out_gpu("20231002_103537.png"),
        ),
    );
    pollster::block_on(
        //
        gpu_main(
            //
            &get_path_in("20231002_103537_resized.jpg"),
            &get_path_out_gpu("20231002_103537_resized.png"),
        ),
    );
    pollster::block_on(
        //
        gpu_main(
            //
            &get_path_in("20240714_1958.png"),
            &get_path_out_gpu("20240714_1958.png"),
        ),
    );
    pollster::block_on(
        //
        gpu_main(
            //
            &get_path_in("20250201_0220.png"),
            &get_path_out_gpu("20250201_0220.png"),
        ),
    );
    pollster::block_on(
        //
        gpu_main(
            //
            &get_path_in("IMG20241009161110.jpg"),
            &get_path_out_gpu("IMG20241009161110.png"),
        ),
    );
    /*create_and_save_int_diff_image_from_paths(
        //
        GPU_INPUT_FILENAME,
        GPU_FINAL_FILENAME,
        GPU_DIFF_FILENAME,
    );*/
}
