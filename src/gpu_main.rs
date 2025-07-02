use crate::gpu::gpu_context::{create_gpu_context, GpuContext};
use crate::gpu::gpu_image_processing::gpu_image_processing;
use crate::gpu::renderer_backend::material::Material;
use crate::gpu::renderer_backend::mesh_builder::Mesh;
use glm::Vec2;
use image::load_from_memory;
use std::fs::{read, read_dir};
use std::path::{Path, PathBuf};
use std::process::exit;

pub async fn gpu_main(
    //
    input_frames_dir: &str,
    output_frames_dir: &str,
) {
    // Frames loading
    let frame_paths = get_png_files_in_folder(input_frames_dir).unwrap();
    println!("Loading {} frames", frame_paths.len());
    let mut dimensions: Option<(u32, u32)> = None;
    let mut files = Vec::new();
    for frame_path_buf in &frame_paths {
        let frame_path = frame_path_buf.as_path();

        let frame_bytes = read(frame_path).unwrap();
        let frame_image = load_from_memory(&frame_bytes).unwrap().to_rgba8();
        let width = frame_image.width();
        let height = frame_image.height();

        if let Some(dimensions) = dimensions {
            if dimensions.0 != width || dimensions.1 != height {
                eprintln!("Frame images don't have the same dimensions");
                exit(0x0100);
            }
        } else {
            dimensions = Some((width, height));
        }

        let frame_file_name = frame_path.file_name().unwrap().to_str().unwrap();
        let output_frame_filepath = format!("{}/{}", output_frames_dir, frame_file_name);

        files.push((frame_image, output_frame_filepath));
    }

    // GPU SETUP
    let gpu_context = create_gpu_context().await;

    // Bind Group Layout: Texture Material
    let material_bind_group_layout = gpu_context
        .create_bind_group_layout_builder()
        .add_material()
        .build("Material Bind Group Layout");

    // Frames: compute
    for (frame_image, output_frame_filepath) in &files {
        // Frame as Texture
        let material_image_input =
            gpu_context.create_material(frame_image, "Frame image", &material_bind_group_layout);
        let (bulk_image_size, image_mesh) = create_quad_mesh(&material_image_input, &gpu_context);

        // GPU Image Processing
        gpu_image_processing(
            &gpu_context,
            &material_bind_group_layout,
            &material_image_input,
            &image_mesh,
            bulk_image_size,
            output_frame_filepath,
        )
        .await;
    }
}

fn create_quad_mesh(image_material: &Material, gpu_context: &GpuContext) -> (u32, Mesh) {
    let block_size = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let max_size_width_height = image_material.width().max(image_material.height());
    let bulk_image_size =
        max_size_width_height + (block_size - (max_size_width_height % block_size)) % block_size;
    let ox = image_material.width() as f32 / bulk_image_size as f32 * 2.0;
    let oy = image_material.height() as f32 / bulk_image_size as f32 * 2.0;
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
    let mut png_files_path_bufs = Vec::new();
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
        let path_buf = entry.path();

        // Check if it's a file and has a .png extension
        if path_buf.is_file() {
            if let Some(extension) = path_buf.extension() {
                if extension == "png" {
                    png_files_path_bufs.push(path_buf);
                }
            }
        }
    }
    Ok(png_files_path_bufs)
}
