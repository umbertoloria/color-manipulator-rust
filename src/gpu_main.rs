use crate::folding::{get_path_in, get_path_out_gpu};
use crate::gpu::gpu_context::{create_gpu_context, GpuContext};
use crate::gpu::gpu_image_processing::gpu_image_processing;
use crate::gpu::renderer_backend::material::Material;
use crate::gpu::renderer_backend::mesh_builder::Mesh;
use crate::gpu::shaders::read_file::make_safe_filepath;
use glm::Vec2;
use std::fs::read;

pub async fn gpu_main() {
    // SETUP
    let gpu_context = create_gpu_context().await;

    // Bind Group Layout: Texture Material
    let material_bind_group_layout = gpu_context
        .create_bind_group_layout_builder()
        .add_material()
        .build("Material Bind Group Layout");

    {
        let material_image_input = gpu_context.create_material(
            &read(make_safe_filepath(&get_path_in("20230226_201501.jpg"))).unwrap(),
            "20230226_201501",
            &material_bind_group_layout,
        );
        let (bulk_image_size, image_mesh) = create_quad_mesh(&material_image_input, &gpu_context);
        gpu_image_processing(
            &gpu_context,
            &material_bind_group_layout,
            &material_image_input,
            &image_mesh,
            bulk_image_size,
            &get_path_out_gpu("20230226_201501.png"),
        )
        .await;
    }

    {
        let material_image_input = gpu_context.create_material(
            &read(make_safe_filepath(&get_path_in("20230301_224920.jpg"))).unwrap(),
            "20230301_224920",
            &material_bind_group_layout,
        );
        let (bulk_image_size, image_mesh) = create_quad_mesh(&material_image_input, &gpu_context);
        gpu_image_processing(
            &gpu_context,
            &material_bind_group_layout,
            &material_image_input,
            &image_mesh,
            bulk_image_size,
            &get_path_out_gpu("20230301_224920_1.png"),
        )
        .await;
        gpu_image_processing(
            &gpu_context,
            &material_bind_group_layout,
            &material_image_input,
            &image_mesh,
            bulk_image_size,
            &get_path_out_gpu("20230301_224920_2.png"),
        )
        .await;
    }

    {
        let material_image_input = gpu_context.create_material(
            &read(make_safe_filepath(&get_path_in("20230301_225057.jpg"))).unwrap(),
            "20230301_225057",
            &material_bind_group_layout,
        );
        let (bulk_image_size, image_mesh) = create_quad_mesh(&material_image_input, &gpu_context);
        gpu_image_processing(
            &gpu_context,
            &material_bind_group_layout,
            &material_image_input,
            &image_mesh,
            bulk_image_size,
            &get_path_out_gpu("20230301_225057.png"),
        )
        .await;
    }

    {
        let material_image_input = gpu_context.create_material(
            &read(make_safe_filepath(&get_path_in("20231002_103537.jpg"))).unwrap(),
            "20231002_103537",
            &material_bind_group_layout,
        );
        let (bulk_image_size, image_mesh) = create_quad_mesh(&material_image_input, &gpu_context);
        gpu_image_processing(
            &gpu_context,
            &material_bind_group_layout,
            &material_image_input,
            &image_mesh,
            bulk_image_size,
            &get_path_out_gpu("20231002_103537_1.png"),
        )
        .await;
        gpu_image_processing(
            &gpu_context,
            &material_bind_group_layout,
            &material_image_input,
            &image_mesh,
            bulk_image_size,
            &get_path_out_gpu("20231002_103537_2.png"),
        )
        .await;
        gpu_image_processing(
            &gpu_context,
            &material_bind_group_layout,
            &material_image_input,
            &image_mesh,
            bulk_image_size,
            &get_path_out_gpu("20231002_103537_3.png"),
        )
        .await;
    }

    {
        let material_image_input = gpu_context.create_material(
            &read(make_safe_filepath(&get_path_in("20240714_1958.png"))).unwrap(),
            "20240714_1958",
            &material_bind_group_layout,
        );
        let (bulk_image_size, image_mesh) = create_quad_mesh(&material_image_input, &gpu_context);
        gpu_image_processing(
            &gpu_context,
            &material_bind_group_layout,
            &material_image_input,
            &image_mesh,
            bulk_image_size,
            &get_path_out_gpu("20240714_1958.png"),
        )
        .await;
    }

    {
        let material_image_input = gpu_context.create_material(
            &read(make_safe_filepath(&get_path_in("IMG20241009161110.jpg"))).unwrap(),
            "20241009161110",
            &material_bind_group_layout,
        );
        let (bulk_image_size, image_mesh) = create_quad_mesh(&material_image_input, &gpu_context);
        gpu_image_processing(
            &gpu_context,
            &material_bind_group_layout,
            &material_image_input,
            &image_mesh,
            bulk_image_size,
            &get_path_out_gpu("20241009_161110.png"),
        )
        .await;
    }

    /*create_and_save_int_diff_image_from_paths(
        //
        GPU_INPUT_FILENAME,
        GPU_FINAL_FILENAME,
        GPU_DIFF_FILENAME,
    );*/
}

fn create_quad_mesh(image_material: &Material, gpu_context: &GpuContext) -> (u32, Mesh) {
    let block_size = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let max_size_width_height = image_material.width.max(image_material.height);
    let bulk_image_size =
        max_size_width_height + (block_size - (max_size_width_height % block_size)) % block_size;
    let ox = image_material.width as f32 / bulk_image_size as f32 * 2.0;
    let oy = image_material.height as f32 / bulk_image_size as f32 * 2.0;
    let image_mesh = gpu_context
        .create_shape_builder()
        .use_custom_rect(
            Vec2::new(-1.0, 1.0),           // Top-left
            Vec2::new(-1.0 + ox, 1.0),      // Top-right
            Vec2::new(-1.0 + ox, 1.0 - oy), // Bottom-right
            Vec2::new(-1.0, 1.0 - oy),      // Bottom-left
        )
        .build();
    (bulk_image_size, image_mesh)
}
