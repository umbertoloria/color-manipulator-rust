use glm::{Vec2, Vec3};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    vertex_attr_array, Buffer, BufferAddress, BufferUsages, Device, VertexAttribute,
    VertexBufferLayout, VertexStepMode,
};

#[repr(C)]
pub struct Vertex {
    position: Vec3,
    color: Vec3,
    tex_coord: Vec2,
}

pub struct Mesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub index_buffer_len: u32,
}

impl Vertex {
    pub fn get_layout() -> VertexBufferLayout<'static> {
        const ATTRIBUTES: [VertexAttribute; 3] =
            vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2];
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
            VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: size_of::<Vec2>::(),
                shader_location: 2,
            },
        ];*/
        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts((p as *const T) as *const u8, size_of::<T>())
}

pub fn make_triangle(device: &Device) -> Buffer {
    let vertices = [
        Vertex {
            position: Vec3::new(-0.75, -0.75, 0.0),
            color: Vec3::new(1.0, 0.0, 0.0),
            tex_coord: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3::new(0.75, -0.75, 0.0),
            color: Vec3::new(0.0, 1.0, 0.0),
            tex_coord: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(0.0, 0.75, 0.0),
            color: Vec3::new(0.0, 0.0, 1.0),
            tex_coord: Vec2::new(0.5, 0.0),
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

pub fn make_rect(ratio: f32, device: &Device) -> Mesh {
    let inv_ratio = 1.0 / ratio;
    let offset_y = 1.0 - inv_ratio; // Vertically aligned on top.
    let vertices = [
        Vertex {
            position: Vec3::new(-1.0, -inv_ratio + offset_y, 0.0),
            color: Vec3::new(1.0, 1.0, 1.0), // White.
            tex_coord: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3::new(1.0, -inv_ratio + offset_y, 0.0),
            color: Vec3::new(1.0, 1.0, 1.0), // White.
            tex_coord: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(1.0, inv_ratio + offset_y, 0.0),
            color: Vec3::new(1.0, 1.0, 1.0), // White.
            tex_coord: Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3::new(-1.0, inv_ratio + offset_y, 0.0),
            color: Vec3::new(1.0, 1.0, 1.0), // White.
            tex_coord: Vec2::new(0.0, 0.0),
        },
    ];
    let vertices_bytes = unsafe { any_as_u8_slice(&vertices) };
    let vertex_buffer_descriptor = BufferInitDescriptor {
        label: Some("Quad vertex buffer"),
        contents: vertices_bytes,
        usage: BufferUsages::VERTEX,
    };
    let vertex_buffer = device.create_buffer_init(&vertex_buffer_descriptor);

    // Typing "u16" is important!
    let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];
    let indices_bytes = unsafe { any_as_u8_slice(&indices) };
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
