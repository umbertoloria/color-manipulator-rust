use crate::gpu::renderer_backend::bind_group_layout::BindGroupLayoutBuilder;
use crate::gpu::renderer_backend::material::{calculate_ratio, Material};
use crate::gpu::renderer_backend::mesh_builder::{make_rect, Mesh, Vertex};
use crate::gpu::renderer_backend::pipeline::PipelineBuilder;
use glfw::PRenderContext;
use wgpu::{
    Backends, Device, DeviceDescriptor, Instance, InstanceDescriptor, PowerPreference, Queue,
    RenderPipeline, RequestAdapterOptionsBase, Surface, SurfaceConfiguration, TextureFormat,
    TextureUsages,
};

pub struct State<'a> {
    instance: Instance,
    surface: Surface<'a>,
    pub device: Device, // Abstract GPU
    pub queue: Queue,   // For submitting works
    config: SurfaceConfiguration,
    pub size: (u32, u32),
    pub render_pipeline: RenderPipeline,
    pub quad_mesh: Mesh,
    pub quad_material: Material,
    pub texture_full_width: u32,
    pub texture_full_height: u32,
}
impl<'a> State<'a> {
    pub async fn new(
        framebuffer_width: u32,
        framebuffer_height: u32,
        render_context: &'a PRenderContext,
    ) -> Self {
        let instance_descriptor = InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        };
        let instance = Instance::new(&instance_descriptor);
        let surface = Self::create_wgpu_surface(&instance, render_context);

        let adapter = instance
            .request_adapter(&RequestAdapterOptionsBase {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        /*
        let device_descriptor = DeviceDescriptor {
            label: Some("Device"),
            required_features: Features::empty(),
            required_limits: Limits::default(),
            memory_hints: MemoryHints::default(),
            trace: Trace::default(),
        };
        */
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default())
            .await
            .unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        /*
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_capabilities.formats[0]);
        for x in surface_capabilities.formats {
            println!(" -> {:?}", x);
        }
        println!(" PICKED -> {:?}", surface_format);
        */
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            // Don't do "format: surface_format" since it picks "Bgra8UnormSrgb" (at least on my PC)
            // but it's the wrong one.
            format: TextureFormat::Rgba8UnormSrgb, // Right one.
            width: framebuffer_width,
            height: framebuffer_height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: Vec::new(),
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let material_bind_group_layout = {
            let mut bind_group_layout_builder = BindGroupLayoutBuilder::new(&device);
            bind_group_layout_builder.add_material();
            bind_group_layout_builder.build("Material Bind Group Layout")
        };

        let render_pipeline = {
            let mut pipeline_builder = PipelineBuilder::new(&device);
            pipeline_builder.set_shader_module("src/gpu/shaders/shader.wgsl", "vs_main", "fs_main");
            pipeline_builder.set_pixel_format(config.format);
            pipeline_builder.add_vertex_buffer_layout(Vertex::get_layout());
            pipeline_builder.add_bind_group_layout(&material_bind_group_layout);
            pipeline_builder.build("Render Pipeline")
        };

        let quad_material = Material::new(
            "input/20230301_224920.jpg",
            &device,
            &queue,
            "Quad Material",
            &material_bind_group_layout,
        );

        let block_size = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let texture_full_width: u32 =
            quad_material.width + (block_size - (quad_material.width % block_size)) % block_size;
        let texture_full_height: u32 =
            quad_material.height + (block_size - (quad_material.height % block_size)) % block_size;
        // Using quad texture.
        let texture_full_height: u32 = texture_full_width;

        let quad_texture_ratio =
            calculate_ratio(quad_material.width as f32, quad_material.height as f32)
                / calculate_ratio(texture_full_width as f32, texture_full_height as f32);

        let quad_mesh = make_rect(quad_texture_ratio, &device);

        /*
        let triangle_mesh = make_triangle(&device);
        let triangle_material = Material::new(
            "input/20240714_1958.png",
            &device,
            &queue,
            "Triangle Material",
            &material_bind_group_layout,
        );
        */

        Self {
            instance,
            surface,
            device,
            queue,
            config,
            size: (framebuffer_width, framebuffer_height),
            render_pipeline,
            quad_mesh,
            quad_material,
            texture_full_width,
            texture_full_height,
            // triangle_mesh,
            // triangle_material,
        }
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0;
            self.config.height = new_size.1;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn update_surface(&mut self, render_context: &'a PRenderContext) {
        self.surface = Self::create_wgpu_surface(&self.instance, render_context);
    }

    fn create_wgpu_surface(instance: &Instance, render_context: &'a PRenderContext) -> Surface<'a> {
        instance.create_surface(render_context).unwrap()
    }
}
