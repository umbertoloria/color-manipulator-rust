use crate::gpu::gpu_context::GpuContext;
use crate::gpu::wgpu::USED_PIXEL_FORMAT;
use image::RgbaImage;
use wgpu::wgt::TextureViewDescriptor;
use wgpu::{
    BindGroup, BindGroupLayout, Extent3d, Origin3d, Sampler, TexelCopyBufferLayout,
    TexelCopyTextureInfo, Texture, TextureAspect, TextureDescriptor, TextureDimension,
    TextureUsages, TextureView,
};

pub struct Material<'a> {
    pub image: &'a RgbaImage,
    pub gpu_context: &'a GpuContext,
    pub texture: Texture,
    pub texture_view: TextureView,
    pub bind_group: BindGroup,
}
impl<'a> Material<'a> {
    pub fn new(
        image: &'a RgbaImage,
        label: &str,
        bind_group_layout: &BindGroupLayout,
        gpu_context: &'a GpuContext,
        sampler: &Sampler,
    ) -> Self {
        let width = image.width();
        let height = image.height();

        let texture = gpu_context.create_texture(&TextureDescriptor {
            label: Some(label),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: USED_PIXEL_FORMAT,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[USED_PIXEL_FORMAT],
        });
        let texture_view = texture.create_view(&TextureViewDescriptor::default());

        let bind_group = gpu_context
            .create_bind_group_builder()
            .set_layout(bind_group_layout)
            .add_material(&texture_view, &sampler)
            .build(label);

        let mut result = Self {
            image,
            gpu_context,
            texture,
            texture_view,
            bind_group,
        };
        result.write_image_to_texture();
        result
    }
    pub fn width(&self) -> u32 {
        self.image.width()
    }
    pub fn height(&self) -> u32 {
        self.image.height()
    }
    pub fn change_image(&mut self, new_image: &'a RgbaImage) -> Result<(), &'static str> {
        if self.image.width() != new_image.width() || self.image.height() != new_image.height() {
            return Err("New image has different size than the original one");
        }
        self.image = new_image;
        self.write_image_to_texture();
        Ok(())
    }
    fn write_image_to_texture(&mut self) {
        let image = self.image;
        let width = image.width();
        let height = image.height();
        self.gpu_context.write_texture(
            TexelCopyTextureInfo {
                texture: &self.texture,
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
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
    }
}
