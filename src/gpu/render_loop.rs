use crate::gpu::renderer_backend::bind_group_layout::BindGroupLayoutBuilder;
use crate::gpu::renderer_backend::material::{calculate_ratio, Material};
use crate::gpu::renderer_backend::mesh_builder::{make_rect, Mesh, Vertex};
use crate::gpu::renderer_backend::pipeline::PipelineBuilder;
use crate::gpu::wgpu::{WGPUWrapper, USED_PIXEL_FORMAT};
use image::imageops::FilterType;
use image::{ImageBuffer, ImageFormat, ImageReader, Rgba};
use std::error::Error;
use std::path::Path;
use wgpu::wgt::TextureViewDescriptor;
use wgpu::{
    Buffer, BufferAddress, BufferDescriptor, BufferUsages, Color, CommandEncoderDescriptor, Device,
    Extent3d, IndexFormat, LoadOp, MapMode, Operations, Origin3d, PollType,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, StoreOp, TexelCopyBufferInfo,
    TexelCopyBufferLayout, TexelCopyTextureInfo, Texture, TextureAspect, TextureDescriptor,
    TextureDimension, TextureUsages, TextureView,
};

const RESULT_GPU_FILENAME: &str = "result.png";
const RESULT_FINAL_FILENAME: &str = "result.png";

pub async fn gpu_main() {
    // WGPU
    let instance = WGPUWrapper::init_instance();
    let wgpu_wrapper = WGPUWrapper::new(&instance, None).await;

    // Glfw
    /*
    const WIN_WIDTH: u32 = 900;
    const WIN_HEIGHT: u32 = 900;
    const WIN_TITLE: &str = "Window title";
    let (
        //
        mut glfw_window,
        glfw_events,
    ) = GlfwWrapper::init_window(WIN_WIDTH, WIN_HEIGHT, WIN_TITLE);
    let glfw_render_context = glfw_window.render_context();
    let window_surface = GlfwWrapper::create_glfw_surface(&instance, &glfw_render_context);
    let window_state = WindowState::new(
        &wgpu_wrapper.adapter,
        &wgpu_wrapper.device,
        glfw_window,
        window_surface,
    );
    let mut glfw_wrapper = GlfwWrapper::new(window_state, glfw_events);
    */

    // Setup
    let material_bind_group_layout = BindGroupLayoutBuilder::new(&wgpu_wrapper.device)
        .add_material()
        .build("Material Bind Group Layout");
    let render_pipeline = PipelineBuilder::new(&wgpu_wrapper.device)
        .set_shader_module("src/gpu/shaders/shader.wgsl", "vs_main", "fs_main")
        .set_pixel_format(USED_PIXEL_FORMAT)
        .add_vertex_buffer_layout(Vertex::get_layout())
        .add_bind_group_layout(&material_bind_group_layout)
        .build("Render Pipeline");
    let quad_material = Material::new(
        "input/20230301_224920.jpg",
        &wgpu_wrapper.device,
        &wgpu_wrapper.queue,
        "Quad Material",
        &material_bind_group_layout,
    );
    let image_real_ratio = quad_material.width as f32 / quad_material.height as f32;
    let block_size = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let texture_full_width: u32 =
        quad_material.width + (block_size - (quad_material.width % block_size)) % block_size;
    let texture_full_height: u32 =
        quad_material.height + (block_size - (quad_material.height % block_size)) % block_size;
    // Using quad texture.
    let texture_full_height: u32 = texture_full_width;

    let quad_texture_ratio =
        calculate_ratio(quad_material.width as f32, quad_material.height as f32)
            / calculate_ratio(texture_full_width as f32, texture_full_height as f32);

    let quad_mesh = make_rect(quad_texture_ratio, &wgpu_wrapper.device);

    /*
    // Render Loop
    glfw_wrapper.enable_events_polling();
    while !glfw_wrapper.should_close() {
        glfw_wrapper.dispatch_events(&instance, &wgpu_wrapper.device, &glfw_render_context);

        // Render here...

        break;
    }
    */

    render_full(
        &wgpu_wrapper,
        &render_pipeline,
        &quad_material,
        quad_mesh,
        texture_full_width,
        texture_full_height,
    )
    .await;

    resize_and_save_image_truncated(
        Path::new(RESULT_GPU_FILENAME),
        Path::new(RESULT_FINAL_FILENAME),
        (texture_full_width as f32 / image_real_ratio) as u32,
        quad_material.width,
        quad_material.height,
    )
    .unwrap();
}

async fn render_full(
    wgpu_wrapper: &WGPUWrapper,
    render_pipeline: &RenderPipeline,
    quad_material: &Material,
    quad_mesh: Mesh,
    texture_full_width: u32,
    texture_full_height: u32,
) {
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
        buffer.save(RESULT_GPU_FILENAME).unwrap();
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
    extended_real_height: u32,
    new_width: u32,
    new_height: u32,
) -> Result<(), Box<dyn Error>> {
    let img = ImageReader::open(input_path)?.decode()?;

    let original_width = img.width();
    let original_height = img.height();

    if new_width > original_width || new_height > original_height {
        return Err(
            format!(
                "New dimensions ({new_width}x{new_height}) cannot be larger than original ({original_width}x{original_height}) for truncation."
            )
                .into(),
        );
    }

    let crop_img = img.crop_imm(0, 0, original_width, extended_real_height);

    let resized_img = crop_img.resize(new_width, new_height, FilterType::Gaussian);

    let format = ImageFormat::from_path(output_path).unwrap_or(ImageFormat::Png);
    resized_img.save_with_format(output_path, format)?;
    Ok(())
}
