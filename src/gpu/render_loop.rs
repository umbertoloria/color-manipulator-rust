use crate::gpu::state::State;
use crate::gpu::window::Window;
use glfw::{flush_messages, Action, Key, WindowEvent};
use wgpu::SurfaceError;

pub enum RenderResult {
    GoNextTick,
    StopRendering,
}

pub async fn gpu_main() {
    let mut window = Window::new();

    let (width, height) = window.get_framebuffer_size();
    let render_context = window.get_render_context();
    let mut state = State::new(width, height, &render_context).await;

    // Render Loop
    window.prepare_events();
    while !window.window.should_close() {
        // Dispatch Events
        window.glfw.poll_events();
        for (_, event) in flush_messages(&window.events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.window.set_should_close(true);
                }

                WindowEvent::FramebufferSize(width, height) => {
                    state.update_surface(&render_context);
                    state.resize((width as u32, height as u32));
                }

                WindowEvent::Pos(..) => {
                    // Workaround for Window Move.
                    state.update_surface(&render_context);
                    state.resize(state.size);
                }

                _ => {
                    // println!("{:?}", e);
                }
            }
        }

        // Render
        match state.render().await {
            Ok(render_result) => match render_result {
                RenderResult::GoNextTick => {}
                RenderResult::StopRendering => {
                    break;
                }
            },
            Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                // Workaround on Window Resize.
                state.update_surface(&render_context);
                state.resize(state.size);
            }
            Err(e) => eprintln!("{:?}", e),
        };
    }
}
