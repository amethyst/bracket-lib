use crate::{BResult, gamestate::BTerm};
use super::{InitHints, BACKEND, WrappedContext};
use winit::{dpi::LogicalSize, event::*, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    window_title: S,
    platform_hints: InitHints,
) -> BResult<BTerm> {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title(window_title.to_string())
        .with_inner_size(LogicalSize::new(
            f64::from(width_pixels),
            f64::from(height_pixels),
        ));
    let window = wb.build(&el).unwrap();
    /*let windowed_context = ContextBuilder::new()
        .with_gl(platform_hints.gl_version)
        .with_gl_profile(platform_hints.gl_profile)
        .with_hardware_acceleration(Some(true))
        .with_vsync(platform_hints.vsync)
        .with_srgb(platform_hints.srgb)
        .build_windowed(wb, &el)?;
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };*/

    /*if platform_hints.fullscreen {
        if let Some(mh) = el.available_monitors().next() {
            windowed_context
                .window()
                .set_fullscreen(Some(winit::window::Fullscreen::Borderless(Some(mh))));
        } else {
            return Err("No available monitor found".into());
        }
    }*/

    // Bunch more goes here

    // Store the backend
    let mut be = BACKEND.lock();
    be.context_wrapper = Some(WrappedContext {
        el,
        window,
    });

    let bterm = BTerm {
        width_pixels,
        height_pixels,
        original_width_pixels: width_pixels,
        original_height_pixels: height_pixels,
        fps: 0.0,
        frame_time_ms: 0.0,
        active_console: 0,
        key: None,
        mouse_pos: (0, 0),
        left_click: false,
        shift: false,
        control: false,
        alt: false,
        web_button: None,
        quitting: false,
        post_scanlines: false,
        post_screenburn: false,
        screen_burn_color: bracket_color::prelude::RGB::from_f32(0.0, 1.0, 1.0),
    };
    Ok(bterm)
}