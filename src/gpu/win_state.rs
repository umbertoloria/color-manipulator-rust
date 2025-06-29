use crate::gpu::state::USED_PIXEL_FORMAT;
use crate::gpu::window::Window;
use glfw::PRenderContext;
use wgpu::{Adapter, Device, Instance, Surface, SurfaceConfiguration, TextureUsages};

pub struct WinState<'a> {
    pub surface: Surface<'a>,
    pub surface_config: SurfaceConfiguration,
    pub curr_size: (u32, u32),
}
impl<'a> WinState<'a> {
    pub fn new(
        adapter: &Adapter,
        device: &Device,
        surface: Surface<'a>,
        width: u32,
        height: u32,
    ) -> Self {
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            // Don't do "format: surface_format" since it picks "Bgra8UnormSrgb" (at least on my PC)
            // but it's the wrong one.
            format: USED_PIXEL_FORMAT, // Right one.
            width,
            height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: Vec::new(),
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);
        Self {
            surface,
            surface_config,
            curr_size: (width, height),
        }
    }
    pub fn resize(&mut self, device: &Device, new_size: (u32, u32)) {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.curr_size = new_size;
            self.surface_config.width = new_size.0;
            self.surface_config.height = new_size.1;
            self.surface.configure(&device, &self.surface_config);
        }
    }
    pub fn update_surface(&mut self, instance: &Instance, render_context: &'a PRenderContext) {
        self.surface = Window::create_glfw_surface(&instance, render_context);
    }
}
