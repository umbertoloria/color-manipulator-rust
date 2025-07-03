use crate::gpu::gpu_context::GpuContext;
use crate::gpu::renderer_backend::material::Material;
use crate::gpu::renderer_backend::mesh_builder::Mesh;
use crate::gpu::wgpu::USED_PIXEL_FORMAT;
use image::{ImageBuffer, ImageFormat, ImageReader, Rgba};
use std::error::Error;
use std::fs::remove_file;
use std::path::Path;
use wgpu::wgt::TextureViewDescriptor;
use wgpu::{
    BufferAddress, BufferDescriptor, BufferUsages, Color, CommandEncoderDescriptor, Extent3d,
    IndexFormat, LoadOp, MapMode, Operations, Origin3d, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, StoreOp, TexelCopyBufferInfo, TexelCopyBufferLayout,
    TexelCopyTextureInfo, TextureAspect, TextureDescriptor, TextureDimension, TextureUsages,
};

pub const U32_SIZE: u32 = size_of::<u32>() as u32;
pub async fn render_full(
    gpu_context: &GpuContext,
    render_pipeline: &RenderPipeline,
    material_image_input: &Material<'_>,
    image_mesh: &Mesh,
    bulk_image_size_width: u32,
    bulk_image_size_height: u32,
    image_bulk_filepath: &str,
) {
    // RENDER
    // Render (1)
    /*
    // Texture View: render on Window.
    let drawable = surface.get_current_texture().unwrap();
    let texture_view = drawable
        .texture
        .create_view(&TextureViewDescriptor::default());
    */

    // Output Buffer
    let output_buffer = gpu_context.create_buffer(&BufferDescriptor {
        label: None,
        size: (U32_SIZE * bulk_image_size_width * bulk_image_size_height) as BufferAddress,
        usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

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
    {
        let buffer_slice = output_buffer.slice(..);

        // NOTE: We have to create the mapping THEN device.poll() before await
        // the future. Otherwise, the application will freeze.
        let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        gpu_context.poll_activities_waiting();
        rx.receive().await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();

        let buffer =
            ImageBuffer::<Rgba<u8>, _>::from_raw(bulk_image_size_width, bulk_image_size_height, data)
                .unwrap();
        buffer.save(image_bulk_filepath).unwrap();
    }
    output_buffer.unmap();

    // Draw on a Window.
    /*
    drawable.present();
    */
}

pub fn save_image_output_and_remove_image_bulk(
    image_bulk_path: &Path,
    image_output_path: &Path,
    crop_x_right: u32,
    crop_y_bottom: u32,
) -> Result<(), Box<dyn Error>> {
    {
        let image_bulk = ImageReader::open(image_bulk_path)?.decode()?;
        let image_output = image_bulk.crop_imm(0, 0, crop_x_right, crop_y_bottom);
        image_output.save_with_format(image_output_path, ImageFormat::Png)?;

        let output_path_str = image_output_path.to_str().unwrap();
        println!("Painting file \"{}\"", output_path_str);
    }

    remove_file(Path::new(image_bulk_path))?;

    Ok(())
}
