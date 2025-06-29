use glfw::PRenderContext;
use wgpu::{
    Backends, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, PowerPreference,
    Queue, RequestAdapterOptionsBase, Surface, SurfaceConfiguration, TextureFormat, TextureUsages,
};

pub struct State<'a> {
    instance: Instance,
    surface: Surface<'a>,
    pub device: Device, // Abstract GPU
    pub queue: Queue,   // For submitting works
    pub config: SurfaceConfiguration,
    pub size: (u32, u32),
}
impl<'a> State<'a> {
    pub async fn new(
        framebuffer_width: u32,
        framebuffer_height: u32,
        render_context: &'a PRenderContext,
    ) -> Self {
        let instance_descriptor = InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        };
        let instance = Instance::new(&instance_descriptor);
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
        /*
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_capabilities.formats[0]);
        for surface_format in surface_capabilities.formats {
            println!(" -> {:?}", surface_format);
        }
        println!(" PICKED -> {:?}", surface_format);
        */
        let config = SurfaceConfiguration {
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
        surface.configure(&device, &config);

        Self {
            instance,
            surface,
            device,
            queue,
            config,
            size: (framebuffer_width, framebuffer_height),
        }
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0;
            self.config.height = new_size.1;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn update_surface(&mut self, render_context: &'a PRenderContext) {
        self.surface = Self::create_wgpu_surface(&self.instance, render_context);
    }

    fn create_wgpu_surface(instance: &Instance, render_context: &'a PRenderContext) -> Surface<'a> {
        instance.create_surface(render_context).unwrap()
    }
}
