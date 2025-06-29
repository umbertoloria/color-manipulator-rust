use wgpu::{
    Adapter, Device, DeviceDescriptor, Features, Instance, PowerPreference, Queue,
    RequestAdapterOptionsBase, Surface, TextureFormat,
};

pub const USED_PIXEL_FORMAT: TextureFormat = TextureFormat::Rgba8UnormSrgb;

pub struct State {
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}
impl State {
    pub async fn new(instance: &Instance, compatible_surface: Option<&Surface<'_>>) -> Self {
        let adapter = instance
            .request_adapter(&RequestAdapterOptionsBase {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface,
            })
            .await
            .unwrap();
        let device_descriptor = DeviceDescriptor {
            label: Some("Device"),
            required_features: Features::empty(),
            ..Default::default()
        };
        let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();
        Self {
            adapter,
            device,
            queue,
        }
    }
}
