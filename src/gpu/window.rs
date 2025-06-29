use glfw::{
    fail_on_errors, ClientApiHint, Glfw, GlfwReceiver, PRenderContext, PWindow, WindowEvent,
    WindowHint,
};
use wgpu::{Instance, Surface};

pub struct Window {
    pub width: u32,
    pub height: u32,
    pub glfw: Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
}
impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        const WIN_TITLE: &str = "Window title";

        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
        let (window, events) = glfw
            .create_window(width, height, WIN_TITLE, glfw::WindowMode::Windowed)
            .unwrap();

        Self {
            width,
            height,
            glfw,
            window,
            events,
        }
    }

    /*
    pub fn get_framebuffer_size(&self) -> (u32, u32) {
        let (width, height) = self.window.get_framebuffer_size();
        let width = width as u32;
        let height = height as u32;
        (width, height)
    }
    */

    pub fn get_render_context(&mut self) -> PRenderContext {
        self.window.render_context()
    }

    pub fn prepare_events(&mut self) {
        // self.window.set_all_polling(true);
        self.window.set_key_polling(true);
        self.window.set_framebuffer_size_polling(true);
        self.window.set_pos_polling(true);
        // self.window.set_mouse_button_polling(true);
    }

    pub fn create_glfw_surface<'a>(
        instance: &Instance,
        render_context: &'a PRenderContext,
    ) -> Surface<'a> {
        instance.create_surface(render_context).unwrap()
    }
}
