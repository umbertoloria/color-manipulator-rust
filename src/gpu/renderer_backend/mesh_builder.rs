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

    let bytes: &[u8] = unsafe { any_as_u8_slice(&vertices) };

    let buffer_descriptor = BufferInitDescriptor {
        label: Some("Triangle vertex buffer"),
        contents: bytes,
        usage: BufferUsages::VERTEX,
    };

    device.create_buffer_init(&buffer_descriptor)
}
