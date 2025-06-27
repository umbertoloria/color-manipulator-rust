use glm::Vec3;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    vertex_attr_array, Buffer, BufferUsages, Device, VertexAttribute, VertexBufferLayout,
    VertexStepMode,
};

#[repr(C)]
pub struct Vertex {
    position: Vec3,
    color: Vec3,
}

pub struct Mesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub index_buffer_len: u32,
}

impl Vertex {
    pub fn get_layout() -> VertexBufferLayout<'static> {
        const ATTRIBUTES: [VertexAttribute; 2] = vertex_attr_array![0 => Float32x3, 1 => Float32x3];
        /* // Or...
        const ATTRIBUTES: [VertexAttribute] = [
            VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: size_of::<Vec3>::(),
                shader_location: 1,
            },
        ];*/
        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts((p as *const T) as *const u8, size_of::<T>())
}

pub fn make_triangle(device: &Device) -> Buffer {
    let vertices: [Vertex; 3] = [
        Vertex {
            position: Vec3::new(-0.75, -0.75, 0.0),
            color: Vec3::new(1.0, 0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.75, -0.75, 0.0),
            color: Vec3::new(0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.0, 0.75, 0.0),
            color: Vec3::new(0.0, 0.0, 1.0),
        },
    ];
    let vertices_bytes = unsafe { any_as_u8_slice(&vertices) };
    let vertex_buffer_descriptor = BufferInitDescriptor {
        label: Some("Triangle vertex buffer"),
        contents: vertices_bytes,
        usage: BufferUsages::VERTEX,
    };
    let vertex_buffer = device.create_buffer_init(&vertex_buffer_descriptor);
    vertex_buffer
}

pub fn make_quad(device: &Device) -> Mesh {
    let vertices: [Vertex; 4] = [
        Vertex {
            position: Vec3::new(-0.75, -0.75, 0.0),
            color: Vec3::new(1.0, 0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.75, -0.75, 0.0),
            color: Vec3::new(0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.75, 0.75, 0.0),
            color: Vec3::new(0.0, 0.0, 1.0),
        },
        Vertex {
            position: Vec3::new(-0.75, 0.75, 0.0),
            color: Vec3::new(0.0, 1.0, 1.0),
        },
    ];
    let vertices_bytes = unsafe { any_as_u8_slice(&vertices) };
    let vertex_buffer_descriptor = BufferInitDescriptor {
        label: Some("Quad vertex buffer"),
        contents: vertices_bytes,
        usage: BufferUsages::VERTEX,
    };
    let vertex_buffer = device.create_buffer_init(&vertex_buffer_descriptor);

    let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
    let indices_bytes: &[u8] = unsafe { any_as_u8_slice(&indices) };
    let index_buffer_descriptor = BufferInitDescriptor {
        label: Some("Quad index buffer"),
        contents: indices_bytes,
        usage: BufferUsages::INDEX,
    };
    let index_buffer = device.create_buffer_init(&index_buffer_descriptor);

    Mesh {
        vertex_buffer,
        index_buffer,
        index_buffer_len: indices.len() as u32,
    }
}
