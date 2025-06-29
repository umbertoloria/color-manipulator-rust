use crate::gpu::win_state::WinState;
use glfw::PRenderContext;
use wgpu::{Adapter, Device, DeviceDescriptor, Features, Instance, Queue, Surface, TextureFormat};

pub const USED_PIXEL_FORMAT: TextureFormat = TextureFormat::Rgba8UnormSrgb;

pub struct State<'a> {
    pub device: Device,
    pub queue: Queue,
    pub win_state: WinState<'a>,
}
impl<'a> State<'a> {
    pub async fn new(
        adapter: &Adapter,
        win_surface: Surface<'a>,
        win_width: u32,
        win_height: u32,
    ) -> Self {
        let device_descriptor = DeviceDescriptor {
            label: Some("Device"),
            required_features: Features::empty(),
            ..Default::default()
        };
        let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();

        let win_state = WinState::new(&adapter, &device, win_surface, win_width, win_height);

        Self {
            device,
            queue,
            win_state,
        }
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        self.win_state.resize(&self.device, new_size);
    }

    pub fn update_surface(&mut self, instance: &Instance, render_context: &'a PRenderContext) {
        self.win_state.update_surface(instance, render_context);
    }
}
