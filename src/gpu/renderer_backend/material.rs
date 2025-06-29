use crate::gpu::renderer_backend::bind_group::BindGroupBuilder;
use crate::gpu::shaders::read_file::make_safe_filepath;
use image::GenericImageView;
use std::fs::read;
use wgpu::wgt::TextureViewDescriptor;
use wgpu::{
    AddressMode, BindGroup, BindGroupLayout, Device, Extent3d, FilterMode, Origin3d, Queue,
    SamplerDescriptor, TexelCopyBufferLayout, TexelCopyTextureInfo, TextureAspect,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};

pub struct Material {
    pub width: u32,
    pub height: u32,
    pub bind_group: BindGroup,
}
impl Material {
    pub fn new(
        filename: &str,
        device: &Device,
        queue: &Queue,
        label: &str,
        bind_group_layout: &BindGroupLayout,
    ) -> Self {
        let bytes = read(make_safe_filepath(filename)).expect("Can't read material!");

        let loaded_image = image::load_from_memory(&bytes).unwrap();
        let converted = loaded_image.to_rgba8();
        let (width, height) = loaded_image.dimensions();
        // println!("Image size: {} x {}", width, height);

        let texture_size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture_descriptor = TextureDescriptor {
            label: Some(label),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[TextureFormat::Rgba8UnormSrgb],
        };
        let texture = device.create_texture(&texture_descriptor);

        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &converted,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
            texture_size,
        );

        let view = texture.create_view(&TextureViewDescriptor::default());

        let sampler_descriptor = SamplerDescriptor {
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            min_filter: FilterMode::Nearest,
            mag_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        };
        let sampler = device.create_sampler(&sampler_descriptor);

        let bind_group = BindGroupBuilder::new(device)
            .set_layout(bind_group_layout)
            .add_material(&view, &sampler)
            .build(label);

        Self {
            width,
            height,
            bind_group,
        }
    }
}

pub fn calculate_ratio(width: f32, height: f32) -> f32 {
    width / height
}
