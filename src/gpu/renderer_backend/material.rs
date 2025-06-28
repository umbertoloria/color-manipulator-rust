use crate::gpu::make_safe_filepath;
use crate::gpu::renderer_backend::bind_group::BindGroupBuilder;
use image::GenericImageView;
use std::fs::read;
use wgpu::wgt::TextureViewDescriptor;
use wgpu::{
    AddressMode, BindGroup, BindGroupLayout, Device, Extent3d, FilterMode, Origin3d, Queue,
    SamplerDescriptor, TexelCopyBufferLayout, TexelCopyTextureInfo, TextureAspect,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};

pub struct Material {
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
        let size = loaded_image.dimensions();

        // println!("Image size: {} x {}", size.0, size.1);

        let texture_size = Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        };

        let texture_descriptor = TextureDescriptor {
            label: Some(label),
            mip_level_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            size: texture_size,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[TextureFormat::Rgba8Unorm],
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
                bytes_per_row: Some(size.0 * 4),
                rows_per_image: Some(size.1),
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

        let mut bind_group_builder = BindGroupBuilder::new(device);
        bind_group_builder.set_layout(bind_group_layout);
        bind_group_builder.add_material(&view, &sampler);
        let bind_group = bind_group_builder.build(label);

        Self {
            //
            bind_group,
        }
    }
}
