use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource, Device,
    Sampler, TextureView,
};

pub struct BindGroupBuilder<'a> {
    entries: Vec<BindGroupEntry<'a>>,
    layout: Option<&'a BindGroupLayout>,
    device: &'a Device,
}
impl<'a> BindGroupBuilder<'a> {
    pub fn new(device: &'a Device) -> Self {
        Self {
            entries: Vec::new(),
            layout: None,
            device,
        }
    }

    fn reset(&mut self) {
        self.entries.clear();
    }

    pub fn set_layout(&mut self, layout: &'a BindGroupLayout) -> &mut Self {
        self.layout = Some(layout);
        self
    }

    pub fn add_material(&mut self, view: &'a TextureView, sampler: &'a Sampler) -> &mut Self {
        self.entries.push(BindGroupEntry {
            binding: self.entries.len() as u32,
            resource: BindingResource::TextureView(view),
        });
        self.entries.push(BindGroupEntry {
            binding: self.entries.len() as u32,
            resource: BindingResource::Sampler(sampler),
        });
        self
    }

    pub fn build(&mut self, label: &str) -> BindGroup {
        let bing_group_descriptor = BindGroupDescriptor {
            label: Some(label),
            layout: self.layout.unwrap(),
            entries: &self.entries,
        };
        let bind_group = self.device.create_bind_group(&bing_group_descriptor);
        self.reset();
        bind_group
    }
}
