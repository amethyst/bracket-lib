use bevy::ecs::schedule::ScheduleLabel;

use bevy::prelude::SystemSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet, ScheduleLabel)]
#[system_set(base)]
pub struct BracketTermUpdate;