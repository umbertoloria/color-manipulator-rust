use glfw::{
    fail_on_errors, ClientApiHint, FlushedMessages, Glfw, GlfwReceiver,
    PRenderContext, PWindow, WindowEvent, WindowHint,
};

pub struct Window {
    pub glfw: Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
    pub render_context: PRenderContext,
}
impl Window {
    pub fn new() -> Self {
        const WIN_WIDTH: u32 = 900;
        const WIN_HEIGHT: u32 = 900;
        const WIN_TITLE: &str = "Window title";

        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
        let (mut window, events) = glfw
            .create_window(WIN_WIDTH, WIN_HEIGHT, WIN_TITLE, glfw::WindowMode::Windowed)
            .unwrap();
        let render_context = window.render_context();

        Self {
            glfw,
            window,
            events,
            render_context,
        }
    }

    pub fn get_framebuffer_size(&self) -> (u32, u32) {
        let (width, height) = self.window.get_framebuffer_size();
        let width = width as u32;
        let height = height as u32;
        (width, height)
    }

    pub fn get_render_context(&mut self) -> PRenderContext {
        self.window.render_context()
    }

    pub fn before_rendering(&mut self) {
        // self.window.set_all_polling(true);
        self.window.set_key_polling(true);
        self.window.set_framebuffer_size_polling(true);
        self.window.set_pos_polling(true);
        // self.window.set_mouse_button_polling(true);
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }
    pub fn set_should_close(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn poll_and_get_events(&mut self) -> FlushedMessages<'_, (f64, WindowEvent)> {
        self.glfw.poll_events();
        glfw::flush_messages(&self.events)
    }
}
