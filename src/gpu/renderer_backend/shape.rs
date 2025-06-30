use crate::gpu::gpu_context::GpuContext;
use crate::gpu::renderer_backend::mesh_builder::{unsafe_u8_slice_from_vec_of_u16s, Mesh, Vertex};
use glm::{Vec2, Vec3};
use wgpu::util::BufferInitDescriptor;
use wgpu::BufferUsages;

pub struct ShapeBuilder<'a> {
    vertices: Vec<Vertex>,
    indices: Vec<u16>, // Typing "u16" is important!
    gpu_context: &'a GpuContext,
}
impl<'a> ShapeBuilder<'a> {
    pub fn new(gpu_context: &'a GpuContext) -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            gpu_context,
        }
    }
    pub fn use_custom_rect(
        &mut self,
        top_left: Vec2,
        top_right: Vec2,
        bottom_right: Vec2,
        bottom_left: Vec2,
    ) -> &mut ShapeBuilder<'a> {
        self.vertices = vec![
            Vertex {
                position: Vec3::new(top_left.x, top_left.y, 0.0),
                color: Vec3::new(1.0, 1.0, 1.0), // White.
                tex_coord: Vec2::new(0.0, 0.0),
            },
            Vertex {
                position: Vec3::new(top_right.x, top_right.y, 0.0),
                color: Vec3::new(1.0, 1.0, 1.0), // White.
                tex_coord: Vec2::new(1.0, 0.0),
            },
            Vertex {
                position: Vec3::new(bottom_right.x, bottom_right.y, 0.0),
                color: Vec3::new(1.0, 1.0, 1.0), // White.
                tex_coord: Vec2::new(1.0, 1.0),
            },
            Vertex {
                position: Vec3::new(bottom_left.x, bottom_left.y, 0.0),
                color: Vec3::new(1.0, 1.0, 1.0), // White.
                tex_coord: Vec2::new(0.0, 1.0),
            },
        ];
        self.indices = vec![
            1, 0, 3, // TL triangle.
            3, 2, 1, // BR triangle.
        ];
        self
    }
    pub fn build(&self) -> Mesh {
        let vertices_bytes = unsafe_u8_slice_from_vec_of_u16s(&self.vertices);
        let vertex_buffer_descriptor = BufferInitDescriptor {
            label: Some("Built vertex buffer"),
            contents: vertices_bytes,
            usage: BufferUsages::VERTEX,
        };
        let vertex_buffer = self
            .gpu_context
            .create_buffer_init(&vertex_buffer_descriptor);

        let indices_bytes = unsafe_u8_slice_from_vec_of_u16s(&self.indices);
        let index_buffer_descriptor = BufferInitDescriptor {
            label: Some("Built index buffer"),
            contents: indices_bytes,
            usage: BufferUsages::INDEX,
        };
        let index_buffer = self
            .gpu_context
            .create_buffer_init(&index_buffer_descriptor);

        Mesh {
            vertex_buffer,
            index_buffer,
            index_buffer_len: self.indices.len() as u32,
        }
    }
}
