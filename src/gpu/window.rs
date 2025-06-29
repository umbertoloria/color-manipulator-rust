use glfw::{fail_on_errors, Action, ClientApiHint, Glfw, GlfwReceiver, Key, PRenderContext, PWindow, WindowEvent, WindowHint};
use wgpu::SurfaceError;
use crate::gpu::gpu_main::{RenderResult, State};

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

    pub async fn render_loop<'a>(&'a mut self, state: &mut State<'a>) {
        // self.window.set_all_polling(true);
        self.window.set_key_polling(true);
        self.window.set_framebuffer_size_polling(true);
        self.window.set_pos_polling(true);
        // self.window.set_mouse_button_polling(true);

        while !self.window.should_close() {
            self.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true)
                    }

                    WindowEvent::FramebufferSize(width, height) => {
                        state.update_surface(&self.render_context);
                        state.resize((width as u32, height as u32));
                    }

                    WindowEvent::Pos(..) => {
                        // Workaround for Window Move.
                        state.update_surface(&self.render_context);
                        state.resize(state.size);
                    }

                    _ => {
                        // println!("{:?}", e);
                    }
                }
            }

            match state.render().await {
                Ok(render_result) => match render_result {
                    RenderResult::GoNextTick => {}
                    RenderResult::StopRendering => {
                        break;
                    }
                },
                Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                    // Workaround on Window Resize.
                    state.update_surface(&self.render_context);
                    state.resize(state.size);
                }
                Err(e) => eprintln!("{:?}", e),
            };
        }
    }
}
