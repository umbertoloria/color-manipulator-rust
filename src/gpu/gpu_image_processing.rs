use crate::gpu::gpu_context::{create_gpu_context, GpuContext};
use crate::gpu::renderer_backend::material::Material;
use crate::gpu::renderer_backend::mesh_builder::{make_custom_rect, Mesh, Vertex};
use crate::gpu::wgpu::USED_PIXEL_FORMAT;
use glm::Vec2;
use image::{ImageBuffer, ImageFormat, ImageReader, Rgba};
use std::error::Error;
use std::fs::remove_file;
use std::path::Path;
use wgpu::wgt::TextureViewDescriptor;
use wgpu::{
    Buffer, BufferAddress, BufferDescriptor, BufferUsages, Color, CommandEncoderDescriptor, Device,
    Extent3d, IndexFormat, LoadOp, MapMode, Operations, Origin3d, PollType,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, StoreOp, TexelCopyBufferInfo,
    TexelCopyBufferLayout, TexelCopyTextureInfo, Texture, TextureAspect, TextureDescriptor,
    TextureDimension, TextureUsages, TextureView,
};

pub async fn gpu_image_processing(
    //
    image_input_filename: &str,
    image_output_filename: &str,
) {
    let gpu_context = create_gpu_context().await;

    // SETUP

    // Bind Group Layout: Texture Material
    let material_bind_group_layout = gpu_context
        .create_bind_group_layout()
        .add_material()
        .build("Material Bind Group Layout");

    // Material: Image (as Texture)
    let image_material = gpu_context.create_material(
        image_input_filename,
        "Image Texture Material",
        &material_bind_group_layout,
    );

    // Render Pipeline
    let render_pipeline = gpu_context
        .create_render_pipeline_builder()
        .set_shader_module("src/gpu/shaders/shader.wgsl", "vs_main", "fs_main")
        .add_vertex_buffer_layout(Vertex::get_layout())
        .add_bind_group_layout(&material_bind_group_layout)
        .build("Render Pipeline");

    // Quad Mesh
    let (
        //
        bulk_image_size,
        image_mesh,
    ) = {
        let block_size = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let max_size_width_height = image_material.width.max(image_material.height);
        let bulk_image_size = max_size_width_height
            + (block_size - (max_size_width_height % block_size)) % block_size;

        // Offsets: to right and to bottom
        let ox = image_material.width as f32 / bulk_image_size as f32 * 2.0;
        let oy = image_material.height as f32 / bulk_image_size as f32 * 2.0;

        let image_mesh = make_custom_rect(
            /*
            // For perfect quad (stretched).
            Vec2::new(-1.0, 1.0),  // Top-left
            Vec2::new(1.0, 1.0),   // Top-right
            Vec2::new(1.0, -1.0),  // Bottom-right
            Vec2::new(-1.0, -1.0), // Bottom-left
            */
            // For actual sizes (proportional).
            Vec2::new(-1.0, 1.0),           // Top-left
            Vec2::new(-1.0 + ox, 1.0),      // Top-right
            Vec2::new(-1.0 + ox, 1.0 - oy), // Bottom-right
            Vec2::new(-1.0, 1.0 - oy),      // Bottom-left
            &gpu_context.wgpu_wrapper.device,
        );
        (
            //
            bulk_image_size,
            image_mesh,
        )
    };

    /*
    // Render Loop
    glfw_wrapper.enable_events_polling();
    while !glfw_wrapper.should_close() {
        glfw_wrapper.dispatch_events(&instance, &wgpu_wrapper.device, &glfw_render_context);

        // Render here...

        break;
    }
    */

    let image_bulk_filename = &format!("{}_middle.png", image_output_filename);
    render_full(
        gpu_context,
        &render_pipeline,
        &image_material,
        image_mesh,
        bulk_image_size,
        bulk_image_size,
        image_bulk_filename,
    )
    .await;

    // Outside GPU scope
    resize_and_save_image_truncated(
        Path::new(image_bulk_filename),
        Path::new(image_output_filename),
        image_material.width,
        image_material.height,
    )
    .unwrap();
}

async fn render_full(
    gpu_context: GpuContext,
    render_pipeline: &RenderPipeline,
    quad_material: &Material,
    quad_mesh: Mesh,
    texture_full_width: u32,
    texture_full_height: u32,
    middle_filename: &str,
) {
    let wgpu_wrapper = &gpu_context.wgpu_wrapper;

    // Render
    let (
        //
        texture,
        texture_view,
        output_buffer,
    ) = render_start(
        &wgpu_wrapper.device,
        texture_full_width,
        texture_full_height,
    );

    // Command Encoder
    let c_e_descriptor = CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    };
    let mut command_encoder = wgpu_wrapper.device.create_command_encoder(&c_e_descriptor);
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

        render_pass.set_bind_group(0, &quad_material.bind_group, &[]);
        render_pass.set_vertex_buffer(0, quad_mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(quad_mesh.index_buffer.slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..quad_mesh.index_buffer_len, 0, 0..1);
    }

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
                bytes_per_row: Some(U32_SIZE * texture_full_width),
                rows_per_image: Some(texture_full_height),
            },
        },
        Extent3d {
            width: texture_full_width,
            height: texture_full_height,
            depth_or_array_layers: 1,
        },
    );
    wgpu_wrapper.queue.submit(Some(command_encoder.finish()));

    render_finish(
        &wgpu_wrapper.device,
        &output_buffer,
        texture_full_width,
        texture_full_height,
        middle_filename,
    )
    .await;
}
pub const U32_SIZE: u32 = size_of::<u32>() as u32;
pub fn render_start(
    device: &Device,
    full_width: u32,
    full_height: u32,
) -> (
    //
    Texture,
    TextureView,
    Buffer,
) {
    /*
    // Texture View: render on Window.
    let drawable = surface.get_current_texture().unwrap();
    let texture_view = drawable
        .texture
        .create_view(&TextureViewDescriptor::default());
    */

    // Texture View: render on Image.
    let texture = device.create_texture(&TextureDescriptor {
        label: Some("Output texture"),
        size: Extent3d {
            width: full_width,
            height: full_height,
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
    let output_buffer_desc = BufferDescriptor {
        label: None,
        size: (U32_SIZE * full_width * full_height) as BufferAddress,
        usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
        mapped_at_creation: false,
    };
    let output_buffer = device.create_buffer(&output_buffer_desc);

    (
        //
        texture,
        texture_view,
        output_buffer,
    )
}
pub async fn render_finish(
    device: &Device,
    output_buffer: &Buffer,
    full_width: u32,
    full_height: u32,
    middle_filename: &str,
) {
    // Save Texture on an Image.
    {
        let buffer_slice = output_buffer.slice(..);

        // NOTE: We have to create the mapping THEN device.poll() before await
        // the future. Otherwise, the application will freeze.
        let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(PollType::Wait).unwrap();
        rx.receive().await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();

        let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(full_width, full_height, data).unwrap();
        buffer.save(middle_filename).unwrap();
    }
    output_buffer.unmap();

    // Draw on a Window.
    /*
    drawable.present();
    */
}

fn resize_and_save_image_truncated(
    input_path: &Path,
    output_path: &Path,
    crop_x_right: u32,
    crop_y_bottom: u32,
) -> Result<(), Box<dyn Error>> {
    {
        let img = ImageReader::open(input_path)?.decode()?;
        let crop_img = img.crop_imm(0, 0, crop_x_right, crop_y_bottom);
        let png_format = ImageFormat::Png;
        crop_img.save_with_format(output_path, png_format)?;

        let output_path_str = output_path.to_str().unwrap();
        println!("Painting file \"{}\"", output_path_str);
    }

    remove_file(Path::new(input_path))?;

    Ok(())
}
