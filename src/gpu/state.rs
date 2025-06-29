use crate::gpu::win_state::WinState;
use glfw::PRenderContext;
use wgpu::{
    Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, PowerPreference, Queue,
    RequestAdapterOptionsBase, TextureFormat,
};

pub const USED_PIXEL_FORMAT: TextureFormat = TextureFormat::Rgba8UnormSrgb;

pub struct State<'a> {
    instance: Instance,
    pub device: Device,
    pub queue: Queue,
    pub win_state: WinState<'a>,
}
impl<'a> State<'a> {
    pub async fn new(
        win_width: u32,
        win_height: u32,
        win_render_context: &'a PRenderContext,
    ) -> Self {
        let instance = Instance::new(&InstanceDescriptor::default());

        let win_surface = WinState::init_win_state(&instance, win_render_context);

        let adapter = instance
            .request_adapter(&RequestAdapterOptionsBase {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&win_surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let device_descriptor = DeviceDescriptor {
            label: Some("Device"),
            required_features: Features::empty(),
            ..Default::default()
        };
        let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();

        let win_state = WinState::new(&adapter, &device, win_surface, win_width, win_height);

        Self {
            instance,
            device,
            queue,
            win_state,
        }
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        self.win_state.resize(&self.device, new_size);
    }

    pub fn update_surface(&mut self, render_context: &'a PRenderContext) {
        self.win_state
            .update_surface(&self.instance, render_context);
    }
}
