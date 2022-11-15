use bevy::{
    prelude::*,
    render::{render_resource::SamplerDescriptor, texture::ImageSampler},
};

#[derive(Resource)]
pub(crate) struct ImagesToLoad(pub(crate) Vec<HandleUntyped>);

pub(crate) fn fix_images(mut fonts: ResMut<ImagesToLoad>, mut images: ResMut<Assets<Image>>) {
    if fonts.0.is_empty() {
        return;
    }

    for (handle, img) in images.iter_mut() {
        let mut to_remove = Vec::new();
        if let Some(i) = fonts.0.iter().enumerate().find(|(_i, h)| h.id == handle) {
            img.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
                label: Some("LeaveItAlone"),
                address_mode_u: bevy::render::render_resource::AddressMode::ClampToEdge,
                address_mode_v: bevy::render::render_resource::AddressMode::ClampToEdge,
                address_mode_w: bevy::render::render_resource::AddressMode::ClampToEdge,
                mag_filter: bevy::render::render_resource::FilterMode::Nearest,
                min_filter: bevy::render::render_resource::FilterMode::Nearest,
                mipmap_filter: bevy::render::render_resource::FilterMode::Nearest,
                ..Default::default()
            });
            to_remove.push(i.0);
        }
        to_remove.iter().for_each(|i| {
            fonts.0.remove(*i);
        });
    }
}
