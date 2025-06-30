use crate::gpu::renderer_backend::bind_group_layout::BindGroupLayoutBuilder;
use crate::gpu::renderer_backend::material::Material;
use crate::gpu::renderer_backend::pipeline::PipelineBuilder;
use crate::gpu::wgpu::WGPUWrapper;
use wgpu::{BindGroupLayout, Instance};

pub async fn create_gpu_context() -> GpuContext {
    // WGPU
    let instance = WGPUWrapper::init_instance();
    let wgpu_wrapper = WGPUWrapper::new(&instance, None).await;

    // Glfw
    /*
    const WIN_WIDTH: u32 = 900;
    const WIN_HEIGHT: u32 = 900;
    const WIN_TITLE: &str = "Window title";
    let (
        //
        mut glfw_window,
        glfw_events,
    ) = GlfwWrapper::init_window(WIN_WIDTH, WIN_HEIGHT, WIN_TITLE);
    let glfw_render_context = glfw_window.render_context();
    let window_surface = GlfwWrapper::create_glfw_surface(&instance, &glfw_render_context);
    let window_state = WindowState::new(
        &wgpu_wrapper.adapter,
        &wgpu_wrapper.device,
        glfw_window,
        window_surface,
    );
    let mut glfw_wrapper = GlfwWrapper::new(window_state, glfw_events);
    */

    GpuContext {
        instance,
        wgpu_wrapper,
    }
}

pub struct GpuContext {
    pub instance: Instance,
    pub wgpu_wrapper: WGPUWrapper,
}
impl GpuContext {
    pub fn create_bind_group_layout(&self) -> BindGroupLayoutBuilder {
        BindGroupLayoutBuilder::new(&self.wgpu_wrapper.device)
    }
    pub fn create_material(
        &self,
        filename: &str,
        label: &str,
        bind_group_layout: &BindGroupLayout,
    ) -> Material {
        Material::new(
            filename,
            &self.wgpu_wrapper.device,
            &self.wgpu_wrapper.queue,
            label,
            &bind_group_layout,
        )
    }
    pub fn create_render_pipeline_builder(&self) -> PipelineBuilder {
        PipelineBuilder::new(&self.wgpu_wrapper.device)
    }
}
