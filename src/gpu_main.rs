use crate::folding::{get_path_in, get_path_out_gpu};
use crate::gpu::gpu_context::create_gpu_context;
use crate::gpu::gpu_image_processing::gpu_image_processing;
use crate::gpu::shaders::read_file::make_safe_filepath;
use std::fs::read;

pub async fn gpu_main() {
    // SETUP
    let gpu_context = create_gpu_context().await;

    // Bind Group Layout: Texture Material
    let material_bind_group_layout = gpu_context
        .create_bind_group_layout()
        .add_material()
        .build("Material Bind Group Layout");

    let bytes_img_20230226_201501 =
        read(make_safe_filepath(&get_path_in("20230226_201501.jpg"))).expect("Can't read texture!");
    gpu_image_processing(
        &gpu_context,
        &material_bind_group_layout,
        &bytes_img_20230226_201501,
        &get_path_out_gpu("20230226_201501.png"),
    )
    .await;

    let bytes_img_20230301_224920 =
        read(make_safe_filepath(&get_path_in("20230301_224920.jpg"))).expect("Can't read texture!");
    gpu_image_processing(
        &gpu_context,
        &material_bind_group_layout,
        &bytes_img_20230301_224920,
        &get_path_out_gpu("20230301_224920_1.png"),
    )
    .await;
    gpu_image_processing(
        &gpu_context,
        &material_bind_group_layout,
        &bytes_img_20230301_224920,
        &get_path_out_gpu("20230301_224920_2.png"),
    )
    .await;

    let bytes_img_20230301_225057 =
        read(make_safe_filepath(&get_path_in("20230301_225057.jpg"))).expect("Can't read texture!");
    gpu_image_processing(
        &gpu_context,
        &material_bind_group_layout,
        &bytes_img_20230301_225057,
        &get_path_out_gpu("20230301_225057.png"),
    )
    .await;

    let bytes_img_20231002_103537 =
        read(make_safe_filepath(&get_path_in("20231002_103537.jpg"))).expect("Can't read texture!");
    gpu_image_processing(
        &gpu_context,
        &material_bind_group_layout,
        &bytes_img_20231002_103537,
        &get_path_out_gpu("20231002_103537_1.png"),
    )
    .await;
    gpu_image_processing(
        &gpu_context,
        &material_bind_group_layout,
        &bytes_img_20231002_103537,
        &get_path_out_gpu("20231002_103537_2.png"),
    )
    .await;
    gpu_image_processing(
        &gpu_context,
        &material_bind_group_layout,
        &bytes_img_20231002_103537,
        &get_path_out_gpu("20231002_103537_3.png"),
    )
    .await;

    let bytes_img_20240714_1958 =
        read(make_safe_filepath(&get_path_in("20240714_1958.png"))).expect("Can't read texture!");
    gpu_image_processing(
        &gpu_context,
        &material_bind_group_layout,
        &bytes_img_20240714_1958,
        &get_path_out_gpu("20240714_1958.png"),
    )
    .await;

    let bytes_img_20241009161110 = read(make_safe_filepath(&get_path_in("IMG20241009161110.jpg")))
        .expect("Can't read texture!");
    gpu_image_processing(
        &gpu_context,
        &material_bind_group_layout,
        &bytes_img_20241009161110,
        &get_path_out_gpu("20241009_161110.png"),
    )
    .await;

    /*create_and_save_int_diff_image_from_paths(
        //
        GPU_INPUT_FILENAME,
        GPU_FINAL_FILENAME,
        GPU_DIFF_FILENAME,
    );*/
}
