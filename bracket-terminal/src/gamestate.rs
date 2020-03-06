pub use crate::prelude::BTerm;

/// Implement this trait on your state struct, so the engine knows what to call on each tick.
pub trait GameState: 'static {
    fn tick(&mut self, ctx: &mut BTerm);
}
