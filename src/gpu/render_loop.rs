use crate::gpu::renderer_backend::bind_group_layout::BindGroupLayoutBuilder;
use crate::gpu::renderer_backend::material::{calculate_ratio, Material};
use crate::gpu::renderer_backend::mesh_builder::{make_rect, Vertex};
use crate::gpu::renderer_backend::pipeline::PipelineBuilder;
use crate::gpu::state::{State, USED_PIXEL_FORMAT};
use crate::gpu::win_state::WinState;
use crate::gpu::window::Window;
use glfw::{flush_messages, Action, Key, WindowEvent};
use image::{ImageBuffer, Rgba};
use wgpu::wgt::TextureViewDescriptor;
use wgpu::{
    Buffer, BufferAddress, BufferDescriptor, BufferUsages, Color, CommandEncoderDescriptor, Device,
    Extent3d, IndexFormat, Instance, InstanceDescriptor, LoadOp, MapMode, Operations, Origin3d,
    PollType, PowerPreference, RenderPassColorAttachment, RenderPassDescriptor,
    RequestAdapterOptionsBase, StoreOp, TexelCopyBufferInfo, TexelCopyBufferLayout,
    TexelCopyTextureInfo, Texture, TextureAspect, TextureDescriptor, TextureDimension,
    TextureFormat, TextureUsages, TextureView,
};

pub enum RenderResult {
    GoNextTick,
    StopRendering,
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
        format: TextureFormat::Rgba8UnormSrgb,
        usage: TextureUsages::COPY_SRC | TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[TextureFormat::Rgba8UnormSrgb],
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
) -> RenderResult {
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
        buffer.save("image.png").unwrap();
    }
    output_buffer.unmap();

    // Draw on a Window.
    /*
    drawable.present();
    RenderResult::GoNextTick
    */

    RenderResult::StopRendering
}

pub async fn gpu_main() {
    // Window
    let mut window = Window::new();
    let (width, height) = window.get_framebuffer_size();
    let render_context = window.get_render_context();

    // WGPU
    let instance = Instance::new(&InstanceDescriptor::default());

    // Window
    let win_surface = WinState::init_win_state(&instance, &render_context);

    // WGPU
    let adapter = instance
        .request_adapter(&RequestAdapterOptionsBase {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&win_surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    // State
    let state = State::new(&adapter).await;

    // WinState
    let mut win_state = WinState::new(&adapter, &state.device, win_surface, width, height);

    // Setup
    let material_bind_group_layout = BindGroupLayoutBuilder::new(&state.device)
        .add_material()
        .build("Material Bind Group Layout");
    let render_pipeline = PipelineBuilder::new(&state.device)
        .set_shader_module("src/gpu/shaders/shader.wgsl", "vs_main", "fs_main")
        .set_pixel_format(USED_PIXEL_FORMAT)
        .add_vertex_buffer_layout(Vertex::get_layout())
        .add_bind_group_layout(&material_bind_group_layout)
        .build("Render Pipeline");
    let quad_material = Material::new(
        "input/20230301_224920.jpg",
        &state.device,
        &state.queue,
        "Quad Material",
        &material_bind_group_layout,
    );
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

    let quad_mesh = make_rect(quad_texture_ratio, &state.device);

    // Render Loop
    window.prepare_events();
    while !window.window.should_close() {
        // Dispatch Events
        window.glfw.poll_events();
        for (_, event) in flush_messages(&window.events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.window.set_should_close(true);
                }

                WindowEvent::FramebufferSize(width, height) => {
                    win_state.update_surface(&instance, &render_context);
                    win_state.resize(&state.device, (width as u32, height as u32));
                }

                WindowEvent::Pos(..) => {
                    // Workaround for Window Move.
                    win_state.update_surface(&instance, &render_context);
                    win_state.resize(&state.device, win_state.curr_size);
                }

                _ => {
                    // println!("{:?}", e);
                }
            }
        }

        // Render
        let (
            //
            texture,
            texture_view,
            output_buffer,
        ) = render_start(&state.device, texture_full_width, texture_full_height);

        // Command Encoder
        let c_e_descriptor = CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };
        let mut command_encoder = state.device.create_command_encoder(&c_e_descriptor);
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
        state.queue.submit(Some(command_encoder.finish()));

        let render_result = render_finish(
            &state.device,
            &output_buffer,
            texture_full_width,
            texture_full_height,
        )
        .await;

        match render_result {
            RenderResult::GoNextTick => {}
            RenderResult::StopRendering => {
                break;
            }
        };
    }
}
