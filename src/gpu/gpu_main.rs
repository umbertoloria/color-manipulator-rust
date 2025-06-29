use crate::gpu::renderer_backend::bind_group_layout::BindGroupLayoutBuilder;
use crate::gpu::renderer_backend::material::{calculate_ratio, Material};
use crate::gpu::renderer_backend::mesh_builder::{make_rect, Mesh, Vertex};
use crate::gpu::renderer_backend::pipeline::PipelineBuilder;
use crate::gpu::window::Window;
use glfw::PRenderContext;
use image::{ImageBuffer, Rgba};
use wgpu::wgt::TextureViewDescriptor;
use wgpu::{
    Backends, BufferAddress, BufferDescriptor, BufferUsages, Color, CommandEncoderDescriptor,
    Device, DeviceDescriptor, Extent3d, IndexFormat, Instance, InstanceDescriptor, LoadOp,
    Operations, Origin3d, PollType, PowerPreference, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RequestAdapterOptionsBase, StoreOp, Surface,
    SurfaceConfiguration, SurfaceError, TexelCopyBufferInfo, TexelCopyBufferLayout,
    TexelCopyTextureInfo, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat,
    TextureUsages,
};

pub enum RenderResult {
    GoNextTick,
    StopRendering,
}

pub struct State<'a> {
    instance: Instance,
    surface: Surface<'a>,
    device: Device, // Abstract GPU
    queue: Queue,   // For submitting works
    config: SurfaceConfiguration,
    pub size: (u32, u32),
    render_pipeline: RenderPipeline,
    quad_mesh: Mesh,
    quad_material: Material,
    texture_full_width: u32,
    texture_full_height: u32,
    // triangle_mesh: Buffer,
    // triangle_material: Material,
}
impl<'a> State<'a> {
    async fn new(
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

    pub async fn render(&mut self) -> Result<RenderResult, SurfaceError> {
        /*
        // Texture View: render on Window.
        let drawable = self.surface.get_current_texture()?;
        let texture_view = drawable
            .texture
            .create_view(&TextureViewDescriptor::default());
        */

        // Texture View: render on Image.
        let texture = self.device.create_texture(&TextureDescriptor {
            label: Some("Output texture"),
            size: Extent3d {
                width: self.texture_full_width,
                height: self.texture_full_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::COPY_SRC | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[TextureFormat::Rgba8UnormSrgb],
        });
        let texture_view = texture.create_view(&TextureViewDescriptor::default());
        let u32_size = size_of::<u32>() as u32;
        let output_buffer_desc = BufferDescriptor {
            label: None,
            size: (u32_size * self.texture_full_width * self.texture_full_height) as BufferAddress,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        };
        let output_buffer = self.device.create_buffer(&output_buffer_desc);

        // Command Encoder
        let c_e_descriptor = CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };
        let mut command_encoder = self.device.create_command_encoder(&c_e_descriptor);
        {
            let render_pass_color_attachment = RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.0,
                    }),
                    store: StoreOp::Store,
                },
            };
            let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(render_pass_color_attachment)],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.set_bind_group(0, &self.quad_material.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.quad_mesh.vertex_buffer.slice(..));
            render_pass
                .set_index_buffer(self.quad_mesh.index_buffer.slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.quad_mesh.index_buffer_len, 0, 0..1);

            /*
            render_pass.set_bind_group(0, &self.triangle_material.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.triangle_mesh.slice(..));
            render_pass.draw(0..3, 0..1);
            */
        }

        // Render on a Texture.
        command_encoder.copy_texture_to_buffer(
            TexelCopyTextureInfo {
                aspect: TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
            },
            TexelCopyBufferInfo {
                buffer: &output_buffer,
                layout: TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(u32_size * self.texture_full_width),
                    rows_per_image: Some(self.texture_full_height),
                },
            },
            Extent3d {
                width: self.texture_full_width,
                height: self.texture_full_height,
                depth_or_array_layers: 1,
            },
        );
        self.queue.submit(Some(command_encoder.finish()));

        // Save Texture on an Image.
        {
            let buffer_slice = output_buffer.slice(..);

            // NOTE: We have to create the mapping THEN device.poll() before await
            // the future. Otherwise, the application will freeze.
            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
            self.device.poll(PollType::Wait).unwrap();
            rx.receive().await.unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(
                self.texture_full_width,
                self.texture_full_height,
                data,
            )
            .unwrap();
            buffer.save("image.png").unwrap();
        }
        output_buffer.unmap();

        // Draw on a Window.
        /*
        drawable.present();
        Ok(RenderResult::GoNextTick)
        */

        Ok(RenderResult::StopRendering)
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

pub async fn gpu_main() {
    let mut window = Window::new();
    let (width, height) = window.get_framebuffer_size();
    let initial_render_context = window.get_render_context();
    let mut state = State::new(width, height, &initial_render_context).await;

    window.render_loop(&mut state).await;
}
