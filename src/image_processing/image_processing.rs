use crate::gpu::gpu_context::{create_gpu_context, GpuContext};
use crate::gpu::gpu_image_processing::{render_full, save_image_output_and_remove_image_bulk};
use crate::gpu::renderer_backend::mesh_builder::{Mesh, Vertex};
use glm::Vec2;
use image::RgbaImage;
use std::path::Path;
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
    // NOTE: Assuming all images from "requests" have the same size.

    // GPU SETUP
    let gpu_context = create_gpu_context().await;

    // Benchmark
    let before = Instant::now();

    // GPU COMPUTE
    let material_bind_group_layout = gpu_context
        .create_bind_group_layout_builder()
        .add_material()
        .build("Material Bind Group Layout");
    let render_pipeline = gpu_context
        .create_render_pipeline_builder()
        .set_shader_module("src/gpu/shaders/shader.wgsl", "vs_main", "fs_main")
        .add_vertex_buffer_layout(Vertex::get_layout())
        .add_bind_group_layout(&material_bind_group_layout)
        .build("Render Pipeline");

    let reference_image = &requests[0].image;
    let reference_image_width = reference_image.width();
    let reference_image_height = reference_image.height();

    let sampler = gpu_context.create_sampler();
    let mut material_image_input = gpu_context.create_material(
        reference_image,
        "Frame image",
        &material_bind_group_layout,
        &sampler,
    );
    let (bulk_image_size, image_mesh) = create_quad_mesh_with_bulk_dimensions(
        reference_image_width,
        reference_image_height,
        &gpu_context,
    );
    let bulk_image_size_width = bulk_image_size;
    let bulk_image_size_height = bulk_image_size;
    for request in requests {
        material_image_input.change_image(&request.image).unwrap();

        /*
        // Render Loop
        glfw_wrapper.enable_events_polling();
        while !glfw_wrapper.should_close() {
            glfw_wrapper.dispatch_events(&instance, &wgpu_wrapper.device, &glfw_render_context);

            // Render here...

            break;
        }
        */

        // GPU Image Processing
        let image_bulk_filepath = &format!("{}_bulk.png", request.image_output_filepath);
        render_full(
            &gpu_context,
            &render_pipeline,
            &material_image_input,
            &image_mesh,
            bulk_image_size_width,
            bulk_image_size_height,
            image_bulk_filepath,
        )
        .await;

        // Outside GPU scope
        save_image_output_and_remove_image_bulk(
            Path::new(image_bulk_filepath),
            Path::new(&request.image_output_filepath),
            reference_image_width,
            reference_image_height,
        )
        .unwrap();
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

fn create_quad_mesh_with_bulk_dimensions(
    width: u32,
    height: u32,
    gpu_context: &GpuContext,
) -> (u32, Mesh) {
    let block_size = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;

    let max_size_width_height = width.max(height);
    let bulk_image_size =
        max_size_width_height + (block_size - (max_size_width_height % block_size)) % block_size;
    let ox = width as f32 / bulk_image_size as f32 * 2.0;
    let oy = height as f32 / bulk_image_size as f32 * 2.0;

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
