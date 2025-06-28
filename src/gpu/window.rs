use crate::gpu::renderer_backend::bind_group_layout::BindGroupLayoutBuilder;
use crate::gpu::renderer_backend::material::{calculate_ratio, Material};
use crate::gpu::renderer_backend::mesh_builder::{make_rect, Mesh, Vertex};
use crate::gpu::renderer_backend::pipeline::PipelineBuilder;
use glfw::{fail_on_errors, Action, ClientApiHint, Key, Window, WindowEvent, WindowHint};
use wgpu::wgt::TextureViewDescriptor;
use wgpu::{
    Backends, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, IndexFormat, Instance,
    InstanceDescriptor, LoadOp, Operations, PowerPreference, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RequestAdapterOptionsBase, StoreOp, Surface,
    SurfaceConfiguration, SurfaceError, TextureFormat, TextureUsages,
};

struct State<'a> {
    instance: Instance,
    surface: Surface<'a>,
    device: Device, // Abstract GPU
    queue: Queue,   // For submitting works
    config: SurfaceConfiguration,
    size: (i32, i32),
    window: &'a mut Window,
    render_pipeline: RenderPipeline,
    quad_mesh: Mesh,
    quad_material: Material,
    // triangle_mesh: Buffer,
    // triangle_material: Material,
}
impl<'a> State<'a> {
    async fn new(window: &'a mut Window) -> Self {
        let (width, height) = window.get_framebuffer_size();

        let instance_descriptor = InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        };
        let instance = Instance::new(&instance_descriptor);
        let surface = Self::create_wgpu_surface(&instance, window);

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
            width: width as u32,
            height: height as u32,
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

        let quad_texture_ratio =
            calculate_ratio(quad_material.width as f32, quad_material.height as f32);
        // println!("Image aspect ratio: {}", quad_texture_ratio);

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
            size: (width, height),
            window,
            render_pipeline,
            quad_mesh,
            quad_material,
            // triangle_mesh,
            // triangle_material,
        }
    }

    fn render(&mut self) -> Result<(), SurfaceError> {
        let drawable = self.surface.get_current_texture()?;
        let image_view = drawable
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut command_encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let color_attachment = RenderPassColorAttachment {
            view: &image_view,
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

        {
            let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(color_attachment)],
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
        self.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();

        Ok(())
    }

    fn resize(&mut self, new_size: (i32, i32)) {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0 as u32;
            self.config.height = new_size.1 as u32;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn update_surface(&mut self) {
        self.surface = Self::create_wgpu_surface(&self.instance, self.window);
    }

    fn create_wgpu_surface(instance: &Instance, window: &mut Window) -> Surface<'a> {
        instance.create_surface(window.render_context()).unwrap()
    }
}

pub async fn gpu_main() {
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));

    const WIN_WIDTH: u32 = 900;
    const WIN_HEIGHT: u32 = 900;
    const WIN_TITLE: &str = "Window title";

    let (mut window, events) = glfw
        .create_window(WIN_WIDTH, WIN_HEIGHT, WIN_TITLE, glfw::WindowMode::Windowed)
        .unwrap();

    let mut state = State::new(&mut window).await;

    // state.window.set_all_polling(true);
    state.window.set_key_polling(true);
    state.window.set_framebuffer_size_polling(true);
    state.window.set_pos_polling(true);
    // state.window.set_mouse_button_polling(true);

    while !state.window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    state.window.set_should_close(true)
                }

                WindowEvent::FramebufferSize(width, height) => {
                    state.update_surface();
                    state.resize((width, height));
                }

                WindowEvent::Pos(..) => {
                    // Workaround for Window Move.
                    state.update_surface();
                    state.resize(state.size);
                }

                _ => {
                    // println!("{:?}", e);
                }
            }
        }

        match state.render() {
            Ok(_) => {}
            Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                // Workaround on Window Resize.
                state.update_surface();
                state.resize(state.size);
            }
            Err(e) => eprintln!("{:?}", e),
        };
    }
}
