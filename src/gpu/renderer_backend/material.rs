use crate::gpu::renderer_backend::bind_group::BindGroupBuilder;
use crate::gpu::wgpu::USED_PIXEL_FORMAT;
use image::RgbaImage;
use wgpu::wgt::TextureViewDescriptor;
use wgpu::{
    BindGroup, BindGroupLayout, Device, Extent3d, Origin3d, Queue, Sampler, TexelCopyBufferLayout,
    TexelCopyTextureInfo, Texture, TextureAspect, TextureDescriptor, TextureDimension,
    TextureUsages, TextureView,
};

pub struct Material<'a> {
    pub image: &'a RgbaImage,
    pub texture: Texture,
    pub texture_view: TextureView,
    pub bind_group: BindGroup,
}
impl<'a> Material<'a> {
    pub fn new(
        image: &'a RgbaImage,
        label: &str,
        bind_group_layout: &BindGroupLayout,
        device: &Device,
        queue: &Queue,
        sampler: &Sampler,
    ) -> Self {
        let width = image.width();
        let height = image.height();

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
            format: USED_PIXEL_FORMAT,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[USED_PIXEL_FORMAT],
        };
        let texture = device.create_texture(&texture_descriptor);

        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &image,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
            texture_size,
        );

        let texture_view = texture.create_view(&TextureViewDescriptor::default());

        let bind_group = BindGroupBuilder::new(device)
            .set_layout(bind_group_layout)
            .add_material(&texture_view, &sampler)
            .build(label);

        Self {
            //
            image,
            texture,
            texture_view,
            bind_group,
        }
    }
    pub fn width(&self) -> u32 {
        self.image.width()
    }
    pub fn height(&self) -> u32 {
        self.image.height()
    }
}
