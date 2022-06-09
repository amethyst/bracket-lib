// Provides Bracket-Lib style CP437/ASCII terminal options to Bevy
mod builder;
mod cp437;
mod fonts;
pub use builder::*;
mod context;
pub use context::*;
mod consoles;
use consoles::*;
mod random_resource;
pub use random_resource::*;

pub mod prelude {
    pub use crate::{consoles::TextAlign, cp437::*, BTermBuilder, BracketContext, RandomNumbers, TerminalScalingMode};
    pub use bracket_color::prelude::*;
}
