use glfw::PRenderContext;
use wgpu::{Instance, Surface};

pub struct Window {}
impl Window {
    pub fn create_glfw_surface<'a>(
        instance: &Instance,
        render_context: &'a PRenderContext,
    ) -> Surface<'a> {
        instance.create_surface(render_context).unwrap()
    }
}
