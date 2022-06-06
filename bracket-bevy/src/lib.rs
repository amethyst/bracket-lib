// Provides Bracket-Lib style CP437/ASCII terminal options to Bevy
mod builder;
mod cp437;
mod fonts;
pub use builder::*;
mod context;
pub use context::*;
mod consoles;
use consoles::*;

pub mod prelude {
    pub use crate::{cp437::*, BTermBuilder, BracketContext};
}
