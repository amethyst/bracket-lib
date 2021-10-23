use crate::{BResult, gamestate::BTerm};
use super::InitHints;

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    window_title: S,
    platform_hints: InitHints,
) -> BResult<BTerm> {
    Err("Not implemented yet".into())
}