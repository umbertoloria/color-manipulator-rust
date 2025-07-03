use crate::gpu::renderer_backend::bind_group_layout::BindGroupLayoutBuilder;
use crate::gpu::renderer_backend::material::Material;
use crate::gpu::renderer_backend::pipeline::PipelineBuilder;
use crate::gpu::renderer_backend::shape::ShapeBuilder;
use crate::gpu::wgpu::WGPUWrapper;
use image::RgbaImage;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    AddressMode, BindGroupLayout, Buffer, BufferDescriptor, CommandBuffer, CommandEncoder,
    CommandEncoderDescriptor, FilterMode, Instance, PollType, Sampler, SamplerDescriptor, Texture,
    TextureDescriptor,
};

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
    instance: Instance,
    wgpu_wrapper: WGPUWrapper,
}
impl GpuContext {
    pub fn create_bind_group_layout_builder(&self) -> BindGroupLayoutBuilder {
        BindGroupLayoutBuilder::new(&self.wgpu_wrapper.device)
    }
    pub fn create_sampler(&self) -> Sampler {
        let sampler_descriptor = SamplerDescriptor {
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            min_filter: FilterMode::Nearest,
            mag_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        };
        let sampler = self.wgpu_wrapper.device.create_sampler(&sampler_descriptor);
        sampler
    }
    pub fn create_material<'a>(
        &self,
        image: &'a RgbaImage,
        label: &str,
        bind_group_layout: &BindGroupLayout,
        sampler: &Sampler,
    ) -> Material<'a> {
        Material::new(
            image,
            label,
            &bind_group_layout,
            &self.wgpu_wrapper.device,
            &self.wgpu_wrapper.queue,
            sampler,
        )
    }
    pub fn create_render_pipeline_builder(&self) -> PipelineBuilder {
        PipelineBuilder::new(&self.wgpu_wrapper.device)
    }
    pub fn create_shape_builder(&self) -> ShapeBuilder {
        ShapeBuilder::new(self)
    }
    pub fn create_buffer_init(&self, buffer_init_descriptor: &BufferInitDescriptor) -> Buffer {
        self.wgpu_wrapper
            .device
            .create_buffer_init(&buffer_init_descriptor)
    }
    pub fn create_texture(&self, texture_descriptor: &TextureDescriptor) -> Texture {
        self.wgpu_wrapper.device.create_texture(texture_descriptor)
    }
    pub fn create_buffer(&self, buffer_descriptor: &BufferDescriptor) -> Buffer {
        self.wgpu_wrapper.device.create_buffer(&buffer_descriptor)
    }
    pub fn create_command_encoder(
        &self,
        command_encoder_descriptor: &CommandEncoderDescriptor,
    ) -> CommandEncoder {
        self.wgpu_wrapper
            .device
            .create_command_encoder(&command_encoder_descriptor)
    }
    pub fn submit_to_queue(&self, command_buffer: CommandBuffer) {
        self.wgpu_wrapper.queue.submit(Some(command_buffer));
    }
    pub fn poll_activities_waiting(&self) {
        self.wgpu_wrapper.device.poll(PollType::Wait).unwrap();
    }
}
