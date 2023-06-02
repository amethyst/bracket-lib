use bevy::ecs::schedule::ScheduleLabel;
// Bevy 0.10, no command flush
use bevy::prelude::SystemSet;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet, ScheduleLabel)]
#[system_set(base)]
pub enum BracketTermDiagnostics {
    BeforeCoreFixedUpdate,
}

