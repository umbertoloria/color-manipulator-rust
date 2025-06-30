use crate::gpu::gpu_context::GpuContext;
use crate::gpu::renderer_backend::mesh_builder::{any_as_u8_slice, Mesh, Vertex};
use glm::{Vec2, Vec3};
use wgpu::util::BufferInitDescriptor;
use wgpu::BufferUsages;

pub struct ShapeBuilder<'a> {
    gpu_context: &'a GpuContext,
}
impl<'a> ShapeBuilder<'a> {
    pub fn new(gpu_context: &'a GpuContext) -> Self {
        Self {
            //
            gpu_context,
        }
    }
    pub fn build_custom_rect(
        &self,
        top_left: Vec2,
        top_right: Vec2,
        bottom_right: Vec2,
        bottom_left: Vec2,
    ) -> Mesh {
        let vertices = [
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
        let vertices_bytes = unsafe { any_as_u8_slice(&vertices) };
        let vertex_buffer_descriptor = BufferInitDescriptor {
            label: Some("Custom rect vertex buffer"),
            contents: vertices_bytes,
            usage: BufferUsages::VERTEX,
        };
        let vertex_buffer = self
            .gpu_context
            .create_buffer_init(&vertex_buffer_descriptor);

        // Typing "u16" is important!
        let indices: [u16; 6] = [1, 0, 3, 3, 2, 1]; // First TL triangle, then BR triangle.
        let indices_bytes = unsafe { any_as_u8_slice(&indices) };
        let index_buffer_descriptor = BufferInitDescriptor {
            label: Some("Custom rect index buffer"),
            contents: indices_bytes,
            usage: BufferUsages::INDEX,
        };
        let index_buffer = self
            .gpu_context
            .create_buffer_init(&index_buffer_descriptor);

        Mesh {
            vertex_buffer,
            index_buffer,
            index_buffer_len: indices.len() as u32,
        }
    }
}
