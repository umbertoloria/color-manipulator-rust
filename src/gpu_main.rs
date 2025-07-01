use crate::gpu::gpu_context::{create_gpu_context, GpuContext};
use crate::gpu::gpu_image_processing::gpu_image_processing;
use crate::gpu::renderer_backend::material::Material;
use crate::gpu::renderer_backend::mesh_builder::Mesh;
use glm::Vec2;
use std::fs::{read, read_dir};
use std::path::{Path, PathBuf};

pub async fn gpu_main(
    //
    input_frames_dir: &str,
    output_frames_dir: &str,
) {
    // Frames loading
    let frame_files = get_png_files_in_folder(input_frames_dir).unwrap();
    println!("Found {} frames", frame_files.len());
    let mut files = Vec::new();
    for frame_file in &frame_files {
        let frame_file_path = frame_file.as_path();
        let frame_file_name = frame_file_path.file_name().unwrap().to_str().unwrap();
        let frame_file_bytes = read(frame_file_path).unwrap();
        files.push((
            //
            frame_file_path,
            frame_file_name,
            frame_file_bytes,
        ));
    }

    // GPU SETUP
    let gpu_context = create_gpu_context().await;

    // Bind Group Layout: Texture Material
    let material_bind_group_layout = gpu_context
        .create_bind_group_layout_builder()
        .add_material()
        .build("Material Bind Group Layout");

    // Frames: compute
    for (_, frame_file_name, frame_file_bytes) in &files {
        // Frame as Texture
        let material_image_input =
            gpu_context.create_material(&frame_file_bytes, "Frame", &material_bind_group_layout);
        let (bulk_image_size, image_mesh) = create_quad_mesh(&material_image_input, &gpu_context);

        // Output frame file path
        let image_output_filepath = &format!("{}/{}", output_frames_dir, frame_file_name);

        // GPU Image Processing
        gpu_image_processing(
            &gpu_context,
            &material_bind_group_layout,
            &material_image_input,
            &image_mesh,
            bulk_image_size,
            image_output_filepath,
        )
        .await;
    }

    /*
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
    */

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

pub fn get_png_files_in_folder(folder_path: &str) -> std::io::Result<Vec<PathBuf>> {
    let mut png_files = Vec::new();
    let path = Path::new(folder_path);

    // Check if the path exists and is a directory
    if !path.exists() || !path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Folder not found or is not a directory: {}", folder_path),
        ));
    }

    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        // Check if it's a file and has a .png extension
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "png" {
                    png_files.push(path);
                }
            }
        }
    }
    Ok(png_files)
}
