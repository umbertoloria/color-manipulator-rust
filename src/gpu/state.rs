use wgpu::{Adapter, Device, DeviceDescriptor, Features, Queue, TextureFormat};

pub const USED_PIXEL_FORMAT: TextureFormat = TextureFormat::Rgba8UnormSrgb;

pub struct State {
    pub device: Device,
    pub queue: Queue,
}
impl State {
    pub async fn new(adapter: &Adapter) -> Self {
        let device_descriptor = DeviceDescriptor {
            label: Some("Device"),
            required_features: Features::empty(),
            ..Default::default()
        };
        let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();
        Self {
            //
            device,
            queue,
        }
    }
}
