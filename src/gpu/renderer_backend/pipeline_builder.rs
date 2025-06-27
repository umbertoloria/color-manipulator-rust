use std::env::current_dir;
use std::fs::read_to_string;
use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Face, FragmentState, FrontFace, MultisampleState,
    PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode, PrimitiveState,
    PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor,
    ShaderSource, TextureFormat, VertexBufferLayout, VertexState,
};

pub struct PipelineBuilder {
    shader_filename: String,
    vertex_entry: String,
    fragment_entry: String,
    pixel_format: TextureFormat,
    vertex_buffer_layout: Vec<VertexBufferLayout<'static>>,
}
impl PipelineBuilder {
    pub fn new() -> Self {
        Self {
            shader_filename: "dummy".into(),
            vertex_entry: "dummy".into(),
            fragment_entry: "dummy".into(),
            pixel_format: TextureFormat::Rgba8Unorm,
            vertex_buffer_layout: Vec::new(),
        }
    }

    pub fn add_buffer_layout(&mut self, layout: VertexBufferLayout<'static>) {
        self.vertex_buffer_layout.push(layout);
    }

    pub fn set_shader_module(
        &mut self,
        shader_filename: &str,
        vertex_entry: &str,
        fragment_enty: &str,
    ) {
        self.shader_filename = shader_filename.into();
        self.vertex_entry = vertex_entry.into();
        self.fragment_entry = fragment_enty.into();
    }

    pub fn set_pixel_format(&mut self, pixel_format: TextureFormat) {
        self.pixel_format = pixel_format;
    }

    pub fn build_pipeline(&mut self, device: &wgpu::Device) -> RenderPipeline {
        let mut filepath = current_dir().unwrap();
        filepath.push("src/");
        filepath.push(self.shader_filename.as_str());
        let filepath = filepath.into_os_string().into_string().unwrap();
        let source_code = read_to_string(filepath).expect("Can't read source code!");

        let shader_module_descriptor = ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: ShaderSource::Wgsl(source_code.into()),
        };
        let shader_module = device.create_shader_module(shader_module_descriptor);

        let pipeline_layout_descriptor = PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        };
        let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_descriptor);

        let render_targets = [Some(ColorTargetState {
            format: self.pixel_format,
            blend: Some(BlendState::REPLACE),
            write_mask: ColorWrites::ALL,
        })];

        let render_pipeline_descriptor = RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),

            vertex: VertexState {
                module: &shader_module,
                entry_point: Some(&self.vertex_entry),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &self.vertex_buffer_layout,
            },

            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },

            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0, // Or 1.
                alpha_to_coverage_enabled: false,
            },

            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: Some(&self.fragment_entry),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &render_targets,
            }),

            multiview: None,
            cache: None, // Use cache?
        };

        device.create_render_pipeline(&render_pipeline_descriptor)
    }
}
