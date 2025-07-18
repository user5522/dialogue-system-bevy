use bevy::prelude::*;

use crate::dialogue::DialogueLogEntry;

#[derive(Resource)]
pub struct DialogueState {
    pub active: bool,
    pub current_text: String,
    pub current_speaker: String,
}

#[derive(Resource, Default)]
pub struct DialogueLog {
    pub entries: Vec<DialogueLogEntry>,
    pub show_log: bool,
}
