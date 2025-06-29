use crate::gpu::wgpu::USED_PIXEL_FORMAT;
use glfw::{
    fail_on_errors, flush_messages, Action, ClientApiHint, GlfwReceiver, Key, PRenderContext,
    PWindow, WindowEvent, WindowHint,
};
use wgpu::{Adapter, Device, Instance, Surface, SurfaceConfiguration, TextureUsages};

pub struct GlfwWrapper<'a> {
    window_state: WindowState<'a>,
    glfw_events: GlfwReceiver<(f64, WindowEvent)>,
}
impl<'a> GlfwWrapper<'a> {
    pub fn init_window(
        width: u32,
        height: u32,
        title: &str,
    ) -> (PWindow, GlfwReceiver<(f64, WindowEvent)>) {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
        let (glfw_window, glfw_events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .unwrap();
        (
            //
            glfw_window,
            glfw_events,
        )
    }
    pub fn create_glfw_surface(
        instance: &Instance,
        glfw_render_context: &'a PRenderContext,
    ) -> Surface<'a> {
        instance.create_surface(glfw_render_context).unwrap()
    }
    pub fn new(
        window_state: WindowState<'a>,
        glfw_events: GlfwReceiver<(f64, WindowEvent)>,
    ) -> Self {
        Self {
            window_state,
            glfw_events,
        }
    }
    pub fn enable_events_polling(&mut self) {
        self.window_state.enable_events_polling();
    }
    pub fn should_close(&self) -> bool {
        self.window_state.should_close()
    }
    pub fn dispatch_events(
        &mut self,
        instance: &Instance,
        device: &Device,
        glfw_render_context: &'a PRenderContext,
    ) {
        self.window_state
            .dispatch_events(&self.glfw_events, instance, device, glfw_render_context);
    }
}

pub struct WindowState<'a> {
    pub glfw_window: PWindow,
    pub surface: Surface<'a>,
    pub surface_config: SurfaceConfiguration,
    pub curr_size: (u32, u32),
}
impl<'a> WindowState<'a> {
    pub fn new(
        adapter: &Adapter,
        device: &Device,
        glfw_window: PWindow,
        surface: Surface<'a>,
    ) -> Self {
        let window_size = glfw_window.get_framebuffer_size();
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
            glfw_window,
            surface,
            surface_config,
            curr_size: (width, height),
        }
    }
    pub fn should_close(&self) -> bool {
        self.glfw_window.should_close()
    }
    pub fn resize(&mut self, device: &Device, new_size: (u32, u32)) {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.curr_size = new_size;
            self.surface_config.width = new_size.0;
            self.surface_config.height = new_size.1;
            self.surface.configure(&device, &self.surface_config);
        }
    }
    pub fn update_surface(&mut self, instance: &Instance, glfw_render_context: &'a PRenderContext) {
        self.surface = GlfwWrapper::create_glfw_surface(&instance, glfw_render_context);
    }
    pub fn enable_events_polling(&mut self) {
        // self.glfw_window.set_all_polling(true);
        self.glfw_window.set_key_polling(true);
        self.glfw_window.set_framebuffer_size_polling(true);
        self.glfw_window.set_pos_polling(true);
        // self.glfw_window.set_mouse_button_polling(true);
    }
    pub fn dispatch_events(
        &mut self,
        glfw_events: &GlfwReceiver<(f64, WindowEvent)>,
        instance: &Instance,
        device: &Device,
        glfw_render_context: &'a PRenderContext,
    ) {
        // Dispatch Events
        self.glfw_window.glfw.poll_events();
        let messages = flush_messages(glfw_events);
        for (_, event) in messages {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.glfw_window.set_should_close(true);
                }

                WindowEvent::FramebufferSize(width, height) => {
                    self.update_surface(&instance, &glfw_render_context);
                    self.resize(&device, (width as u32, height as u32));
                }

                WindowEvent::Pos(..) => {
                    // Workaround for Window Move.
                    self.update_surface(&instance, &glfw_render_context);
                    self.resize(&device, self.curr_size);
                }

                _ => {
                    // println!("{:?}", e);
                }
            }
        }
    }
}
