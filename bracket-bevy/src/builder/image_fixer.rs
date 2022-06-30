use bevy::{prelude::*, render::texture::ImageSampler};

pub(crate) struct ImagesToLoad (pub(crate) Vec<HandleUntyped>);

pub(crate) fn fix_images(
    fonts: ResMut<ImagesToLoad>,
    mut events: EventReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,    
) {
    for event in events.iter() {
        //println!("{:?}", fonts.0);
        //println!("{:?}", event);
        match event {
            AssetEvent::Created { handle } => {
                if let Some(_font_handle) = fonts.0.iter().find(|h| **h == handle.clone_untyped()) {
                    // Try to fix the image scaling
                    if let Some(img) = images.get_mut(&handle) {
                        //println!("Acquired");
                        img.sampler_descriptor = ImageSampler::Descriptor(ImageSampler::linear_descriptor());
                    }
                }
            }
            _ => {}
        }
    }
}