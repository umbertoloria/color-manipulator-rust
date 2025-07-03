use crate::gpu::gpu_context::{create_gpu_context, GpuContext};
use crate::gpu::renderer_backend::mesh_builder::{Mesh, Vertex};
use crate::gpu::wgpu::USED_PIXEL_FORMAT;
use glm::Vec2;
use image::{ImageBuffer, ImageFormat, ImageReader, Rgba, RgbaImage};
use std::error::Error;
use std::fs::remove_file;
use std::path::Path;
use std::time::{Duration, Instant};
use wgpu::wgt::TextureViewDescriptor;
use wgpu::{
    Buffer, BufferAddress, BufferDescriptor, BufferUsages, Color, CommandEncoderDescriptor,
    Extent3d, IndexFormat, LoadOp, MapMode, Operations, Origin3d, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, TexelCopyBufferInfo, TexelCopyBufferLayout,
    TexelCopyTextureInfo, TextureAspect, TextureDescriptor, TextureDimension, TextureUsages,
};

pub struct ImageProcessingRequest {
    pub image: RgbaImage,
    pub image_output_filepath: String,
}
pub struct ImageProcessingResults {
    pub frames: usize,
    pub avg_fps: usize,
    pub duration: Duration, // excluded GPU setup
}
const U32_SIZE: u32 = size_of::<u32>() as u32;
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

    // Texture View: render on Image.
    let texture = gpu_context.create_texture(&TextureDescriptor {
        label: Some("Output texture"),
        size: Extent3d {
            width: bulk_image_size_width,
            height: bulk_image_size_height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: USED_PIXEL_FORMAT,
        usage: TextureUsages::COPY_SRC | TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[USED_PIXEL_FORMAT],
    });
    let texture_view = texture.create_view(&TextureViewDescriptor::default());

    let mut deferred_resize_bulk_file_list = Vec::new();

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

        // RENDER
        // Render (1)
        /*
        // Texture View: render on Window.
        let drawable = surface.get_current_texture().unwrap();
        let texture_view = drawable
            .texture
            .create_view(&TextureViewDescriptor::default());
        */

        // Render (2)
        // Command Encoder
        let c_e_descriptor = CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };
        let mut command_encoder = gpu_context.create_command_encoder(&c_e_descriptor);
        {
            let render_pass_color_attachment = RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.0,
                    }),
                    store: StoreOp::Store,
                },
            };
            let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(render_pass_color_attachment)],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            render_pass.set_pipeline(&render_pipeline);

            render_pass.set_bind_group(0, &material_image_input.bind_group, &[]);
            render_pass.set_vertex_buffer(0, image_mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(image_mesh.index_buffer.slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..image_mesh.index_buffer_len, 0, 0..1);
        }

        // Output Buffer
        let output_buffer = gpu_context.create_buffer(&BufferDescriptor {
            label: None,
            size: (U32_SIZE * bulk_image_size_width * bulk_image_size_height) as BufferAddress,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Render on a Texture.
        command_encoder.copy_texture_to_buffer(
            TexelCopyTextureInfo {
                aspect: TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
            },
            TexelCopyBufferInfo {
                buffer: &output_buffer,
                layout: TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(U32_SIZE * bulk_image_size_width),
                    rows_per_image: Some(bulk_image_size_height),
                },
            },
            Extent3d {
                width: bulk_image_size_width,
                height: bulk_image_size_height,
                depth_or_array_layers: 1,
            },
        );
        gpu_context.submit_to_queue(command_encoder.finish());

        // Render (3)

        // Save Texture on an Image.
        let image_bulk_filepath = format!("{}_bulk.png", request.image_output_filepath);
        let deferred_output_buffer = DeferredOutputBuffer {
            output_buffer,
            bulk_image_size_width,
            bulk_image_size_height,
            image_bulk_filepath,
        };
        deferred_output_buffer.write_on_file(&gpu_context).await;

        // Draw on a Window.
        /*
        drawable.present();
        */

        // Outside GPU scope
        deferred_resize_bulk_file_list.push(DeferredResizeBulkFile {
            image_bulk_filepath: deferred_output_buffer.image_bulk_filepath,
            image_output_filepath: request.image_output_filepath.clone(),
        });
    }

    // Benchmark
    let after = Instant::now();
    let duration = after - before;

    // Resizing bulk images
    println!("Resizing bulk images");
    for item in deferred_resize_bulk_file_list {
        item.save_image_output_and_remove_image_bulk(reference_image_width, reference_image_height)
            .unwrap();
    }

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

struct DeferredOutputBuffer {
    output_buffer: Buffer,
    bulk_image_size_width: u32,
    bulk_image_size_height: u32,
    image_bulk_filepath: String,
}
impl DeferredOutputBuffer {
    pub async fn write_on_file(&self, gpu_context: &GpuContext) {
        println!("Painting file \"{}\"", self.image_bulk_filepath);
        {
            let buffer_slice = self.output_buffer.slice(..);

            // NOTE: We have to create the mapping THEN device.poll() before await
            // the future. Otherwise, the application will freeze.
            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
            gpu_context.poll_activities_waiting();
            rx.receive().await.unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(
                self.bulk_image_size_width,
                self.bulk_image_size_height,
                data,
            )
            .unwrap();
            buffer.save(&self.image_bulk_filepath).unwrap();
        }
        self.output_buffer.unmap();
    }
}

struct DeferredResizeBulkFile {
    image_bulk_filepath: String,
    image_output_filepath: String,
}
impl DeferredResizeBulkFile {
    fn save_image_output_and_remove_image_bulk(
        &self,
        crop_x_right: u32,
        crop_y_bottom: u32,
    ) -> Result<(), Box<dyn Error>> {
        let image_bulk_path = Path::new(&self.image_bulk_filepath);
        let image_output_path = Path::new(&self.image_output_filepath);
        {
            let image_bulk = ImageReader::open(image_bulk_path)?.decode()?;
            let image_output = image_bulk.crop_imm(0, 0, crop_x_right, crop_y_bottom);
            image_output.save_with_format(image_output_path, ImageFormat::Png)?;

            let output_path_str = image_output_path.to_str().unwrap();
            println!("Resizing file \"{}\"", output_path_str);
        }

        remove_file(Path::new(image_bulk_path))?;

        Ok(())
    }
}
