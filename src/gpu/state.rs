use glfw::PRenderContext;
use wgpu::{
    Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, PowerPreference,
    Queue, RequestAdapterOptionsBase, Surface, SurfaceConfiguration, TextureFormat, TextureUsages,
};

pub struct State<'a> {
    instance: Instance,
    pub device: Device, // Abstract GPU
    pub queue: Queue,   // For submitting works
    surface: Surface<'a>,
    pub surface_config: SurfaceConfiguration,
    pub curr_win_size: (u32, u32),
}
impl<'a> State<'a> {
    pub async fn new(
        framebuffer_width: u32,
        framebuffer_height: u32,
        render_context: &'a PRenderContext,
    ) -> Self {
        let instance = Instance::new(&InstanceDescriptor::default());
        let surface = Self::create_wgpu_surface(&instance, render_context);

        let adapter = instance
            .request_adapter(&RequestAdapterOptionsBase {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
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

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            // Don't do "format: surface_format" since it picks "Bgra8UnormSrgb" (at least on my PC)
            // but it's the wrong one.
            format: TextureFormat::Rgba8UnormSrgb, // Right one.
            width: framebuffer_width,
            height: framebuffer_height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: Vec::new(),
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        Self {
            instance,
            surface,
            device,
            queue,
            surface_config,
            curr_win_size: (framebuffer_width, framebuffer_height),
        }
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.curr_win_size = new_size;
            self.surface_config.width = new_size.0;
            self.surface_config.height = new_size.1;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub fn update_surface(&mut self, render_context: &'a PRenderContext) {
        self.surface = Self::create_wgpu_surface(&self.instance, render_context);
    }

    fn create_wgpu_surface(instance: &Instance, render_context: &'a PRenderContext) -> Surface<'a> {
        instance.create_surface(render_context).unwrap()
    }
}
