use glfw::{fail_on_errors, Action, Context, Key, WindowEvent};

pub fn gpu_main() {
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    const WIN_WIDTH: u32 = 800;
    const WIN_HEIGHT: u32 = 600;
    const WIN_TITLE: &str = "Window title";

    let (mut window, events) = glfw
        .create_window(WIN_WIDTH, WIN_HEIGHT, WIN_TITLE, glfw::WindowMode::Windowed)
        .unwrap();

    // window.set_all_polling(true);
    window.set_key_polling(true);

    window.make_current();

    while !window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
                _ => {
                    // println!("{:?}", e);
                }
            }
        }

        window.swap_buffers();
    }
}
