use crate::gpu::state::State;
use crate::gpu::window::Window;
use glfw::{flush_messages, Action, Key, WindowEvent};
use image::{ImageBuffer, Rgba};
use wgpu::wgt::TextureViewDescriptor;
use wgpu::{
    Buffer, BufferAddress, BufferDescriptor, BufferUsages, Device, Extent3d, MapMode, PollType,
    Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
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
) -> (Texture, TextureView, Buffer) {
    /*
    // Texture View: render on Window.
    let drawable = self.surface.get_current_texture().unwrap();
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
    Ok(RenderResult::GoNextTick)
    */

    RenderResult::StopRendering
}

pub async fn gpu_main() {
    let mut window = Window::new();

    let (width, height) = window.get_framebuffer_size();
    let render_context = window.get_render_context();
    let mut state = State::new(width, height, &render_context).await;

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
                    state.update_surface(&render_context);
                    state.resize((width as u32, height as u32));
                }

                WindowEvent::Pos(..) => {
                    // Workaround for Window Move.
                    state.update_surface(&render_context);
                    state.resize(state.size);
                }

                _ => {
                    // println!("{:?}", e);
                }
            }
        }

        // Render
        match state.render().await {
            RenderResult::GoNextTick => {}
            RenderResult::StopRendering => {
                break;
            }
        };
    }
}
