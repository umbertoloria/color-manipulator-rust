use crate::gpu::state::USED_PIXEL_FORMAT;
use crate::gpu::window::Window;
use glfw::{flush_messages, Action, GlfwReceiver, Key, PRenderContext, PWindow, WindowEvent};
use wgpu::{Adapter, Device, Instance, Surface, SurfaceConfiguration, TextureUsages};

pub struct WinState<'a> {
    pub window: PWindow,
    // pub events: GlfwReceiver<(f64, WindowEvent)>,
    pub surface: Surface<'a>,
    pub surface_config: SurfaceConfiguration,
    pub curr_size: (u32, u32),
}
impl<'a> WinState<'a> {
    pub fn new(
        adapter: &Adapter,
        device: &Device,
        window: PWindow,
        // events: GlfwReceiver<(f64, WindowEvent)>,
        surface: Surface<'a>,
    ) -> Self {
        let window_size = window.get_framebuffer_size();
        let width = window_size.0 as u32;
        let height = window_size.1 as u32;

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
            window,
            surface,
            surface_config,
            curr_size: (width, height),
        }
    }
    pub fn should_close(&self) -> bool {
        self.window.should_close()
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
    pub fn enable_events_polling(&mut self) {
        // self.window.set_all_polling(true);
        self.window.set_key_polling(true);
        self.window.set_framebuffer_size_polling(true);
        self.window.set_pos_polling(true);
        // self.window.set_mouse_button_polling(true);
    }
    pub fn dispatch_events(
        &mut self,
        events: &GlfwReceiver<(f64, WindowEvent)>,
        instance: &Instance,
        device: &Device,
        render_context: &'a PRenderContext,
    ) {
        // Dispatch Events
        self.window.glfw.poll_events();
        let messages = flush_messages(&events);
        for (_, event) in messages {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true);
                }

                WindowEvent::FramebufferSize(width, height) => {
                    self.update_surface(&instance, &render_context);
                    self.resize(&device, (width as u32, height as u32));
                }

                WindowEvent::Pos(..) => {
                    // Workaround for Window Move.
                    self.update_surface(&instance, &render_context);
                    self.resize(&device, self.curr_size);
                }

                _ => {
                    // println!("{:?}", e);
                }
            }
        }
    }
}
