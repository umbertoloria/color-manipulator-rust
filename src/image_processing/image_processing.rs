use crate::gpu::gpu_context::{create_gpu_context, GpuContext};
use crate::gpu::gpu_image_processing::gpu_image_processing;
use crate::gpu::renderer_backend::material::Material;
use crate::gpu::renderer_backend::mesh_builder::Mesh;
use glm::Vec2;
use image::RgbaImage;
use std::time::{Duration, Instant};

pub struct ImageProcessingRequest {
    pub image: RgbaImage,
    pub image_output_filepath: String,
}
pub struct ImageProcessingResults {
    pub frames: usize,
    pub avg_fps: usize,
    pub duration: Duration, // excluded GPU setup
}
pub async fn image_processing_compute(
    requests: &Vec<ImageProcessingRequest>,
) -> ImageProcessingResults {
    // GPU SETUP
    let gpu_context = create_gpu_context().await;

    // Benchmark
    let before = Instant::now();

    // Bind Group Layout: Texture Material
    let material_bind_group_layout = gpu_context
        .create_bind_group_layout_builder()
        .add_material()
        .build("Material Bind Group Layout");

    // Frames: compute
    for request in requests {
        // Frame as Texture
        let material_image_input = gpu_context.create_material(
            //
            &request.image,
            "Frame image",
            &material_bind_group_layout,
        );
        let (bulk_image_size, image_mesh) = create_quad_mesh(&material_image_input, &gpu_context);

        // GPU Image Processing
        gpu_image_processing(
            &gpu_context,
            &material_bind_group_layout,
            &material_image_input,
            &image_mesh,
            bulk_image_size,
            &request.image_output_filepath,
        )
        .await;
    }

    // Benchmark
    let after = Instant::now();
    let duration = after - before;

    // RESULTS
    let avg_fps = (requests.len() as f32 / duration.as_secs_f32()) as usize;
    ImageProcessingResults {
        frames: requests.len(),
        avg_fps,
        duration,
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
