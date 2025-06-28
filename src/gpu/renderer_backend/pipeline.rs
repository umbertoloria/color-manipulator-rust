use crate::gpu::make_safe_filepath;
use std::fs::read_to_string;
use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Device, Face, FragmentState, FrontFace,
    MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode,
    PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor,
    ShaderModuleDescriptor, ShaderSource, TextureFormat, VertexBufferLayout, VertexState,
};

pub struct PipelineBuilder<'a> {
    shader_filename: String,
    vertex_entry: String,
    fragment_entry: String,
    pixel_format: TextureFormat,
    vertex_buffer_layouts: Vec<VertexBufferLayout<'static>>,
    device: &'a Device,
}
impl<'a> PipelineBuilder<'a> {
    pub fn new(device: &'a Device) -> Self {
        Self {
            shader_filename: "dummy".into(),
            vertex_entry: "dummy".into(),
            fragment_entry: "dummy".into(),
            pixel_format: TextureFormat::Rgba8Unorm,
            vertex_buffer_layouts: Vec::new(),
            device,
        }
    }

    fn reset(&mut self) {
        self.vertex_buffer_layouts.clear();
    }

    pub fn add_vertex_buffer_layout(&mut self, layout: VertexBufferLayout<'static>) {
        self.vertex_buffer_layouts.push(layout);
    }

    pub fn set_shader_module(
        &mut self,
        shader_filename: &str,
        vertex_entry: &str,
        fragment_entry: &str,
    ) {
        self.shader_filename = shader_filename.into();
        self.vertex_entry = vertex_entry.into();
        self.fragment_entry = fragment_entry.into();
    }

    pub fn set_pixel_format(&mut self, pixel_format: TextureFormat) {
        self.pixel_format = pixel_format;
    }

    pub fn build(&mut self) -> RenderPipeline {
        let source_code = read_to_string(make_safe_filepath(&self.shader_filename))
            .expect("Can't read source code!");

        let shader_module_descriptor = ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: ShaderSource::Wgsl(source_code.into()),
        };
        let shader_module = self.device.create_shader_module(shader_module_descriptor);

        let pipeline_layout_descriptor = PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        };
        let pipeline_layout = self
            .device
            .create_pipeline_layout(&pipeline_layout_descriptor);

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
                buffers: &self.vertex_buffer_layouts,
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

        let render_pipeline = self
            .device
            .create_render_pipeline(&render_pipeline_descriptor);

        self.reset();

        render_pipeline
    }
}
